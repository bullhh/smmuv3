//! ARM System Memory Management Unit (SMMU) v3 driver written in Rust.

#![no_std]
#![feature(const_option)]
#![feature(const_nonnull_new)]

#[macro_use]
extern crate log;

use core::panic;
use core::ptr::NonNull;

use memory_addr::PhysAddr;
use tock_registers::interfaces::{Readable, Writeable};
use tock_registers::register_structs;
use tock_registers::registers::{ReadOnly, ReadWrite};

mod hal;
mod queue;
mod regs;
mod stream_table;

pub use hal::PagingHandler;
pub use regs::*;

use queue::Queue;
use stream_table::LinearStreamTable;

register_structs! {
    #[allow(non_snake_case)]
    pub SMMUv3Page0Regs  {
        (0x0000 => IDR0: IDR0Reg),
        (0x0004 => IDR1: IDR1Reg),
        (0x0008 => IDR2: ReadOnly<u32>),
        (0x000C => IDR3: ReadOnly<u32>),
        (0x0010 => IDR4: ReadOnly<u32>),
        (0x0014 => IDR5: ReadOnly<u32>),
        (0x0018 => IIDR: ReadOnly<u32>),
        (0x001C => AIDR: AIDRReg),
        (0x0020 => CR0: Cr0Reg),
        (0x0024 => CR0ACK: Cr0AckReg),
        (0x0028 => CR1: Cr1Reg),
        (0x002c => CR2: ReadWrite<u32>),
        (0x0030 => _reserved0),
        (0x0050 => IRQ_CTRL: ReadWrite<u32>),
        (0x0054 => IRQ_CTRLACK: ReadOnly<u32>),
        (0x0058 => _reserved1),
        (0x0060 => GERROR: ReadOnly<u32>),
        (0x0064 => GERRORN: ReadWrite<u32>),
        (0x0068 => GERROR_IRQ_CFG0: ReadWrite<u64>),
        (0x0070 => _reserved2),
        (0x0080 => STRTAB_BASE: StrtabBaseReg),
        (0x0088 => STRTAB_BASE_CFG: StrtabBaseCfgReg),
        (0x008c => _reserved3),
        (0x0090 => CMDQ_BASE: CmdQBaseReg),
        (0x0098 => CMDQ_PROD: CmdQProdReg),
        (0x009c => CMDQ_CONS: CmdQConsReg),
        (0x00a0 => EVENTQ_BASE: ReadWrite<u64>),
        (0x00a8 => _reserved4),
        (0x00b0 => EVENTQ_IRQ_CFG0: ReadWrite<u64>),
        (0x00b8 => EVENTQ_IRQ_CFG1: ReadWrite<u32>),
        (0x00bc => EVENTQ_IRQ_CFG2: ReadWrite<u32>),
        (0x00c0 => _reserved5),
        (0x100a8 => EVENTQ_PROD: ReadWrite<u32>),
        (0x100ac => EVENTQ_CONS: ReadWrite<u32>),
        (0x100b0 => _reserved6),
        (0x20000 => @END),
    }
}

pub struct SMMUv3<H: PagingHandler> {
    base: NonNull<SMMUv3Page0Regs>,
    stream_table: LinearStreamTable<H>,
    cmd_queue: Queue<H>,
}

unsafe impl<H: PagingHandler> Send for SMMUv3<H> {}
unsafe impl<H: PagingHandler> Sync for SMMUv3<H> {}

impl<H: PagingHandler> SMMUv3<H> {
    /// Construct a new SMMUv3 instance from the base address.
    pub const fn new(base: *mut u8) -> Self {
        Self {
            base: NonNull::new(base).unwrap().cast(),
            stream_table: LinearStreamTable::uninit(),
            cmd_queue: Queue::uninit(),
        }
    }

    pub fn init(&mut self) {
        let sid_max_bits = self.regs().IDR1.read(IDR1::SIDSIZE);
        info!(
            "Max SID bits: {}, max SIE count {}",
            sid_max_bits,
            1 << sid_max_bits
        );

        if sid_max_bits >= 7
            && self.regs().IDR0.read(IDR0::ST_LEVEL) == IDR0::ST_LEVEL::LinearStreamTable.into()
        {
            // SMMU supports one stream
            panic!("Smmuv3 the system must support for 2-level table");
        }

        self.stream_table.init(sid_max_bits);

        self.regs().STRTAB_BASE.write(
            STRTAB_BASE::RA::Enable
                + STRTAB_BASE::ADDR.val(self.stream_table.base_addr().as_usize() as u64 >> 6),
        );

        self.regs()
            .STRTAB_BASE_CFG
            .write(STRTAB_BASE_CFG::FMT::Linear + STRTAB_BASE_CFG::LOG2SIZE.val(sid_max_bits));

        let cmdqs_log2 = self.regs().IDR1.read(IDR1::CMDQS);
        self.cmd_queue.init(cmdqs_log2);
        self.regs().CMDQ_BASE.write(
            CMDQ_BASE::RA::ReadAllocate
                + CMDQ_BASE::ADDR.val(self.cmd_queue.base_addr().as_usize() as u64 >> 5)
                + CMDQ_BASE::LOG2SIZE.val(cmdqs_log2 as _),
        );
        self.regs()
            .CMDQ_PROD
            .write(CMDQ_PROD::WR.val(self.cmd_queue.prod()));
        self.regs()
            .CMDQ_CONS
            .write(CMDQ_CONS::RD.val(self.cmd_queue.cons()));

        self.enable();
    }

    fn enable(&mut self) {
        self.regs().CR1.write(
            CR1::TABLE_IC::WriteBackCacheable
                + CR1::TABLE_OC::WriteBackCacheable
                + CR1::TABLE_SH::InnerShareable
                + CR1::QUEUE_IC::WriteBackCacheable
                + CR1::QUEUE_OC::WriteBackCacheable
                + CR1::QUEUE_SH::InnerShareable,
        );

        self.regs().CR0.write(CR0::SMMUEN::Enable);

        const ARM_SMMU_SYNC_TIMEOUT: usize = 0x1000000;

        for _timeout in 0..ARM_SMMU_SYNC_TIMEOUT {
            if self.regs().CR0ACK.is_set(CR0ACK::SMMUEN) {
                info!("SMMUv3 enabled");
                return;
            }
        }
        error!("CR0 write err!");
    }

    const fn regs(&self) -> &SMMUv3Page0Regs {
        unsafe { self.base.as_ref() }
    }

    pub fn version(&self) -> &'static str {
        match self.regs().AIDR.read_as_enum(AIDR::ArchMinorRev) {
            Some(AIDR::ArchMinorRev::Value::SMMUv3_0) => "SMMUv3.0",
            Some(AIDR::ArchMinorRev::Value::SMMUv3_1) => "SMMUv3.1",
            Some(AIDR::ArchMinorRev::Value::SMMUv3_2) => "SMMUv3.2",
            Some(AIDR::ArchMinorRev::Value::SMMUv3_3) => "SMMUv3.3",
            Some(AIDR::ArchMinorRev::Value::SMMUv3_4) => "SMMUv3.4",
            _ => "Unknown",
        }
    }

    pub fn aidr(&self) -> &AIDRReg {
        &self.regs().AIDR
    }

    pub fn check_features(&self) {
        self.regs().IDR0.is_set(IDR0::S1P);
    }

    pub fn add_device(&mut self, sid: usize, vmid: usize, s2pt_base: PhysAddr) {
        self.stream_table
            .set_s2_translated_ste(sid, vmid, s2pt_base);
    }
}
