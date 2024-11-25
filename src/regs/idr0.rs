use tock_registers::register_bitfields;
use tock_registers::registers::ReadOnly;

register_bitfields! {u32,
    pub IDR0 [
        /// Multi-level Stream table support.
        /// 
        /// - 0b00 Linear Stream table supported.
        /// - 0b01 2-level Stream table supported in addition to Linear Stream table.
        /// - 0b1x Reserved.
        ST_LEVEL OFFSET(27) NUMBITS(2) [
            LinearStreamTable = 0b00,
            TwoLevelStreamTableInAdditionToLinearStreamTable = 0b01
        ],
        /// 16-bit VMID supported.
        /// 
        /// - 0b0 16-bit VMID not supported.
        ///     - VMID[15:8] is RES0in command parameters and must be zero in STE.S2VMID.
        /// - 0b1 16-bit VMID supported.
        VMID16 OFFSET(18) NUMBITS(1) [
            NotSupported = 0,
            Supported = 1
        ],
        /// Address Translation Operations supported.
        ///
        /// - 0b0 Address Translation Operations not supported.
        ///     - MMU_IDR0.VATOS is RES0 and all SMMU_(S_)GATOS_* registers are Reserved.
        /// - 0b1 Address Translation Operations supported
        ATOS OFFSET(15) NUMBITS(1) [
            NotSupported = 0,
            Supported = 1
        ],
        /// H/W translation table Access flag and Dirty state of the page updates supported.
        /// 
        /// - 0b00 No flag updates supported.
        /// - 0b01 Access flag update supported.
        /// - 0b10 Access flag and Dirty state of the page update supported.
        /// - 0b11 Access flag and Dirty state, and Access flag for Table descriptors supported.
        HTTU OFFSET(6) NUMBITS(2) [
            NoFlags = 0b00,
            AccessFlag = 0b01,
            AccessFlagDirtyState = 0b10,
            AccessFlagDirtyStateAccessFlagTableDescriptors = 0b11
        ],
        /// Broadcast TLB Maintenance. Indicates support for receiving broadcast TLBI operations issued by Arm PEs in the system.
        /// 
        /// - 0b0 Broadcast TLB maintenance not supported.
        /// - 0b1 Boradcast TLB maintenance supported.
        ///
        /// This bit reflects the ability of the system, and SMMU implementation, to support broadcast maintenance. If either the SMMU, or the system, or the interconnect cannot fully support broadcast TLB maintenance, this bit reads as 0.
        BTM OFFSET(5) NUMBITS(1) [
            NotSupported = 0,
            Supported = 1
        ],
        /// Coherent access supported to translations, structures and queues.
        /// 
        /// - 0b0 Coherent access for translations, structures and queues is not supported.
        /// - 0b1 IO-coherent access is supported for: • Translation table walks.
        ///     - Fetches of L1STD, STE, L1CD and CD. • Command queue, Event queue and PRI queue access.
        ///     - GERROR, CMD_SYNC, Event queue and PRI queue MSIs, if supported.
        ///     - Whether a specific access is performed in a cacheable shareable manner is dependent on the access type configured for access to structures, queues and translation table walks.
        CHOACC OFFSET(4) NUMBITS(1) [
            NotSupported = 0,
            Supported = 1
        ],
        /// Translation table formats supported at both stage 1 and stage 2.
        ///
        /// - 0b00 Reserved.
        /// - 0b01 VMSAv8-32 LPAE.
        /// - 0b10 VMSAv8-64.
        /// - 0b11 VMSAv8-32 LPAE and VMSAv8-64.
        ///
        /// TTF[0] is 0 in implementations where either SMMU_IDR3.DPT is 1 or SMMU_R_IDR3.DPT is 1.
        TTF OFFSET(2) NUMBITS(2) [
            Reserved = 0b00,
            VMSAV8_32_LPAE = 0b01,
            VMSAV8_64 = 0b10,
            VMSAV8_32_LPAE_AND_VMSAV8_64 = 0b11
        ],
        /// Stage1 translation supported.
        ///
        /// - 0b0 Stage 1 translation not supported.
        /// - 0b1 Stage 1 translation supported.
        S1P OFFSET(1) NUMBITS(1) [
            Supported = 1,
            NotSupported = 0
        ],
        /// Stage2 translation supported.
        ///
        /// - 0b0 Stage 2 translation not supported.
        /// - 0b1 Stage 2 translation supported.
        S2P OFFSET(0) NUMBITS(1) [
            Supported = 1,
            NotSupported = 0
        ],
    ]
}

pub type IDR0Reg = ReadOnly<u32, IDR0::Register>;
