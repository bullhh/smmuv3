use core::mem::size_of;

use memory_addr::{align_up_4k, va, VirtAddr, PAGE_SIZE_4K};

use crate::hal::PagingHandler;

/// According to the SMMUv3 spec, Chapter 3. Operation 3.5. Command and Event queues.
///
/// Each circular buffer is 2^n-items in size, where 0 <= n <= 19.
/// An implementation might support fewer than 19 bits of index.
/// Each PROD and CONS register is 20 bits to accommodate the maximum 19-bit index plus the wrap bit.
pub const MAX_CMD_EVENT_QS: u32 = 19;

/// Chapter 4.
/// Commands 4.1. Commands overview
/// 4.1 Commands overview
/// 4.1.1 Command opcodes
const CMD_CFGI_STE: u64 = 0x03;
const CMD_SYNC: u64 = 0x46;

const CMDQ_ENT_DWORDS: usize = 2;

#[derive(Default)]
#[repr(C)]
pub struct Cmd([u64; CMDQ_ENT_DWORDS]);

impl Cmd {
    /// 4.3.1 CMD_CFGI_STE(StreamID, SSec, Leaf)
    ///
    /// Invalidate the STE indicated by StreamID and SSec.
    pub fn cmd_cfgi_ste(stream_id: u32) -> Self {
        const CMD_CFGI_STE_SID_OFFSET: u64 = 32;
        const CMDQ_CFGI_1_LEAF: u64 = 1;

        let mut cmd = Self::default();
        cmd.0[0] |= CMD_CFGI_STE;
        cmd.0[0] |= (stream_id as u64) << CMD_CFGI_STE_SID_OFFSET;
        // Leaf == 1
        cmd.0[1] |= CMDQ_CFGI_1_LEAF;
        info!("CMD: 0x{:x}, 0x{:x}", cmd.0[0], cmd.0[1]);
        cmd
    }

    /// 4.7.3 CMD_SYNC(ComplSignal, MSIAddress, MSIData, MSIWriteAttributes)
    ///
    /// This command provides a synchronization mechanism for the following:
    /// - Preceding commands that were issued to the same Command queue as the CMD_SYNC.
    /// - Visibility of event records for client transactions terminated before the CMD_SYNC.
    /// - HTTU updates caused by completed translations.
    pub fn cmd_sync() -> Self {
        let mut cmd = Self::default();
        cmd.0[0] |= CMD_SYNC;
        cmd
    }
}

/// 3.5 Command and Event queues
pub struct Queue<H: PagingHandler> {
    base: VirtAddr,
    queue_size: u32,
    qs: u32,//log2(queue_size),
    prod: u32,
    cons: u32,
    _marker: core::marker::PhantomData<H>,
}

impl<H: PagingHandler> Queue<H> {
    pub const fn uninit() -> Self {
        Self {
            base: va!(0xdead_beef),
            queue_size: 0,
            qs: 0,
            prod: 0,
            cons: 0,
            _marker: core::marker::PhantomData,
        }
    }

    pub fn init(&mut self, qs: u32) {
        assert_eq!(size_of::<Cmd>(), CMDQ_ENT_DWORDS << 3);

        let qs = u32::min(qs, MAX_CMD_EVENT_QS);
        self.qs = qs;
        self.queue_size = 1 << qs;

        let num_pages = align_up_4k(self.queue_size as usize * size_of::<Cmd>()) / PAGE_SIZE_4K;
        self.base = H::phys_to_virt(H::alloc_pages(num_pages).expect("Failed to allocate queue"));
    }

    pub fn base_addr(&self) -> VirtAddr {
        self.base
    }

    pub fn prod_value(&self) -> u32 {
        self.prod
    }

    pub fn cons_value(&self) -> u32 {
        self.cons
    }

    pub fn set_cons_value(&mut self, cons: u32) {
        if cons & !((1 << self.qs) - 1) != 0 {
            warn!("Invalid cons value {}", cons);
        }
        self.cons = cons;
    }

    fn prod_wr_wrap(&self) -> bool {
        self.prod & (1 << self.qs) != 0
    }

    fn cons_rd_wrap(&self) -> bool {
        self.cons & (1 << self.qs) != 0
    }

    fn prod_wr(&self) -> u32 {
        self.prod & (self.queue_size - 1)
    }

