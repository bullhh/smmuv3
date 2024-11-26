use memory_addr::{PhysAddr, VirtAddr};

/// The low-level **OS-dependent** helpers that must be provided for
/// [`crate::SMMUv3`].
pub trait PagingHandler: Sized {
    /// Request to allocate contiguous 4K-sized pages.
    fn alloc_pages(num_pages: usize) -> Option<PhysAddr>;
    /// Request to free allocated physical pages.
    fn dealloc_pages(paddr: PhysAddr, num_pages: usize);
    /// Returns a virtual address that maps to the given physical address.
    ///
    /// Used to access the physical memory directly in page table implementation.
    fn phys_to_virt(paddr: PhysAddr) -> VirtAddr;
}
