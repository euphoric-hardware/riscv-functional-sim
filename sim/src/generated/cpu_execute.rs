use crate::{
    cpu::{self, Cpu, Insn},
    bus::Bus,
    insn_impl,
};

impl Cpu {
    pub fn execute_insn(&mut self, insn: Insn, bus: &mut Bus) -> cpu::Result<u64> {
        let bits = insn.bits();

        if bits & 0xfe00707f == 0x33 {
            insn_impl::add::add(insn, self, bus)
        }

        else if bits & 0x707f == 0x13 {
            insn_impl::addi::addi(insn, self, bus)
        }

        else if bits & 0x707f == 0x1b {
            insn_impl::addiw::addiw(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x3b {
            insn_impl::addw::addw(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x7033 {
            insn_impl::and::and(insn, self, bus)
        }

        else if bits & 0x707f == 0x7013 {
            insn_impl::andi::andi(insn, self, bus)
        }

        else if bits & 0x7f == 0x17 {
            insn_impl::auipc::auipc(insn, self, bus)
        }

        else if bits & 0x707f == 0x63 {
            insn_impl::beq::beq(insn, self, bus)
        }

        else if bits & 0x707f == 0x5063 {
            insn_impl::bge::bge(insn, self, bus)
        }

        else if bits & 0x707f == 0x7063 {
            insn_impl::bgeu::bgeu(insn, self, bus)
        }

        else if bits & 0x707f == 0x4063 {
            insn_impl::blt::blt(insn, self, bus)
        }

        else if bits & 0x707f == 0x6063 {
            insn_impl::bltu::bltu(insn, self, bus)
        }

        else if bits & 0x707f == 0x1063 {
            insn_impl::bne::bne(insn, self, bus)
        }

        else if bits & 0x707f == 0x3073 {
            insn_impl::csrrc::csrrc(insn, self, bus)
        }

        else if bits & 0x707f == 0x7073 {
            insn_impl::csrrci::csrrci(insn, self, bus)
        }

        else if bits & 0x707f == 0x2073 {
            insn_impl::csrrs::csrrs(insn, self, bus)
        }

        else if bits & 0x707f == 0x6073 {
            insn_impl::csrrsi::csrrsi(insn, self, bus)
        }

        else if bits & 0x707f == 0x1073 {
            insn_impl::csrrw::csrrw(insn, self, bus)
        }

        else if bits & 0x707f == 0x5073 {
            insn_impl::csrrwi::csrrwi(insn, self, bus)
        }

        else if bits & 0xffffffff == 0x100073 {
            insn_impl::ebreak::ebreak(insn, self, bus)
        }

        else if bits & 0xffffffff == 0x73 {
            insn_impl::ecall::ecall(insn, self, bus)
        }

        else if bits & 0x707f == 0xf {
            insn_impl::fence::fence(insn, self, bus)
        }

        else if bits & 0xfff0707f == 0x8330000f {
            insn_impl::fence_tso::fence_tso(insn, self, bus)
        }

        else if bits & 0x7f == 0x6f {
            insn_impl::jal::jal(insn, self, bus)
        }

        else if bits & 0xfff == 0xef {
            insn_impl::jal_pseudo::jal_pseudo(insn, self, bus)
        }

        else if bits & 0x707f == 0x67 {
            insn_impl::jalr::jalr(insn, self, bus)
        }

        else if bits & 0xfff07fff == 0xe7 {
            insn_impl::jalr_pseudo::jalr_pseudo(insn, self, bus)
        }

        else if bits & 0x707f == 0x3 {
            insn_impl::lb::lb(insn, self, bus)
        }

        else if bits & 0x707f == 0x4003 {
            insn_impl::lbu::lbu(insn, self, bus)
        }

        else if bits & 0x707f == 0x3003 {
            insn_impl::ld::ld(insn, self, bus)
        }

        else if bits & 0x707f == 0x1003 {
            insn_impl::lh::lh(insn, self, bus)
        }

        else if bits & 0x707f == 0x5003 {
            insn_impl::lhu::lhu(insn, self, bus)
        }

        else if bits & 0x7f == 0x37 {
            insn_impl::lui::lui(insn, self, bus)
        }

        else if bits & 0x707f == 0x2003 {
            insn_impl::lw::lw(insn, self, bus)
        }

        else if bits & 0x707f == 0x6003 {
            insn_impl::lwu::lwu(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x6033 {
            insn_impl::or::or(insn, self, bus)
        }

        else if bits & 0x707f == 0x6013 {
            insn_impl::ori::ori(insn, self, bus)
        }

        else if bits & 0xffffffff == 0x100000f {
            insn_impl::pause::pause(insn, self, bus)
        }

        else if bits & 0x707f == 0x23 {
            insn_impl::sb::sb(insn, self, bus)
        }

        else if bits & 0xffffffff == 0x100073 {
            insn_impl::sbreak::sbreak(insn, self, bus)
        }

        else if bits & 0xffffffff == 0x73 {
            insn_impl::scall::scall(insn, self, bus)
        }

        else if bits & 0x707f == 0x3023 {
            insn_impl::sd::sd(insn, self, bus)
        }

        else if bits & 0x707f == 0x1023 {
            insn_impl::sh::sh(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x1033 {
            insn_impl::sll::sll(insn, self, bus)
        }

        else if bits & 0xfc00707f == 0x1013 {
            insn_impl::slli::slli(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x101b {
            insn_impl::slliw::slliw(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x103b {
            insn_impl::sllw::sllw(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x2033 {
            insn_impl::slt::slt(insn, self, bus)
        }

        else if bits & 0x707f == 0x2013 {
            insn_impl::slti::slti(insn, self, bus)
        }

        else if bits & 0x707f == 0x3013 {
            insn_impl::sltiu::sltiu(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x3033 {
            insn_impl::sltu::sltu(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x40005033 {
            insn_impl::sra::sra(insn, self, bus)
        }

        else if bits & 0xfc00707f == 0x40005013 {
            insn_impl::srai::srai(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x4000501b {
            insn_impl::sraiw::sraiw(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x4000503b {
            insn_impl::sraw::sraw(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x5033 {
            insn_impl::srl::srl(insn, self, bus)
        }

        else if bits & 0xfc00707f == 0x5013 {
            insn_impl::srli::srli(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x501b {
            insn_impl::srliw::srliw(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x503b {
            insn_impl::srlw::srlw(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x40000033 {
            insn_impl::sub::sub(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x4000003b {
            insn_impl::subw::subw(insn, self, bus)
        }

        else if bits & 0x707f == 0x2023 {
            insn_impl::sw::sw(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x4033 {
            insn_impl::xor::xor(insn, self, bus)
        }

        else if bits & 0x707f == 0x4013 {
            insn_impl::xori::xori(insn, self, bus)
        }

        else {
            Err(cpu::Error::UnknownInsn)
        }
    }
}

