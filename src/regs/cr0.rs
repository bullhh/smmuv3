//! Chapter 6. Memory map and registers
//! 6.3. Register formats
//! 6.3.9 SMMU_CR0
//!
//! The SMMU_CR0 characteristics are:
//! ## Purpose
//! Non-secure SMMU programming interface control and configuration register
//! ## Attributes
//! SMMU_CR0 is a 32-bit register.
//! This register is part of the SMMUv3_PAGE_0 block.

use tock_registers::register_bitfields;
use tock_registers::registers::ReadWrite;

register_bitfields! {u32,
    pub CR0 [
        /// Bits [31:11] Reserved, RES0.
        Reserved11 OFFSET(11) NUMBITS(21) [],
        /// DPT_WALK_EN, bit [10]
        /// When SMMU_IDR3.DPT == 1:
        /// Enable DPT walks for Non-secure state.
        /// - 0b0 Non-secure DPT walks are disabled.
        /// - 0b1 Non-secure DPT walks are enabled.
        ///  This field has similar Update behavior to other CR0 fields, in that: When it is writable and its value is changed by a write, the SMMU begins a transition which is then acknowledged by updating SMMU_CR0ACK.DPT_WALK_EN to the new value.
        /// Otherwise: Reserved, RES0.
        DPT_WALK_EN OFFSET(10) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],
        /// Bit [9] Reserved, RES0.
        Reserved9 OFFSET(9) NUMBITS(1) [],
        /// VMW, bits [8:6]
        /// When SMMU_IDR0.VMW == 1: VMID Wildcard.
        /// - 0b000 TLB invalidations match VMID tags exactly.
        /// - 0b001 TLB invalidations match VMID[N:1].
        /// - 0b010 TLB invalidations match VMID[N:2].
        /// - 0b011 TLB invalidations match VMID[N:3].
        /// - 0b100 TLB invalidations match VMID[N:4].
        ///
        /// All other values are reserved, and behave as 0b000.
        /// - N == upper bit of VMID as determined by SMMU_IDR0.VMID16.
        ///
        /// This field has no effect on VMID matching on translation lookup.
        ///
        /// The reset behavior of this field is:
        /// - This field resets to 0b000.
        ///
        /// Otherwise: Reserved, RES0.
        VMW OFFSET(6) NUMBITS(3) [
            MatchVMIDExactly = 0b000,
            MatchVMIDN1 = 0b001,
            MatchVMIDN2 = 0b010,
            MatchVMIDN3 = 0b011,
            MatchVMIDN4 = 0b100
        ],
        /// Bit [5] Reserved, RES0.
        Reserved5 OFFSET(5) NUMBITS(1) [],
        /// ATSCHK, bit [4]
        /// When SMMU_IDR0.ATS == 1:
        /// ATS behavior.
        /// - 0b0 Fast mode, all ATS Translated traffic passes through the SMMU without Stream table or TLB lookup.
        /// - 0b1 Safe mode, all ATS Translated traffic is checked against the corresponding STE.EATS field to determine whether the StreamID is allowed to produce Translated transactions.
        ///
        /// The reset behavior of this field is:
        /// - This field resets to 0b0.
        ///
        /// Otherwise: Reserved, RES0.
        ATSCHK OFFSET(4) NUMBITS(1) [
            FastMode = 0,
            SafeMode = 1
        ],
        /// CMDQEN, bit [3]
        /// Enable Command queue processing.
        ///
        /// - 0b0 Processing of commands from the Non-secure Command queue is disabled.
        /// - 0b1 Processing of commands from the Non-secure Command queue is enabled.
        ///
        ///  The reset behavior of this field is:
        /// - This field resets to 0b0.
        CMDQEN OFFSET(3) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],
        /// EVENTQEN, bit [2] Enable Event queue writes.
        ///
        /// - 0b0 Writes to the Non-secure Event queue are disabled.
        /// - 0b1 Writes to the Non-secure Event queue are enabled.
        ///
        /// The reset behavior of this field is:
        /// - This field resets to 0b0.
        EVENTQEN OFFSET(2) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],
        /// PRIQEN, bit [1]
        /// - When SMMU_IDR0.PRI == 1:
        ///     - Enable PRI queue writes.
        ///         - 0b0 Writes to the PRI queue are disabled.
        ///         - 0b1 Writes to the PRI queue are enabled.
        ///     - The reset behavior of this field is:
        ///         - This field resets to 0b0.
        /// - Otherwise: Reserved, RES0.
        PRIQEN OFFSET(1) NUMBITS(1) [
            Disable = 0,
            Enable = 1
        ],
        /// SMMUEN, bit [0] Non-secure SMMU enable
        /// - 0b0 All Non-secure streams bypass SMMU, with attributes determined from SMMU_GBPA.
        /// - 0b1 All Non-secure streams are checked against configuration structures, and might undergo translation.
        ///
        /// The reset behavior of this field is:
        /// - This field resets to 0b0.
        SMMUEN OFFSET(0) NUMBITS(1) [
            Bypass = 0,
            Enable = 1
        ]
    ]
}

/// Each field in this register has a corresponding field in SMMU_CR0ACK. An individual field is described as Updated after the value of the field observed in SMMU_CR0ACK matches the value that was written to the field in [`Cr0Reg`]. Reserved fields in [`Cr0Reg`] are not reflected in SMMU_CR0ACK. To ensure a field change has taken effect, software must poll the equivalent field in SMMU_CR0ACK after writing the field in this register.
pub type Cr0Reg = ReadWrite<u32, CR0::Register>;
