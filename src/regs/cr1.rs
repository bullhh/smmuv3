//! Chapter 6.
//! Memory map and registers
//! 6.3. Register formats
//! 6.3.11 SMMU_CR1
//! The SMMU_CR1 characteristics are:
//! ## Purpose
//! Non-secure SMMU programming interface control and configuration register.
//! ## Attributes
//! SMMU_CR1 is a 32-bit register.
//! This register is part of the SMMUv3_PAGE_0 block.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u32,
    pub CR1 [
        /// Bits [31:12] Reserved, RES0.
        Reserved12 OFFSET(12) NUMBITS(19) [],
        /// TABLE_SH, bits [11:10]
        /// Table access Shareability.
        ///     - 0b00 Non-shareable.
        ///     - 0b01 Reserved, treated as 0b00.
        ///     - 0b10 Outer Shareable.
        ///     - 0b11 Inner Shareable.
        /// - Note: When SMMU_CR1.TABLE_OC == 0b00 and SMMU_CR1.TABLE_IC == 0b00, this field is IGNORED and behaves as Outer Shareable.
        /// The reset behavior of this field is:
        /// - When SMMU_IDR1.TABLES_PRESET == 1, this field resets to an IMPLEMENTATION DEFINED value.
        /// - Otherwise, this field resets to an UNKNOWN value.
        /// Accessing this field has the following behavior:
        /// - Access to this field is RW if all of the following are true:
        ///     - SMMU_CR0.SMMUEN == 0
        ///     – SMMU_CR0ACK.SMMUEN == 0
        /// - Otherwise, access to this field is RO.
        TABLE_SH OFFSET(10) NUMBITS(2) [
            NonShareable = 0b00,
            /// treated as 0b00
            Reserved = 0b01,
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],
        /// TABLE_OC, bits [9:8]
        /// Table access Outer Cacheability.
        /// - 0b00 Non-cacheable.
        /// - 0b01 Write-Back Cacheable.
        /// - 0b10 Write-Through Cacheable.
        /// - 0b11 Reserved, treated as 0b00.
        /// The reset behavior of this field is:
        /// - When SMMU_IDR1.TABLES_PRESET == 1, this field resets to an IMPLEMENTATION DEFINED value.
        /// - Otherwise, this field resets to an UNKNOWN value.
        /// Accessing this field has the following behavior:
        /// -  Access to this field is RW if all of the following are true:
        ///     – SMMU_CR0.SMMUEN == 0
        ///     – SMMU_CR0ACK.SMMUEN == 0
        /// - Otherwise, access to this field is RO.
        TABLE_OC OFFSET(8) NUMBITS(2) [
            NonCacheable = 0b00,
            WriteBackCacheable = 0b01,
            WriteThroughCacheable = 0b10,
            /// treated as 0b00
            Reserved = 0b11
        ],
        /// TABLE_IC, bits [7:6]
        /// Table access Inner Cacheability.
        /// - 0b00 Non-cacheable.
        /// - 0b01 Write-Back Cacheable.
        /// - 0b10 Write-Through Cacheable.
        /// - 0b11 Reserved. Treated as 0b00.
        /// The reset behavior of this field is:
        /// - When SMMU_IDR1.TABLES_PRESET == 1, this field resets to an IMPLEMENTATION DEFINED value.
        /// - Otherwise, this field resets to an UNKNOWN value.
        ///  Accessing this field has the following behavior:
        /// - Access to this field is RW if all of the following are true:
        ///     - SMMU_CR0.SMMUEN == 0
        ///     – SMMU_CR0ACK.SMMUEN == 0
        /// - Otherwise, access to this field is RO.
        TABLE_IC OFFSET(6) NUMBITS(2) [
            NonCacheable = 0b00,
            WriteBackCacheable = 0b01,
            WriteThroughCacheable = 0b10,
            /// treated as 0b00
            Reserved = 0b11
        ],
        /// QUEUE_SH, bits [5:4]
        /// Queue access Shareability.
        ///     - 0b00 Non-shareable.
        ///     - 0b01 Reserved, treated as 0b00.
        ///     - 0b10 Outer Shareable.
        ///     - 0b11 Inner Shareable.
        QUEUE_SH OFFSET(4) NUMBITS(2) [
            NonShareable = 0b00,
            /// treated as 0b00
            Reserved = 0b01,
            OuterShareable = 0b10,
            InnerShareable = 0b11
        ],
        /// QUEUE_OC, bits [3:2]
        /// Queue access Outer Cacheability.
        /// - 0b00 Non-cacheable.
        /// - 0b01 Write-Back Cacheable.
        /// - 0b10 Write-Through Cacheable.
        /// - 0b11 Reserved, treated as 0b00.
        QUEUE_OC OFFSET(2) NUMBITS(2) [
            NonCacheable = 0b00,
            WriteBackCacheable = 0b01,
            WriteThroughCacheable = 0b10,
            /// treated as 0b00
            Reserved = 0b11
        ],
        /// QUEUE_IC, bits [1:0]
        /// Queue access Inner Cacheability.
        /// - 0b00 Non-cacheable.
        /// - 0b01 Write-Back Cacheable.
        /// - 0b10 Write-Through Cacheable.
        /// - 0b11 Reserved, treated as 0b00.
        QUEUE_IC OFFSET(0) NUMBITS(2) [
            NonCacheable = 0b00,
            WriteBackCacheable = 0b01,
            WriteThroughCacheable = 0b10,
            /// treated as 0b00
            Reserved = 0b11
        ]
    ]
}

/// CR1 register, read-write.
pub type Cr1Reg = ReadWrite<u32, CR1::Register>;
