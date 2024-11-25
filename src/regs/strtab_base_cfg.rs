//! Chapter 6. Memory map and registers
//! 6.3. Register formats
//! 6.3.25 SMMU_STRTAB_BASE_CFG
//!
//! The SMMU_STRTAB_BASE_CFG characteristics are:
//!
//! ## Purpose
//! Configuration of Stream table.
//!
//! ## Attributes
//! SMMU_STRTAB_BASE_CFG is a 32-bit register.
//!
//! This register is part of the SMMUv3_PAGE_0 block.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u32,
    pub STRTAB_BASE_CFG [
        /// Bits [31:18] Reserved, RES0.
        Reserved31 OFFSET(18) NUMBITS(14) [],
        /// When SMMU_IDR0.ST_LEVEL != 0b00: 
        /// Format of Stream table. 
        /// - 0b00 Linear - ADDR points to an array of STEs. 
        /// - 0b01 2-level - ADDR points to an array of Level 1 Stream Table Descriptors.
        /// 
        /// Other values are reserved, behave as 0b00.
        /// 
        /// The reset behavior of this field is:
        /// - When SMMU_IDR1.TABLES_PRESET == 1, this field resets to an IMPLEMENTATION DEFINED value.
        /// - Otherwise, this field resets to an UNKNOWN value. 
        /// 
        /// Otherwise: Reserved, RES0.
        FMT OFFSET(16) NUMBITS(2) [
            Linear = 0b00,
            TwoLevel = 0b01
        ],
        /// Bits [15:11] Reserved, RES0.
        Reserved15 OFFSET(11) NUMBITS(5) [],
        /// When SMMU_IDR0.ST_LEVEL != 0b00: 
        /// StreamID split point for multi-level table.
        /// - This field determines the split point of a 2-level Stream table, selected by the number of bits at the bottom level.
        /// - This field is IGNORED if FMT == 0b00.
        ///     - 0b00110 6 bits - 4KB leaf tables.
        ///     - 0b01000 8 bits - 16KB leaf tables.
        ///     - 0b01010 10 bits - 64KB leaf tables.
        ///     - Other values are reserved, behave as 6 (0b0110).
        /// 
        /// - The upper-level L1STD is located using StreamID[LOG2SIZE - 1:SPLIT] and this indicates the lowest-level table which is indexed by StreamID[SPLIT - 1:0].
        ///     - For example, selecting SPLIT == 6 (0b0110) causes StreamID[5:0] to be used to index the lowest level Stream table and StreamID[LOG2SIZE - 1:6] to index the upper level table.
        /// - Note: If SPLIT >= LOG2SIZE, a single upper-level descriptor indicates one bottom-level Stream table with 2LOG2SIZE usable entries. The L1STD.Span value’s valid range is up to SPLIT + 1, but not all of this Span is accessible, as it is not possible to use a StreamID >= 2LOG2SIZE. 
        /// 
        /// Note: Arm recommends that a Linear table, FMT == 0b00, is used instead of programming SPLIT > LOG2SIZE. 
        /// 
        /// The reset behavior of this field is:
        /// - When SMMU_IDR1.TABLES_PRESET == 1, this field resets to an IMPLEMENTATION DEFINED value.
        /// - Otherwise, this field resets to an UNKNOWN value. 
        /// 
        /// Otherwise: Reserved, RES0.
        SPLIT OFFSET(6) NUMBITS(5) [
            Split6Bits = 0b00110,
            Split8Bits = 0b01000,
            Split10Bits = 0b01010
        ],
        /// Table size as log2(entries).
        ///
        /// - The maximum StreamID value that can be used to index into the Stream table is 2LOG2SIZE - 1. The StreamID range is equal to the number of STEs in a linear Stream table or the maximum sum of the STEs in all second-level tables. The number of L1STDs in the upper level of a 2-level table is MAX(1, 2LOG2SIZE-SPLIT). Except for readback of a written value, the effective LOG2SIZE is <= SMMU_IDR1.SIDSIZE for the purposes of input StreamID range checking and upper/lower/linear Stream table index address calculation.
        ///
        /// The reset behavior of this field is:
        /// - When SMMU_IDR1.TABLES_PRESET == 1, this field resets to an IMPLEMENTATION DEFINED value.
        /// - Otherwise, this field resets to an UNKNOWN value.
        LOG2SIZE OFFSET(0) NUMBITS(6) []
    ]
}

/// SMMU_STRTAB_BASE_CFG is Guarded by SMMU_CR0.SMMUEN and must only be written when SMMU_CR0.SMMUEN == 0.
/// See SMMU_STRTAB_BASE for detailed behavior.
pub type StrtabBaseCfgReg = ReadWrite<u32, STRTAB_BASE_CFG::Register>;
