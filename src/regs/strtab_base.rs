//! Chapter 6. Memory map and registers
//! 6.3. Register formats
//! 6.3.24 SMMU_STRTAB_BASE
//!
//! The SMMU_STRTAB_BASE characteristics are:
//!
//! ## Purpose
//!     Configuration of Stream table base address.
//!
//! ## Attributes
//!     SMMU_STRTAB_BASE is a 64-bit register.
//!     This register is part of the SMMUv3_PAGE_0 block.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u64,
    pub STRTAB_BASE [
        /// Bit [63] Reserved, RES0.
        Reserved63 OFFSET(63) NUMBITS(1) [],
        /// Read-Allocate hint.
        ///
        /// - 0b0 No Read-Allocate.
        /// - 0b1 Read-Allocate.
        RA OFFSET(62) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],
        /// Bits [61:56] Reserved, RES0.
        Reserved56 OFFSET(56) NUMBITS(6) [],
        /// Physical address of Stream table base, bits [55:6].
        ///
        /// - Address bits above and below this field range are implied as zero.
        /// - High-order bits of the ADDR field above the system physical address size, as reported by SMMU_IDR5.OAS, are RES0.
        ///     - Note: An implementation is not required to store these bits.
        /// • When a Linear Stream table is used, that is when SMMU_STRTAB_BASE_CFG.FMT == 0b00, the effective base address is aligned by the SMMU to the table size, ignoring the least-significant bits in the ADDR range as required to do so:
        ///     ADDR[LOG2SIZE + 5:0] = 0.
        /// When a 2-level Stream table is used, that is when SMMU_STRTAB_BASE_CFG.FMT == 0b01, the effective base address is aligned by the SMMU to the larger of 64 bytes or the first-level table size:
        ///     ADDR[MAX(5, (LOG2SIZE - SPLIT - 1 + 3)):0] = 0.
        ///
        /// The alignment of ADDR is affected by the literal value of the respective SMMU_STRTAB_BASE_CFG.LOG2SIZE field and is not limited by SIDSIZE.
        /// Note: This means that configuring a table that is larger than required by the incoming StreamID span results in some entries being unreachable, but the table is still aligned to the configured size.
        ///
        /// The reset behavior of this field is:
        /// - When SMMU_IDR1.TABLES_PRESET == 1, this field resets to an IMPLEMENTATION DEFINED value.
        /// - Otherwise, this field resets to an UNKNOWN value.
        ADDR OFFSET(6) NUMBITS(50) [],
        /// Bits [5:0] Reserved, RES0.
        Reserved0 OFFSET(0) NUMBITS(6) []
    ]
}

/// SMMU Stream table base address register, Read-Write.
pub type StrtabBaseReg = ReadWrite<u64, STRTAB_BASE::Register>;
