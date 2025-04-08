use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn, Result},
    insn_impl::insn_cached,
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
    pub shamtd: u64,
    pub shamtw: u64,
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
            shamtd: (0),
            shamtw: (0),
            op: (insn_cached::nop_cached::nop_cached),
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
        entry.imm_u = Insn::sign_extend(insn.imm20() << 12, 32) as u64;
        entry.imm_j = Insn::sign_extend(
            ((insn.jimm20() & 0x80000) << 1
                | ((insn.jimm20() & 0xff) << 12)
                | ((insn.jimm20() & 0x7fe00) >> 9 << 1)
                | ((insn.jimm20() & 0x100) >> 8 << 11)) as u64,
            20,
        ) as u64;
        entry.shamtd = insn.shamtd();
        entry.shamtw = insn.shamtw();

        // select operation here
        let bits = insn.bits();
        if bits & 0x707f == 0x13 {
            entry.op = insn_cached::addi_cached::addi_cached;
        } else if bits & 0x707f == 0x2013 {
            entry.op = insn_cached::slti_cached::slti_cached;
        } else if bits & 0x707f == 0x3013 {
            entry.op = insn_cached::sltiu_cached::sltiu_cached;
        } else if bits & 0x707f == 0x4013 {
            entry.op = insn_cached::xori_cached::xori_cached;
        } else if bits & 0x707f == 0x6013 {
            entry.op = insn_cached::ori_cached::ori_cached;
        } else if bits & 0x707f == 0x7013 {
            entry.op = insn_cached::andi_cached::andi_cached;
        } else if bits & 0xfe00707f == 0x33 {
            entry.op = insn_cached::add_cached::add_cached;
        } else if bits & 0xfe00707f == 0x40000033 {
            entry.op = insn_cached::sub_cached::sub_cached;
        } else if bits & 0xfe00707f == 0x1033 {
            entry.op = insn_cached::sll_cached::sll_cached;
        } else if bits & 0xfe00707f == 0x2033 {
            entry.op = insn_cached::slt_cached::slt_cached;
        } else if bits & 0xfe00707f == 0x3033 {
            entry.op = insn_cached::sltu_cached::sltu_cached;
        } else if bits & 0xfe00707f == 0x4033 {
            entry.op = insn_cached::xor_cached::xor_cached;
        } else if bits & 0xfe00707f == 0x5033 {
            entry.op = insn_cached::srl_cached::srl_cached;
        } else if bits & 0xfe00707f == 0x40005033 {
            entry.op = insn_cached::sra_cached::sra_cached;
        } else if bits & 0xfe00707f == 0x6033 {
            entry.op = insn_cached::or_cached::or_cached;
        } else if bits & 0xfe00707f == 0x7033 {
            entry.op = insn_cached::and_cached::and_cached;
        } else if bits & 0x707f == 0x3 {
            entry.op = insn_cached::lb_cached::lb_cached;
        } else if bits & 0x707f == 0x1003 {
            entry.op = insn_cached::lh_cached::lh_cached;
        } else if bits & 0x707f == 0x2003 {
            entry.op = insn_cached::lw_cached::lw_cached;
        } else if bits & 0x707f == 0x4003 {
            entry.op = insn_cached::lbu_cached::lbu_cached;
        } else if bits & 0x707f == 0x5003 {
            entry.op = insn_cached::lhu_cached::lhu_cached;
        } else if bits & 0x707f == 0x6003 {
            entry.op = insn_cached::lwu_cached::lwu_cached;
        } else if bits & 0x707f == 0x3003 {
            entry.op = insn_cached::ld_cached::ld_cached;
        } else if bits & 0x707f == 0x23 {
            entry.op = insn_cached::sb_cached::sb_cached;
        } else if bits & 0x707f == 0x1023 {
            entry.op = insn_cached::sh_cached::sh_cached;
        } else if bits & 0x707f == 0x2023 {
            entry.op = insn_cached::sw_cached::sw_cached;
        } else if bits & 0x707f == 0x3023 {
            entry.op = insn_cached::sd_cached::sd_cached;
        } else if bits & 0x707f == 0x63 {
            entry.op = insn_cached::beq_cached::beq_cached;
        } else if bits & 0x707f == 0x1063 {
            entry.op = insn_cached::bne_cached::bne_cached;
        } else if bits & 0x707f == 0x4063 {
            entry.op = insn_cached::blt_cached::blt_cached;
        } else if bits & 0x707f == 0x5063 {
            entry.op = insn_cached::bge_cached::bge_cached;
        } else if bits & 0x707f == 0x6063 {
            entry.op = insn_cached::bltu_cached::bltu_cached;
        } else if bits & 0x707f == 0x7063 {
            entry.op = insn_cached::bgeu_cached::bgeu_cached;
        } else if bits & 0x7f == 0x6f {
            entry.op = insn_cached::jal_cached::jal_cached;
        } else if bits & 0x707f == 0x67 {
            entry.op = insn_cached::jalr_cached::jalr_cached;
        } else if bits & 0x7f == 0x37 {
            entry.op = insn_cached::lui_cached::lui_cached;
        } else if bits & 0x7f == 0x17 {
            entry.op = insn_cached::auipc_cached::auipc_cached;
        } else if bits & 0x707f == 0x1b {
            entry.op = insn_cached::addiw_cached::addiw_cached;
        } else if bits & 0xfe00707f == 0x101b {
            entry.op = insn_cached::slliw_cached::slliw_cached;
        } else if bits & 0xfe00707f == 0x501b {
            entry.op = insn_cached::srliw_cached::srliw_cached;
        } else if bits & 0xfe00707f == 0x4000501b {
            entry.op = insn_cached::sraiw_cached::sraiw_cached;
        } else if bits & 0xfe00707f == 0x3b {
            entry.op = insn_cached::addw_cached::addw_cached;
        } else if bits & 0xfe00707f == 0x4000003b {
            entry.op = insn_cached::subw_cached::subw_cached;
        } else if bits & 0xfe00707f == 0x103b {
            entry.op = insn_cached::sllw_cached::sllw_cached;
        } else if bits & 0xfe00707f == 0x503b {
            entry.op = insn_cached::srlw_cached::srlw_cached;
        } else if bits & 0xfe00707f == 0x4000503b {
            entry.op = insn_cached::sraw_cached::sraw_cached;
        } else {
            return None;
        }

        return Some(entry);
    }

    pub fn execute_cached_insn(&self, cpu: &mut Cpu, bus: &mut Bus) -> cpu::Result<u64> {
        (self.op)(cpu, bus, self)
    }
}
