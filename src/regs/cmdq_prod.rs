//! Chapter 6. Memory map and registers
//! 6.3. Register formats
//! 6.3.27 SMMU_CMDQ_PROD
//!
//! The SMMU_CMDQ_PROD characteristics are:
//!
//! ## Purpose
//! Allows Command queue producer to update the write index.
//!
//! ## Attributes
//! SMMU_CMDQ_PROD is a 32-bit register.
//! This register is part of the SMMUv3_PAGE_0 block.
//!
use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u32,
    pub CMDQ_PROD [
        /// Bits [31:20] Reserved, RES0.
        Reserved31 OFFSET(20) NUMBITS(12) [],
        /// WR, bits [19:0]
        /// Command queue write index.
        ///
        /// This field is treated as two sub-fields, depending on the configured queue size:
        /// - **Bit [QS]: WR_WRAP** - Command queue write index wrap flag.
        /// - **Bits [QS-1:0]: WR** - Command queue write index.
        ///     - Updated by the host PE (producer) indicating the next empty space in the queue after new data.
        ///
        /// The reset behavior of this field is:
        /// - This field resets to an UNKNOWN value.
        WR OFFSET(0) NUMBITS(20) []
    ]
}

/// Additional Information
///
/// QS == SMMU_CMDQ_BASE.LOG2SIZE, see SMMU_CMDQ_CONS.
///
/// If QS < 19, bits [19:QS + 1] are RES0. If software writes a non-zero value to these bits, the value might be stored but has no other effect. In addition, if SMMU_IDR1.CMDQS < 19, bits [19:CMDQS+1] are UNKNOWN on read.
///
/// If QS == 0 the queue has one entry. Zero bits of WR index are present and WR_WRAP is bit zero.
///
/// When software increments WR, if the index would pass off the end of the queue it must be correctly wrapped to the queue size given by QS and WR_WRAP toggled.
///
/// Note: In the degenerate case of a one-entry queue, an increment of WR consists solely of a toggle of WR_WRAP.
/// There is space in the queue for additional commands if:
///
/// `SMMU_CMDQ_CONS.RD != SMMU_CMDQ_PROD.WR || SMMU_CMDQ_CONS.RD_WRAP == SMMU_CMDQ_PROD.WR_WRAP`
pub type CmdQProdReg = ReadWrite<u32, CMDQ_PROD::Register>;


register_bitfields! {u32,
    pub EVENTQ_PROD [
        /// OVSLG, bit [31] Overflow flag.
        OVSLG OFFSET(31) NUMBITS(1) [],
        /// Bits [30:20] Reserved, RES0.
        Reserved31 OFFSET(20) NUMBITS(11) [],
        /// WR, bits [19:0]
        /// event queue write index.
        ///
        /// This field is treated as two sub-fields, depending on the configured queue size:
        /// - **Bit [QS]: WR_WRAP** -  queue write index wrap flag.
        /// - **Bits [QS-1:0]: WR** -  queue write index.
        ///     - Updated by the host PE (producer) indicating the next empty space in the queue after new data.
        ///
        /// The reset behavior of this field is:
        /// - This field resets to an UNKNOWN value.
        WR OFFSET(0) NUMBITS(20) []
    ]
}

pub type EventQProdReg = ReadWrite<u32, EVENTQ_PROD::Register>;