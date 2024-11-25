//! Chapter 6. Memory map and registers
//! 6.3. Register formats
//! 6.3.26 SMMU_CMDQ_BASE
//!
//! The SMMU_CMDQ_BASE characteristics are:
//!
//! ## Purpose
//! Configuration of the Command queue base address.
//!
//! ## Attributes
//! SMMU_CMDQ_BASE is a 64-bit register.
//!
//! This register is part of the SMMUv3_PAGE_0 block.
//!

use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u64,
    pub CMDQ_BASE [
        /// Bit [63] Reserved, RES0.
        Reserved31 OFFSET(63) NUMBITS(1) [],
        /// RA, bit [62] Read-Allocate hint.
        ///
        /// - 0b0 No Read-Allocate.
        /// - 0b1 Read-Allocate.
        ///
        /// The reset behavior of this field is:
        /// - When SMMU_IDR1.QUEUES_PRESET == 1, this field resets to an IMPLEMENTATION DEFINED value.
        /// - Otherwise, this field resets to an UNKNOWN value.
        RA OFFSET(62) NUMBITS(1) [
            NoReadAllocate = 0,
            ReadAllocate = 1
        ],
        /// Bits [61:56] Reserved, RES0.
        Reserved56 OFFSET(56) NUMBITS(6) [],
        /// ADDR, bits [55:5] PA of Command queue base, bits [55:5].
        /// - Address bits above and below this field range are treated as zero.
        /// - High-order bits of the ADDR field above the system physical address size, as reported by SMMU_IDR5.OAS, are RES0.
        ///     – Note: An implementation is not required to store these bits.
        /// - The effective base address is aligned by the SMMU to the larger of the queue size in bytes or 32 bytes, ignoring the least-significant bits of ADDR as required. ADDR bits [4:0] are treated as zero.
        ///     – Note: For example, a queue with 28 entries is 4096 bytes in size so software must align an allocation, and therefore ADDR, to a 4KB boundary.
        ///
        /// The reset behavior of this field is:
        /// - When SMMU_IDR1.QUEUES_PRESET == 1, this field resets to an IMPLEMENTATION DEFINED value.
        /// - Otherwise, this field resets to an UNKNOWN value.
        ADDR OFFSET(5) NUMBITS(51) [],
        /// LOG2SIZE, bits [4:0] Queue size as log2(entries).
        /// - LOG2SIZE must be less than or equal to SMMU_IDR1.CMDQS. Except for the purposes of readback of this register, any use of the value of this field is capped at the maximum, SMMU_IDR1.CMDQS.
        /// - The minimum size is 0, for one entry, but this must be aligned to a 32-byte (2 entry) boundary as above.
        ///
        /// The reset behavior of this field is:
        /// - When SMMU_IDR1.QUEUES_PRESET == 1, this field resets to an IMPLEMENTATION DEFINED value.
        /// - Otherwise, this field resets to an UNKNOWN value.
        LOG2SIZE OFFSET(0) NUMBITS(5) []
    ]
}

/// Upon initialization, if SMMU_IDR1.QUEUES_PRESET == 0 then the SMMU_CMDQ_BASE.LOG2SIZE field might affect which bits of SMMU_CMDQ_CONS.RD and SMMU_CMDQ_PROD.WR can be written upon initialization. The registers must be initialized in this order:
///     1. Write SMMU_CMDQ_BASE to set the queue base and size.
///     2. Write initial values to SMMU_CMDQ_CONS and SMMU_CMDQ_PROD.
///     3. Enable the queue with an Update of the respective SMMU_CR0.CMDQEN to 1.
///
/// This also applies to the initialization of Event queue and PRI queue registers.
/// Access attributes of the Command queue are set using the SMMU_CR1.QUEUE_* fields. A Read-Allocate hint is provided for Command queue accesses with the RA field.
///
/// SMMU_CMDQ_BASE is Guarded by SMMU_CR0.CMDQEN and must only be modified when SMMU_CR0.CMDQEN == 0
pub type CmdQBaseReg = ReadWrite<u64, CMDQ_BASE::Register>;
