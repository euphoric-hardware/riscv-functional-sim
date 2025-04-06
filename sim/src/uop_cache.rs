use crate::{
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
    pub op: fn(cpu: &mut Cpu, &UopCacheEntry) -> cpu::Result<u64>,
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
        } else {
            return None;
        }

        return Some(entry);
    }

    pub fn execute_cached_insn(&self, cpu: &mut Cpu) -> cpu::Result<u64> {
        (self.op)(cpu, self)
    }
}
