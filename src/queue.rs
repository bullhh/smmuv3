use memory_addr::{pa, PhysAddr};

use crate::hal::PagingHandler;

/// According to the SMMUv3 spec, Chapter 3. Operation 3.5. Command and Event queues.
///
/// Each circular buffer is 2^n-items in size, where 0 <= n <= 19.
/// An implementation might support fewer than 19 bits of index.
/// Each PROD and CONS register is 20 bits to accommodate the maximum 19-bit index plus the wrap bit.
pub const MAX_CMDQS: u32 = 19;

pub struct Queue<H: PagingHandler> {
    base: PhysAddr,
    queue_size: u32,
    prod: u32,
    cons: u32,
    _marker: core::marker::PhantomData<H>,
}

impl<H: PagingHandler> Queue<H> {
    pub const fn uninit() -> Self {
        Self {
            base: pa!(0xdead_beef),
            queue_size: 0,
            prod: 0,
            cons: 0,
            _marker: core::marker::PhantomData,
        }
    }

    pub fn init(&mut self, cmdqs: u32) {
        self.base = H::alloc_pages(1).expect("Failed to allocate queue");

        let cmdqs = u32::max(cmdqs, MAX_CMDQS);
        self.queue_size = 1 << cmdqs;
    }

    pub fn base_addr(&self) -> PhysAddr {
        self.base
    }

    pub fn prod(&self) -> u32 {
        self.prod
    }

    pub fn cons(&self) -> u32 {
        self.cons
    }
}
