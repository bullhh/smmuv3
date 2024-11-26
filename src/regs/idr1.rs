//! Chapter 6. Memory map and registers
//! 6.3. Register formats
//! 6.3.2 SMMU_IDR1
//! The SMMU_IDR1 characteristics are:
//!
//! ## Purpose
//! Provides information about the features implemented for the SMMU Non-secure programming interface.
//!
//! ## Attributes
//! SMMU_IDR1 is a 32-bit register.
//!
//! This register is part of the SMMUv3_PAGE_0 block.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadOnly;

register_bitfields! {u32,
    pub IDR1 [
        /// Support for enhanced Command queue interface.
        ///
        /// - 0b0 Enhanced Command queue interface not supported. SMMU_IDR6 is RES0.
        /// - 0b1 Enhanced Command queue interface details are advertised in SMMU_IDR6.
        ECMDQS OFFSET(31) NUMBITS(1) [
            NotSupported = 0,
            Supported = 1
        ],
        /// Table base addresses fixed.
        ///
        /// - 0b0 The contents of the registers SMMU_(*_)STRTAB_BASE and SMMU_(*_)STRTAB_BASE_CFG are not fixed.
        /// - 0b1 The contents of the registers SMMU_(*_)STRTAB_BASE and SMMU_(*_)STRTAB_BASE_CFG are fixed
        TABLES_PRESET OFFSET(30) NUMBITS(1) [
            NotFixed = 0,
            Fixed = 1
        ],
        /// Queue base addresses fixed.
        ///
        /// - 0b0 The contents of the registers SMMU_(*_)CMDQ_BASE, SMMU_(*_)EVENTQ_BASE, and if present, SMMU_(R_)PRIQ_BASE are not fixed.
        /// - 0b1 The contents of the registers SMMU_(*_)CMDQ_BASE, SMMU_(*_)EVENTQ_BASE, and if present, SMMU_(R_)PRIQ_BASE are fixed.
        QUEUES_PRESET OFFSET(29) NUMBITS(1) [
            NotFixed = 0,
            Fixed = 1
        ],
        /// Relative base pointers.
        ///
        /// - 0b0 When the corresponding preset field is set, base address registers report an absolute address.
        /// - 0b1 When the corresponding preset field is set, base address registers report an address offset.
        ///     - Relative addresses are calculated using an addition of the unsigned ADDR field onto the base address of Page 0.
        REL OFFSET(28) NUMBITS(1) [
            Absolute = 0,
            Relative = 1
        ],
        /// Incoming MemType, Shareability, allocation and transient hints override.
        ///
        /// - 0b0 Incoming attributes cannot be overridden before translation or by global bypass.
        /// - 0b1 Incoming attributes can be overridden.
        ATTR_TYPES_OVR OFFSET(27) NUMBITS(1) [
            NotSupported = 0,
            Supported = 1
        ],
        /// Incoming Data or Instruction, User or Privileged, input NS attribute override.
        ///
        /// - 0b0 Incoming attributes cannot be overridden before translation or by global bypass.
        /// - 0b1 Incoming attributes can be overridden.
        ATTR_PERMS_OVR OFFSET(26) NUMBITS(1) [
            NotSupported = 0,
            Supported = 1
        ],
        /// Maximum number of Command queue entries.
        ///
        /// - Maximum number of entries as log2(entries).
        ///     - Maximum value 19.
        /// - Note: The index register values include an extra bit for wrap. Therefore a queue with 2N entries has indices of N bits, but an index register containing (N+1) bits.
        CMDQS OFFSET(21) NUMBITS(5) [],
        /// Maximum number of Event queue entries.
        ///
        /// - Maximum number of entries as log2(entries).
        ///     - Maximum value 19.
        EVENTQS OFFSET(16) NUMBITS(5) [],
        /// Maximum number of PRI queue entries.
        ///
        /// - Maximum number of entries as log2(entries).
        ///     - Maximum value 19.
        ///     - If SMMU_IDR0.PRI == 0, this field has an IMPLEMENTATION SPECIFIC value.
        PRIQS OFFSET(11) NUMBITS(5) [],
        /// Max bits of SubstreamID.
        ///
        /// - Valid range 0 to 20 inclusive, 0 meaning no substreams are supported.
        /// - Reflects physical SubstreamID representation size, that is the SMMU cannot represent or be presented with SubstreamIDs greater than SSIDSIZE.
        SSIDSIZE OFFSET(6) NUMBITS(5) [],
        /// Max bits of StreamID.
        ///
        /// - This value is between 0 and 32 inclusive.
        ///     - Note: 0 is a legal value. In this case the SMMU supports one stream.
        /// - This must reflect the physical StreamID size, that is the SMMU cannot represent or be presented with StreamIDs greater than SIDSIZE.
        ///     - When SMMU_IDR1.SIDSIZE >= 7, SMMU_IDR0.ST_LEVEL != 0b00.
        SIDSIZE OFFSET(0) NUMBITS(6) [],
    ]
}

/// IDR1 Register, read-only.
pub type IDR1Reg = ReadOnly<u32, IDR1::Register>;
