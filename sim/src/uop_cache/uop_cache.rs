use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn, Result},
    insn_impl::{insn_cached, jump_table},
};

use super::set_cached_insn;

#[repr(C, packed)]
#[derive(Debug, Clone)]
pub struct UopCacheEntry {
    pub valid: bool,
    pub insn_bits: u64,

    /* regular registers */
    pub rs1: u64,
    pub rs2: u64,
    pub rd: u64,

    /* base instruction set immediates */
    pub imm_i: u64,
    pub imm_s: u64,
    pub imm_b: u64,
    pub imm_u: u64,
    pub imm_j: u64,
    pub shamtd: u64,
    pub shamtw: u64,

    /* compressed immediates */
    pub imm_c_lwsp: u64,
    pub imm_c_ldsp: u64,
    pub imm_c_swsp: u64,
    pub imm_c_sdsp: u64,
    pub imm_c_lw: u64,
    pub imm_c_ld: u64,
    pub imm_c_sw: u64,
    pub imm_c_sd: u64,
    pub imm_c_j: u64,
    pub imm_c_b: u64,
    pub imm_c_li: u64,
    pub imm_c_lui: u64,
    pub imm_c_addi: u64,
    pub imm_c_addi16sp: u64,
    pub imm_c_addi4spn: u64,
    pub imm_c_shamt: u64,
    pub imm_c_andi: u64,

    /* compressed registers */
    pub rs1_p: u64,
    pub rs2_p: u64,
    pub rd_p: u64,
    pub rd_rs1_n0: u64,
    pub rd_rs1_p: u64,
    pub rd_rs1: u64,
    pub rd_n2: u64,
    pub rd_n0: u64,
    pub rs1_n0: u64,
    pub c_rs2_n0: u64,
    pub c_rs1_n0: u64,
    pub c_rs2: u64,
    pub c_sreg1: u64,
    pub c_sreg2: u64,
    pub c_rlist: u64,

    /* float registers/rounding mode */
    pub rs3: u64,
    pub rm: u64,

    /* csrs */
    pub csr: u64,
    pub zimm: u64,

    pub jump_table_index: usize,
    pub op: fn(cpu: &mut Cpu, bus: &mut Bus, &UopCacheEntry) -> cpu::Result<u64>,
}

impl UopCacheEntry {
    pub fn default() -> Self {
        UopCacheEntry {
            valid: false,
            insn_bits: 0,
            rs1: 0,
            rs2: 0,
            rd: 0,
            imm_i: 0,
            imm_s: 0,
            imm_b: 0,
            imm_u: 0,
            imm_j: 0,
            shamtd: 0,
            shamtw: 0,
            imm_c_lwsp: 0,
            imm_c_ldsp: 0,
            imm_c_swsp: 0,
            imm_c_sdsp: 0,
            imm_c_lw: 0,
            imm_c_ld: 0,
            imm_c_sw: 0,
            imm_c_sd: 0,
            imm_c_j: 0,
            imm_c_b: 0,
            imm_c_li: 0,
            imm_c_lui: 0,
            imm_c_addi: 0,
            imm_c_addi16sp: 0,
            imm_c_addi4spn: 0,
            imm_c_shamt: 0,
            imm_c_andi: 0,
            rs1_p: 0,
            rs2_p: 0,
            rd_p: 0,
            rd_rs1_n0: 0,
            rd_rs1_p: 0,
            rd_rs1: 0,
            rd_n2: 0,
            rd_n0: 0,
            rs1_n0: 0,
            c_rs2_n0: 0,
            c_rs1_n0: 0,
            c_rs2: 0,
            c_sreg1: 0,
            c_sreg2: 0,
            c_rlist: 0,
            rs3: 0,
            rm: 0,
            csr: 0,
            zimm: 0,
            jump_table_index: 0,
            op: insn_cached::nop_cached::nop_cached, // default safe op
        }
    }

