use memory_addr::{PhysAddr, VirtAddr};

/// The low-level **OS-dependent** helpers that must be provided for
/// [`crate::SMMUv3`].
pub trait PagingHandler: Sized {
    /// 6.3.24 SMMU_STRTAB_BASE
    /// • When a Linear Stream table is used, that is when SMMU_STRTAB_BASE_CFG.FMT == 0b00, the
    /// effective base address is aligned by the SMMU to the table size, ignoring the least-significant bits in the
    /// ADDR range as required to do so:
    /// ADDR[LOG2SIZE + 5:0] = 0.
    /// • When a 2-level Stream table is used, that is when SMMU_STRTAB_BASE_CFG.FMT == 0b01, the
    /// effective base address is aligned by the SMMU to the larger of 64 bytes or the first-level table size:
    /// ADDR[MAX(5, (LOG2SIZE - SPLIT - 1 + 3)):0] = 0.
    /// The alignment of ADDR is affected by the literal value of the respective
    /// SMMU_STRTAB_BASE_CFG.LOG2SIZE field and is not limited by SIDSIZE.
    /// Note: This means that configuring a table that is larger than required by the incoming StreamID span results
    /// in some entries being unreachable, but the table is still aligned to the configured size.
    /// For example, SID_BITS_SET = 16, when alloc page alignment is to 2^(16 + 6) = 2^22 = 4MB.
    const SID_BITS_SET: u32 ;

    /// 6.3.26 SMMU_CMDQ_BASE
    /// • The effective base address is aligned by the SMMU to the larger of the queue size in bytes or 32 bytes,
    /// ignoring the least-significant bits of ADDR as required. ADDR bits [4:0] are treated as zero.
    /// – Note: For example, a queue with 2^8 entries is 4096 bytes in size so software must align an allocation,
    /// and therefore ADDR, to a 4KB boundary
    /// 2^8*16=4096 bytes.this means 256 entries, 16 bytes per entry.
    const CMDQ_EVENTQ_BITS_SET: u32;
    
    /// Request to allocate contiguous 4K-sized pages.
    fn alloc_pages(num_pages: usize) -> Option<PhysAddr>;
    /// Request to free allocated physical pages.
    fn dealloc_pages(paddr: PhysAddr, num_pages: usize);
    /// Returns a virtual address that maps to the given physical address.
    ///
    /// Used to access the physical memory directly in page table implementation.
    fn phys_to_virt(paddr: PhysAddr) -> VirtAddr;
    ///flush the memory range [start, start+len)
    fn flush(start: usize, len: usize);
}
