use core::marker::PhantomData;

use aarch64_cpu::registers::VTCR_EL2;

use memory_addr::{pa, PhysAddr, PAGE_SIZE_4K};

use crate::hal::PagingHandler;

const STRTAB_STE_DWORDS_BITS: usize = 3;
const STRTAB_STE_DWORDS: usize = 1 << STRTAB_STE_DWORDS_BITS;
const STRTAB_STE_SIZE: usize = STRTAB_STE_DWORDS << 3;

/// V, bit [0]
/// STE Valid.
///
/// - 0b0 Structure contents are invalid. Other STE fields are IGNORED.
/// - 0b1 Structure contents are valid. Other STE fields behave as described.
const STRTAB_STE_0_V: u64 = 0b1 << 0;
/// Config, bits [3:1]
/// Stream configuration.
///
/// | Value | Traffic can pass? | Stage 1 | Stage 2 | Notes |
///  
/// * 0b000 No   –            –           Report abort to device, no event recorded.
/// * 0b0xx No   –            –           Reserved (behaves as 0b000)
/// * 0b100 Yes  Bypass       Bypass      STE.EATS value effectively 0b00
/// * 0b101 Yes  Translate    Bypass      S1* valid
/// * 0b110 Yes  Bypass       Translate   S2* valid
/// * 0b111 Yes  Translate    Translate   S1* and S2* valid.
const STRTAB_STE_0_CFG_S1_BYPASS_S2_BYPASS: u64 = 0b100 << 1;
const STRTAB_STE_0_CFG_S1_BYPASS_S2_TRANS: u64 = 0b110 << 1;
/// SHCFG, bits [109:108]
/// Shareability configuration.
///
/// - 0b00 Non-shareable
/// - 0b01 Use incoming Shareability attribute
/// - 0b10 Outer shareable
/// - 0b11 Inner shareable
const STRTAB_STE_1_SHCFG_INCOMING: u64 = 0b01 << 44; // 44 = 108 - 64
/// S2VMID, bits [143:128]
/// Virtual Machine Identifier
///
/// Marks TLB entries inserted because of translations located through this STE, differentiating them from translations belonging to different virtual machines.
///
const STRTAB_STE_2_S2VMID_OFFSET: u64 = 0; // 0 = 128 - 128
/// S2T0SZ, bits [165:160]
///
/// Size of IPA input region covered by stage 2 translation table.
///
/// This field is equivalent to VTCR_EL2.T0SZ in the A-profile architecture.
const STRTAB_STE_2_S2T0SZ_OFFSET: u64 = 32; // 32 = 160 - 128

/// S2T0SZ, bits [165:160]
/// S2SL0, bits [167:166]
/// S2IR0, bits [169:168]
/// S2OR0, bits [171:170]
/// S2SH0, bits [173:172]
/// S2TG, bits [175:174]
/// S2PS, bits [178:176]
/// Overall, bits [178:160] refers to the lower 19 bits of [`aarch64_cpu::registers::VTCR_EL2`].
const STRTAB_STE_2_S2VTCR_LEN: u64 = 19;

const DEFAULT_S2VTCR: u64 = VTCR_EL2::PS::PA_40B_1TB.mask()
    | VTCR_EL2::TG0::Granule4KB.mask()
    | VTCR_EL2::SH0::Inner.mask()
    | VTCR_EL2::ORGN0::NormalWBRAWA.mask()
    | VTCR_EL2::IRGN0::NormalWBRAWA.mask()
    | VTCR_EL2::SL0.val(0b01).mask()
    | VTCR_EL2::T0SZ.val(16).mask();

/// S2AA64, bit [179]
///
/// Stage 2 translation table format for S2TTB0, and S_S2TTB0 if appropriate.
///
/// - 0b0 Use VMSAv8-32 LPAE descriptor formats. SMMU_IDR0.TTF[0] == 1
/// - 0b0 Use VMSAv9-128 descriptor formats. SMMU_IDR5.D128 == 1
/// - 0b1 Use VMSAv8-64 descriptor formats.
///
/// If stage 2 is not implemented, that is when SMMU_IDR0.S2P == 0, this field is RES0.
const STRTAB_STE_2_S2AA64: u64 = 1 << 51; // 51 = 179 - 128
/// S2PTW, bit [182]
/// Protected Table Walk.
///
/// For an STE configured for translation at both stages, a stage 1 translation table walk access or CD fetch access made to a stage 2 page with any Device type is terminated and recorded as a stage 2 Permission fault if this field is set.
///
/// Note: This might provide early indication of a programming error.
///
/// - 0b0 If SMMU_IDR3.PTWNNC == 0: CD fetch and stage 1 translation table walks allowed to any valid stage 2 address. If SMMU_IDR3.PTWNNC == 1: A translation table access or CD fetch mapped as any Device type occurs as if it is to Normal Non-cacheable memory.
/// - 0b1 CD fetch or Stage 1 translation table walks to stage 2 addresses mapped as any Device are terminated. A stage 2 Permission fault is recorded.
const STRTAB_STE_2_S2PTW: u64 = 1 << 54; // 54 = 182 - 128