    pub fn new(insn: Insn) -> Self {
        let mut entry = UopCacheEntry {
            valid: false,
            insn_bits: 0,
            rs1: 0,
            rs2: 0,
            rd: 0,
            imm_i: 0,
            imm_s: 0,
            imm_b: 0,
            imm_u: 0,
            imm_j: 0,
            shamtd: 0,
            shamtw: 0,

            /* c immediates */
            imm_c_lwsp: 0,
            imm_c_ldsp: 0,
            imm_c_swsp: 0,
            imm_c_sdsp: 0,
            imm_c_lw: 0,
            imm_c_ld: 0,
            imm_c_sw: 0,
            imm_c_sd: 0,
            imm_c_j: 0,
            imm_c_b: 0,
            imm_c_li: 0,
            imm_c_lui: 0,
            imm_c_addi: 0,
            imm_c_addi16sp: 0,
            imm_c_addi4spn: 0,
            imm_c_shamt: 0,
            imm_c_andi: 0,

            /* c registers */
            rs1_p: 0,
            rs2_p: 0,
            rd_p: 0,
            rd_rs1_n0: 0,
            rd_rs1_p: 0,
            rd_rs1: 0,
            rd_n2: 0,
            rd_n0: 0,
            rs1_n0: 0,
            c_rs2_n0: 0,
            c_rs1_n0: 0,
            c_rs2: 0,
            c_sreg1: 0,
            c_sreg2: 0,
            c_rlist: 0,

            /* float registers/immediates */
            rs3: 0,
            rm: 0,

            /* csrs */
            csr: 0,
            zimm: 0,

            jump_table_index: 0,
            op: (insn_cached::nop_cached::nop_cached),
        };

        entry.insn_bits = insn.bits();

        /* regular registers */
        entry.rs1 = insn.rs1();
        entry.rs2 = insn.rs2();
        entry.rd = insn.rd();

        /* base immediates */
        entry.imm_i = insn.imm12();
        entry.imm_s = Insn::sign_extend((insn.imm12hi() << 5) | insn.imm12lo(), 12) as u64;
        entry.imm_b = Insn::sign_extend(
            ((insn.bimm12hi() & 0x40) << 6)
                | ((insn.bimm12lo() & 0x01) << 11)
                | ((insn.bimm12hi() & 0x3F) << 5)
                | (insn.bimm12lo() & 0x1E),
            13,
        ) as u64;
        entry.imm_u = Insn::sign_extend(insn.imm20() << 12, 32) as u64;
        entry.imm_j = Insn::sign_extend(
            ((insn.jimm20() & 0x80000) << 1) | ((insn.jimm20() & 0xff) << 12)
                | ((insn.jimm20() & 0x7fe00) >> 9 << 1)
                | ((insn.jimm20() & 0x100) >> 8 << 11),
            20,
        ) as u64;
        entry.shamtd = insn.shamtd();
        entry.shamtw = insn.shamtw();

        /* c immediates */
        entry.imm_c_lwsp =
            (insn.c_uimm8sphi() << 5) | ((insn.c_uimm8splo() & 0x3) << 6) | insn.c_uimm8splo() & 0x1c;
        entry.imm_c_ldsp =
            ((insn.c_uimm9splo() & 0x7) << 6) | (insn.c_uimm9sphi() << 5) | (insn.c_uimm9splo() & 0x18);
        entry.imm_c_swsp = ((insn.c_uimm8sp_s() & 0x3) << 6) | insn.c_uimm8sp_s() & 0x3c;
        entry.imm_c_sdsp = ((insn.c_uimm9sp_s() & 0x7) << 6) | insn.c_uimm9sp_s() & 0x38;
        entry.imm_c_lw =
            ((insn.c_uimm7lo() & 0x1) << 6) | (insn.c_uimm7hi() << 3) | ((insn.c_uimm7lo() & 0x2) << 1);
        entry.imm_c_ld = (insn.c_uimm8lo() << 6) | (insn.c_uimm8hi() << 3);
        entry.imm_c_sw =
            ((insn.c_uimm7lo() & 0x1) << 6) | (insn.c_uimm7hi() << 3) | ((insn.c_uimm7lo() & 0x2) << 1);
        entry.imm_c_sd = (insn.c_uimm8lo() << 6) | (insn.c_uimm8hi() << 3);
        entry.imm_c_j = Insn::sign_extend(
            ((insn.c_imm12() & 0x400) << 1) | ((insn.c_imm12() & 0x40) << 4) | ((insn.c_imm12() & 0x180) << 1) | ((insn.c_imm12() & 0x10) << 3) | ((insn.c_imm12() & 0x20) << 1)
                | (insn.c_imm12() & 0x200) >> 5
                | (insn.c_imm12() & 0xe)
                | (insn.c_imm12() & 0x1) << 5,
            12,
        ) as u64;
        entry.imm_c_b = Insn::sign_extend(
            ((insn.c_bimm9hi() & 0x4) << 6) | ((insn.c_bimm9lo() & 0x18) << 3) | ((insn.c_bimm9lo() & 0x1) << 5) | ((insn.c_bimm9hi() & 0x3) << 3)
                | insn.c_bimm9lo() & 0x6,
            9,
        ) as u64;
        entry.imm_c_li = Insn::sign_extend((insn.c_imm6hi() << 5) | insn.c_imm6lo(), 6) as u64;
        entry.imm_c_lui =
            Insn::sign_extend((insn.c_nzimm18hi() << 17) | (insn.c_nzimm18lo() << 12), 18) as u64;
        entry.imm_c_addi = Insn::sign_extend((insn.c_nzimm6hi() << 5) | insn.c_nzimm6lo(), 6) as u64;
        entry.imm_c_addi16sp = Insn::sign_extend(
            (insn.c_nzimm10hi() << 9) | ((insn.c_nzimm10lo() & 0x6) << 6) | ((insn.c_nzimm10lo() & 0x8) << 3) | ((insn.c_nzimm10lo() & 0x1) << 5)
                | (insn.c_nzimm10lo()) & 0x10,
            10,
        ) as u64;
        entry.imm_c_addi4spn = ((insn.c_nzuimm10() & 0xc0) >> 2) | ((insn.c_nzuimm10() & 0x3c) << 4) | ((insn.c_nzuimm10() & 0x02) << 1) | ((insn.c_nzuimm10() & 0x01) << 3);
        entry.imm_c_shamt = (insn.c_nzuimm6hi() << 5) | insn.c_nzuimm6lo();
        entry.imm_c_andi = Insn::sign_extend((insn.c_imm6hi() << 5) | insn.c_imm6lo(), 6) as u64;

        /* c registers */
        entry.rs1_p = insn.rs1_p();
        entry.rs2_p = insn.rs2_p();
        entry.rd_p = insn.rd_p();
        entry.rd_rs1_n0 = insn.rd_rs1_n0();
        entry.rd_rs1_p = insn.rd_rs1_p();
        entry.rd_rs1 = insn.rd_rs1();
        entry.rd_n2 = insn.rd_n2();
        entry.rd_n0 = insn.rd_n0();
        entry.rs1_n0 = insn.rs1_n0();
        entry.c_rs2_n0 = insn.c_rs2_n0();
        entry.c_rs1_n0 = insn.c_rs1_n0();
        entry.c_rs2 = insn.c_rs2();
        entry.c_sreg1 = insn.c_sreg1();
        entry.c_sreg2 = insn.c_sreg2();
        entry.c_rlist = insn.c_rlist();

        /* float registers */
        entry.rs3 = insn.rs3();
        entry.rm = insn.rm();

        /* csrs */
        entry.csr = insn.csr();
        entry.zimm = insn.zimm();

        // select operation here
        let bits = insn.bits();
        let jump_table_index: Option<usize> = UopCacheEntry::set_cached_insn(insn.bits());
        if let Some(index) = jump_table_index {
            entry.jump_table_index = index;
            entry.op = jump_table::JUMP_TABLE[index];
            entry.valid = true;
        }
        entry
    }

    #[inline(always)]
    pub fn execute_cached_insn(&self, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
        (self.op)(cpu, bus, self)
    }
}
