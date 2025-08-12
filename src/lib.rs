//! ARM System Memory Management Unit (SMMU) v3 driver written in Rust.

#![no_std]

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

use queue::{Cmd, Queue};
use stream_table::{LinearStreamTable, StreamTableEntry};

register_structs! {
    /// Chapter 6. Memory map and registers 6.2.
    /// SMMU registers occupy two consecutive 64KB pages starting from an at least 64KB-aligned boundary.
    /// The following registers are accessible from the SMMU page 0 and page 1 region.
    /// - 0x00000-0x0FFFF SMMU registers, Page 0
    /// - 0x10000-0x1FFFF SMMU registers, Page 1
    #[allow(non_snake_case)]
    pub SMMUv3Regs  {
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
        (0x002c => CR2: Cr2Reg),
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
        (0x00a0 => EVENTQ_BASE: EventQBaseReg),
        (0x00a8 => _reserved4),
        (0x00b0 => EVENTQ_IRQ_CFG0: ReadWrite<u64>),
        (0x00b8 => EVENTQ_IRQ_CFG1: ReadWrite<u32>),
        (0x00bc => EVENTQ_IRQ_CFG2: ReadWrite<u32>),
        (0x00c0 => _reserved5),
        (0x100a8 => EVENTQ_PROD: EventQProdReg),
        (0x100ac => EVENTQ_CONS: EventQConsReg),
        (0x100b0 => _reserved6),
        (0x20000 => @END),
    }
}

/// SMMUv3 driver with a linear stream table and cmd queue.
pub struct SMMUv3<H: PagingHandler> {
    base: NonNull<SMMUv3Regs>,
    stream_table: LinearStreamTable<H>,
    cmd_queue: Queue<H>,
    event_queue: Queue<H>,
}

unsafe impl<H: PagingHandler> Send for SMMUv3<H> {}
unsafe impl<H: PagingHandler> Sync for SMMUv3<H> {}

const ARM_SMMU_SYNC_TIMEOUT: usize = 0x1000000;

impl<H: PagingHandler> SMMUv3<H> {
    /// Construct a new SMMUv3 instance from the base address.
    pub const fn new(base: *mut u8) -> Self {
        Self {
            base: NonNull::new(base).unwrap().cast(),
            stream_table: LinearStreamTable::uninit(),
            cmd_queue: Queue::uninit(),
            event_queue: Queue::uninit(),
        }
    }

    /// Initialize the SMMUv3 instance.
    pub fn init(&mut self) {
        let sid_max_bits = self.regs().IDR1.read(IDR1::SIDSIZE);
        info!("Max SID bits: {}, max SIE count {}", sid_max_bits, 1 << sid_max_bits);

        if sid_max_bits >= 7
            && self.regs().IDR0.read(IDR0::ST_LEVEL) == IDR0::ST_LEVEL::LinearStreamTable.into()
        {
            // SMMU supports one stream
            panic!("Smmuv3 the system must support for 2-level table");
        }

        let cmdqs_log2 = H::CMDQ_EVENTQ_BITS_SET;
        self.cmd_queue.init(cmdqs_log2);
        self.regs().CMDQ_BASE.write(
            CMDQ_BASE::RA::ReadAllocate
                + CMDQ_BASE::ADDR.val(self.cmd_queue.base_addr().as_usize() as u64 >> 5)
                + CMDQ_BASE::LOG2SIZE.val(cmdqs_log2 as _),
        );

        self.regs()
            .CMDQ_PROD
            .write(CMDQ_PROD::WR.val(self.cmd_queue.prod_value()));
        self.regs()
            .CMDQ_CONS
            .write(CMDQ_CONS::RD.val(self.cmd_queue.cons_value()));

        self.regs()
            .CR0
            .write(CR0::CMDQEN::Enable);

        self.stream_table_init();

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

        self.regs().CR2.write(CR2::VALID::defaul);
        self.regs()
            .CR0
            .write(CR0::SMMUEN::Enable + CR0::CMDQEN::Enable);

        for _timeout in 0..ARM_SMMU_SYNC_TIMEOUT {
            if self.regs().CR0ACK.is_set(CR0ACK::SMMUEN)
                && self.regs().CR0ACK.is_set(CR0ACK::CMDQEN)
            {
                info!("SMMUv3 enabled");
                return;
            }
        }
        error!("SMMUv3 enabled timeout");
    }

    pub fn stream_table_init(&mut self) {
        self.stream_table.init(H::SID_BITS_SET);
        for sid in 0..self.stream_table.entry_count() {
            self.stream_table.set_bypass_ste(sid);
        }
        self.regs().STRTAB_BASE_CFG.write(
            STRTAB_BASE_CFG::FMT::Linear + STRTAB_BASE_CFG::LOG2SIZE.val(H::SID_BITS_SET),
        );
        self.regs().STRTAB_BASE.write(
            STRTAB_BASE::RA::Enable
                + STRTAB_BASE::ADDR.val(self.stream_table.base_addr().as_usize() as u64 >> 6),
        );
    }

    /// Get the SMMUv3 registers.
    pub const fn regs(&self) -> &SMMUv3Regs {
        unsafe { self.base.as_ref() }
    }

    /// Get the SMMUv3 version.
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

    /// Add a command to the command queue.
    pub fn add_cmd(&mut self, cmd: Cmd, sync: bool) {
        while self.cmd_queue.full() {
            warn!("Command queue is full, try consuming");
            let cmdq_cons = self.regs().CMDQ_CONS.get();
            if cmdq_cons & (CMDQ_CONS::ERR.mask << CMDQ_CONS::ERR.shift) != 0 {
                warn!(
                    "CMDQ_CONS ERR code {}",
                    (cmdq_cons & (CMDQ_CONS::ERR.mask << CMDQ_CONS::ERR.shift)) >> CMDQ_CONS::ERR.shift
                );
            }

            let cons_value = cmdq_cons & (CMDQ_CONS::RD.mask << CMDQ_CONS::RD.shift);
            self.cmd_queue.set_cons_value(cons_value);
        }

        self.cmd_queue.cmd_insert(cmd.clone());

        self.regs()
            .CMDQ_PROD
            .write(CMDQ_PROD::WR.val(self.cmd_queue.prod_value()));

        while !self.cmd_queue.empty() {
            trace!("Command queue is not empty, consuming");
            let cmdq_cons = self.regs().CMDQ_CONS.get();
            if cmdq_cons & (CMDQ_CONS::ERR.mask << CMDQ_CONS::ERR.shift) != 0 {
                warn!(
                    "CMDQ_CONS ERR code {}",
                    (cmdq_cons & (CMDQ_CONS::ERR.mask << CMDQ_CONS::ERR.shift)) >> CMDQ_CONS::ERR.shift
                );
            }
            let cons_value = cmdq_cons & (CMDQ_CONS::RD.mask << CMDQ_CONS::RD.shift);
            self.cmd_queue.set_cons_value(cons_value);
        }

        if sync {
            self.add_cmd(Cmd::cmd_sync(), false);
        }
    }

    /// Add a passthrough device, updating the stream table.
    pub fn add_device(&mut self, sid: usize, vmid: usize, s2pt_base: PhysAddr) {
        let cmd = Cmd::cmd_cfgi_ste(sid as u32);

        self.stream_table
            .set_s2_translated_ste(sid, vmid, s2pt_base);

        self.add_cmd(cmd, true);
    }
}