/// S2S, bit [185]
/// Stage 2 fault behavior - Stall
/// See section 5.5 Fault configuration (A, R, S bits) for a description of fault configuration.
/// When STE.Config == 0b10x (Stage 2 disabled), {S2S, S2R} are IGNORED.
/// If stage 2 is not implemented, that is when SMMU_IDR0.S2P == 0, this field is RES0.
const STRTAB_STE_2_S2S: u64 = 1 << 57; // 57 = 185 - 128
/// S2R, bit [186]
///
/// Stage 2 fault behavior - Record.
/// See section 5.5 Fault configuration (A, R, S bits) for a description of fault configuration.
/// When STE.Config == 0b10x (Stage 2 disabled), {S2R, S2S} are IGNORED.
/// If stage 2 is not implemented, that is when SMMU_IDR0.S2P == 0, this field is RES0.
const STRTAB_STE_2_S2R: u64 = 1 << 58; // 58 = 186 - 128
/// S2TTB, bits [247:196]
/// In SMMUv3.1 and later, if STE.S2AA64 selects VMSAv9-128, then bits[247:196] represent the address of Stage 2 Translation Table base, bits[55:4]. Otherwise:
/// - In SMMUv3.1 and later:
///     – Bits[243:196] represent the address of Stage 2 Translation Table base, bits[51:4].
///     – Bits[247:244] are RES0.
/// - In SMMUv3.0:
///     – Bits[239:196] represent the address of Stage 2 Translation Table base, bits[47:4].
///     – Bits[247:240] are RES0.
///
/// Address bits above and below the field range are treated as zero.
///
/// Bits [(x-1):0] are treated as if all the bits are zero, where x is defined by the required alignment of the translation table as given in the A-profile architecture[2].
/// Note: The SMMU effectively aligns the value in this field before use.
const STRTAB_STE_3_S2TTB_OFF: u64 = 4;
const STRTAB_STE_3_S2TTB_LEN: u64 = 48;

const fn extract_bits(value: u64, start: u64, length: u64) -> u64 {
    let mask = (1 << length) - 1;
    (value >> start) & mask
}

#[allow(unused)]
pub struct StreamTableEntry([u64; STRTAB_STE_DWORDS]);

impl StreamTableEntry {
    pub const fn bypass_entry() -> Self {
        Self([
            STRTAB_STE_0_V | STRTAB_STE_0_CFG_S1_BYPASS_S2_BYPASS,
            STRTAB_STE_1_SHCFG_INCOMING,
            0,
            0,
            0,
            0,
            0,
            0,
        ])
    }

    pub const fn s2_translated_entry(vmid: u64, s2pt_base: PhysAddr) -> Self {
        Self([
            STRTAB_STE_0_V | STRTAB_STE_0_CFG_S1_BYPASS_S2_TRANS,
            STRTAB_STE_1_SHCFG_INCOMING,
            (vmid << STRTAB_STE_2_S2VMID_OFFSET)
                | extract_bits(DEFAULT_S2VTCR, 0, STRTAB_STE_2_S2VTCR_LEN)
                    << STRTAB_STE_2_S2T0SZ_OFFSET
                | STRTAB_STE_2_S2AA64
                | STRTAB_STE_2_S2PTW
                | STRTAB_STE_2_S2R,
            extract_bits(
                s2pt_base.as_usize() as u64,
                STRTAB_STE_3_S2TTB_OFF,
                STRTAB_STE_3_S2TTB_LEN,
            ) << STRTAB_STE_3_S2TTB_OFF,
            0,
            0,
            0,
            0,
        ])
    }
}

pub struct LinearStreamTable<H: PagingHandler> {
    base: PhysAddr,
    entry_count: usize,
    _phantom: PhantomData<H>,
}

impl<H: PagingHandler> LinearStreamTable<H> {
    pub const fn uninit() -> Self {
        Self {
            base: pa!(0xdead_beef),
            entry_count: 0,
            _phantom: PhantomData,
        }
    }

    pub fn init(&mut self, sid_bits: u32) {
        self.entry_count = 1 << sid_bits;
        let size = self.entry_count * STRTAB_STE_SIZE;
        let base = H::alloc_pages(size / PAGE_SIZE_4K).expect("Failed to allocate stream table");
        self.base = base;
        info!(
            "Stream table base address: {:?}, entry_count: {}, size: {}",
            self.base,
            self.entry_count,
            size
        );
        // First we just mark all entries as bypass.
        for sid in 0..self.entry_count {
            self.set_bypass_ste(sid);
        }
    }

    pub fn base_addr(&self) -> PhysAddr {
        self.base
    }

    fn ste(&self, sid: usize) -> &mut StreamTableEntry {
        let base = self.base + sid * STRTAB_STE_SIZE;
        unsafe { &mut *(base.as_usize() as *mut StreamTableEntry) }
    }

    fn set_bypass_ste(&self, sid: usize) {
        let tab = self.ste(sid);
        *tab = StreamTableEntry::bypass_entry();
    }

    pub(crate) fn set_s2_translated_ste(&self, sid: usize, vmid: usize, s2pt_base: PhysAddr) {
        // info!(
        //     "write ste, sid: 0x{:x}, vmid: 0x{:x}, ste_addr:0x{:x}, root_pt: {:?}",
        //     sid,
        //     vmid,
        //     self.base + sid * STRTAB_STE_SIZE,
        //     s2pt_base
        // );

        let entry = self.ste(sid);
        *entry = StreamTableEntry::s2_translated_entry(vmid as _, s2pt_base);
    }

    pub fn entry_count(&self) -> usize {
        self.entry_count
    }
}
