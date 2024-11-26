//! Chapter 6. Memory map and registers
//! 6.3. Register formats
//! 6.3.8 SMMU_AIDR
//!
//! The SMMU_AIDR characteristics are:
//!
//! ## Purpose
//! This register identifies the SMMU architecture version to which the implementation conforms.
//!
//! ## Attributes
//! SMMU_AIDR is a 32-bit register.
//!
//! This register is part of the SMMUv3_PAGE_0 block.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadOnly;

register_bitfields! {u32,
    pub AIDR [
        /// Major Architecture revision.
        ///
        /// - 0b0000 SMMUv3.x.
        ArchMajorRev OFFSET(4) NUMBITS(4) [
            SMMUv3_x = 0b0000
        ],
        /// Minor Architecture revision.
        ///
        /// - 0b0000 SMMUv3.0.
        /// - 0b0001 SMMUv3.1.
        /// - 0b0010 SMMUv3.2.
        /// - 0b0011 SMMUv3.3.
        /// - 0b0100 SMMUv3.4.
        ArchMinorRev OFFSET(0) NUMBITS(4) [
            SMMUv3_0 = 0b0000,
            SMMUv3_1 = 0b0001,
            SMMUv3_2 = 0b0010,
            SMMUv3_3 = 0b0011,
            SMMUv3_4 = 0b0100
        ],
    ]
}

/// SMMU_AIDR register, read-write.
pub type AIDRReg = ReadOnly<u32, AIDR::Register>;
