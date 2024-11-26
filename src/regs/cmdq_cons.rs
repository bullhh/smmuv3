//! Chapter 6. Memory map and registers
//! 6.3. Register formats
//! 6.3.28 SMMU_CMDQ_CONS
//!
//! The SMMU_CMDQ_CONS characteristics are:
//!
//! ## Purpose
//! Command queue consumer read index.
//!
//! ## Attributes
//! SMMU_CMDQ_CONS is a 32-bit register.
//! This register is part of the SMMUv3_PAGE_0 block.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u32,
    pub CMDQ_CONS [
        /// Bit [31] Reserved, RES0.
        Reserved31 OFFSET(31) NUMBITS(1) [],
        /// ERR, bits [30:24] Error reason code.
        /// - When a command execution error is detected, ERR is set to a reason code and then the SMMU_GERROR.CMDQ_ERR global error becomes active.
        /// - The value in this field is UNKNOWN when the CMDQ_ERR global error is not active.
        ///
        /// The reset behavior of this field is: • This field resets to an UNKNOWN value.
        ERR OFFSET(24) NUMBITS(7) [],
        /// Bits [23:20] Reserved, RES0.
        Reserved23 OFFSET(20) NUMBITS(4) [],
        /// RD, bits [19:0] Command queue read index.
        /// This field is treated as two sub-fields, depending on the configured queue size:
        /// - **Bit [QS]: RD_WRAP** - Queue read index wrap flag.
        /// - **Bits [QS-1:0]: RD** - Queue read index.
        ///     - Updated by the SMMU (consumer) to point at the queue entry after the entry it has just consumed.
        ///
        /// The reset behavior of this field is:
        /// - This field resets to an UNKNOWN value.
        RD OFFSET(0) NUMBITS(20) []
    ]
}

/// QS == SMMU_CMDQ_BASE.LOG2SIZE and SMMU_CMDQ_BASE.LOG2SIZE <= SMMU_IDR1.CMDQS <= 19.
///
/// This gives a configurable-sized index pointer followed immediately by the wrap bit.
///
/// If QS < 19, bits [19:QS + 1] are RAZ. When incremented by the SMMU, the RD index is always wrapped to the current queue size given by SMMU_CMDQ_BASE.LOG2SIZE.
///
/// If QS == 0 the queue has one entry. Zero bits of RD index are present and RD_WRAP is bit zero.
///
/// When SMMU_CMDQ_BASE.LOG2SIZE is increased within its valid range, the value of the bits of this register that were previously above the old wrap flag position are UNKNOWN and when it is decreased, the value of the bits from the wrap flag downward are the effective truncation of the value in the old field.
pub type CmdQConsReg = ReadWrite<u32, CMDQ_CONS::Register>;