    fn cons_rd(&self) -> u32 {
        self.cons & (self.queue_size - 1)
    }

    fn inc_proc_wq(&mut self) {
        let mut current_proc_wq = self.prod_wr();
        let mut current_proc_wrap = self.prod_wr_wrap();
        current_proc_wq += 1;

        // Check overflow, update wrap bit.
        if (current_proc_wq & (self.queue_size - 1)) == 0 {
            current_proc_wq %= self.queue_size;
            current_proc_wrap = !current_proc_wrap;
        }

        assert!(current_proc_wq & !((1 << self.qs) - 1) == 0);

        let current_proc_wrap_bit = if current_proc_wrap {
            1 << self.qs
        } else {
            0
        };

        self.prod = current_proc_wrap_bit | current_proc_wq;
    }

    pub fn full(&self) -> bool {
        // PROD.WR == CONS.RD and PROD.WR_WRAP != CONS.RD_WRAP,
        // representing a full queue.
        self.prod_wr() == self.cons_rd() && self.prod_wr_wrap() != self.cons_rd_wrap()
    }

    pub fn empty(&self) -> bool {
        // PROD.WR == CONS.RD and PROD.WR_WRAP == CONS.RD_WRAP,
        // representing an empty queue.
        self.prod_wr() == self.cons_rd() && self.prod_wr_wrap() == self.cons_rd_wrap()
    }

    pub fn cmd_insert(&mut self, cmd: Cmd) {
        let idx = self.prod_wr() as usize;
        let base = self.base.as_mut_ptr() as *mut Cmd;
        unsafe {
            base.add(idx).write(cmd);
        }
        self.inc_proc_wq();
    }
}

#[cfg(test)]
mod test {
    use memory_addr::{pa, va, PhysAddr, VirtAddr, PAGE_SIZE_4K};

    use crate::queue::Queue;

    static mut DUMMY_PAGE: [u8; PAGE_SIZE_4K] = [0; PAGE_SIZE_4K];

    struct DummyPagingHandler {}

    impl crate::hal::PagingHandler for DummyPagingHandler {
        fn alloc_pages(pages: usize) -> Option<PhysAddr> {
            assert!(pages == 1);
            Some(pa!(unsafe { DUMMY_PAGE.as_mut_ptr() } as usize))
        }

        fn phys_to_virt(addr: PhysAddr) -> VirtAddr {
            va!(addr.as_usize())
        }

        fn dealloc_pages(paddr: PhysAddr, _num_pages: usize) {
            assert!(paddr == pa!(unsafe { DUMMY_PAGE.as_mut_ptr() } as usize));
        }
    }

    #[test]
    fn test_queue() {
        let mut queue = Queue::<DummyPagingHandler>::uninit();
        queue.init(7);

        assert_eq!(
            queue.base_addr(),
            va!(unsafe { DUMMY_PAGE.as_mut_ptr() } as usize)
        );
        assert_eq!(queue.prod_value(), 0);
        assert_eq!(queue.cons_value(), 0);
        assert_eq!(queue.prod_wr(), 0);
        assert_eq!(queue.prod_wr_wrap(), false);
        assert_eq!(queue.cons_rd(), 0);
        assert_eq!(queue.cons_rd_wrap(), false);

        assert_eq!(queue.full(), false);
        assert_eq!(queue.empty(), true);

        for i in 0..64 {
            queue.cmd_insert(crate::queue::Cmd::cmd_cfgi_ste(i));
        }

        assert_eq!(queue.full(), false);
        assert_eq!(queue.empty(), false);
        assert_eq!(queue.prod_wr(), 64);
        assert_eq!(queue.prod_wr_wrap(), false);
        assert_eq!(queue.cons_rd(), 0);
        assert_eq!(queue.cons_rd_wrap(), false);

        for i in 64..128 {
            queue.cmd_insert(crate::queue::Cmd::cmd_cfgi_ste(i));
        }

        assert_eq!(queue.full(), true);
        assert_eq!(queue.empty(), false);
        assert_eq!(queue.prod_wr(), 0);
        assert_eq!(queue.prod_wr_wrap(), true);
        assert_eq!(queue.cons_rd(), 0);
        assert_eq!(queue.cons_rd_wrap(), false);
    }
}
