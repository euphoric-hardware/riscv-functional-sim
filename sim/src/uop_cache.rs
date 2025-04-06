use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn, Result},
    insn_impl::insn_raw::{self, add_cached},
};

#[derive(Debug, Clone)]
pub struct UopCacheEntry {
    pub rs1: u64,
    pub rs2: u64,
    pub rd: u64,
    pub imm_i: u64,
    pub imm_s: u64,
    pub imm_b: u64,
    pub imm_u: u64,
    pub imm_j: u64,
    pub op: fn(cpu: &mut Cpu, bus: &mut Bus, &UopCacheEntry) -> cpu::Result<u64>,
}

impl UopCacheEntry {
    pub fn new(insn: Insn) -> Option<Self> {
        let mut entry = UopCacheEntry {
            rs1: (0),
            rs2: (0),
            rd: (0),
            imm_i: (0),
            imm_s: (0),
            imm_b: (0),
            imm_u: (0),
            imm_j: (0),
            op: (insn_raw::nop_cached::nop_cached),
        };
        entry.rs1 = insn.rs1();
        entry.rs2 = insn.rs2();
        entry.rd = insn.rd();
        entry.imm_i = insn.imm12();
        entry.imm_s = Insn::sign_extend(insn.imm12hi() << 5 | insn.imm12lo(), 12) as u64;
        entry.imm_b = Insn::sign_extend(
            ((insn.bimm12hi() & 0x40) << 6)
                | ((insn.bimm12lo() & 0x01) << 11)
                | ((insn.bimm12hi() & 0x3F) << 5)
                | (insn.bimm12lo() & 0x1E),
            13,
        ) as u64;
        entry.imm_u = insn.imm20() << 12;
        entry.imm_j = Insn::sign_extend(
            ((insn.jimm20() & 0x80000) << 1
                | ((insn.jimm20() & 0xff) << 12)
                | ((insn.jimm20() & 0x7fe00) >> 9 << 1)
                | ((insn.jimm20() & 0x100) >> 8 << 11)) as u64,
            20,
        ) as u64;

        // select operation here
        let bits = insn.bits();
        if bits & 0x707f == 0x13 {
            entry.op = insn_raw::addi_cached::addi_cached;
        } else if bits & 0x707f == 0x2013 {
            entry.op = insn_raw::slti_cached::slti_cached;
        } else if bits & 0x707f == 0x3013 {
            entry.op = insn_raw::sltiu_cached::sltiu_cached;
        } else if bits & 0x707f == 0x4013 {
            entry.op = insn_raw::xori_cached::xori_cached;
        } else if bits & 0x707f == 0x6013 {
            entry.op = insn_raw::ori_cached::ori_cached;
        } else if bits & 0x707f == 0x7013 {
            entry.op = insn_raw::andi_cached::andi_cached;
        } else if bits & 0xfe00707f == 0x33 {
            entry.op = insn_raw::add_cached::add_cached;
        } else if bits & 0xfe00707f == 0x40000033 {
            entry.op = insn_raw::sub_cached::sub_cached;
        } else if bits & 0xfe00707f == 0x1033 {
            entry.op = insn_raw::sll_cached::sll_cached;
        } else if bits & 0xfe00707f == 0x2033 {
            entry.op = insn_raw::slt_cached::slt_cached;
        } else if bits & 0xfe00707f == 0x3033 {
            entry.op = insn_raw::sltu_cached::sltu_cached;
        } else if bits & 0xfe00707f == 0x4033 {
            entry.op = insn_raw::xor_cached::xor_cached;
        } else if bits & 0xfe00707f == 0x5033 {
            entry.op = insn_raw::srl_cached::srl_cached;
        } else if bits & 0xfe00707f == 0x40005033 {
            entry.op = insn_raw::sra_cached::sra_cached;
        } else if bits & 0xfe00707f == 0x6033 {
            entry.op = insn_raw::or_cached::or_cached;
        } else if bits & 0xfe00707f == 0x7033 {
            entry.op = insn_raw::and_cached::and_cached;
        } else if bits & 0x707f == 0x3 {
            entry.op = insn_raw::lb_cached::lb_cached;
        } else if bits & 0x707f == 0x1003 {
            entry.op = insn_raw::lh_cached::lh_cached;
        } else if bits & 0x707f == 0x2003 {
            entry.op = insn_raw::lw_cached::lw_cached;
        } else if bits & 0x707f == 0x4003 {
            entry.op = insn_raw::lbu_cached::lbu_cached;
        } else if bits & 0x707f == 0x5003 {
            entry.op = insn_raw::lhu_cached::lhu_cached;
        } else if bits & 0x707f == 0x23 {
            entry.op = insn_raw::sb_cached::sb_cached;
        } else if bits & 0x707f == 0x1023 {
            entry.op = insn_raw::sh_cached::sh_cached;
        } else if bits & 0x707f == 0x2023 {
            entry.op = insn_raw::sw_cached::sw_cached;
        } else {
            return None;
        }

        return Some(entry);
    }

    pub fn execute_cached_insn(&self, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
        (self.op)(cpu, bus, self)
    }
}
