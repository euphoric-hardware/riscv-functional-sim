use crate::{
    bus::Bus,
    cpu::{Cpu, Exception, Insn, Result},
    insn_impl,
};

#[rustfmt::skip]
impl Cpu {
    pub fn execute_insn(&mut self, insn: Insn, bus: &mut Bus) -> Result<u64> {
        let bits = insn.bits();

        if bits & 0x33 == 0xfe00707f { insn_impl::add::add(insn, self, bus) }
        else if bits & 0x13 == 0x707f { insn_impl::addi::addi(insn, self, bus) }
        else if bits & 0x1b == 0x707f { insn_impl::addiw::addiw(insn, self, bus) }
        else if bits & 0x3b == 0xfe00707f { insn_impl::addw::addw(insn, self, bus) }
        else if bits & 0x7033 == 0xfe00707f { insn_impl::and::and(insn, self, bus) }
        else if bits & 0x7013 == 0x707f { insn_impl::andi::andi(insn, self, bus) }
        else if bits & 0x17 == 0x7f { insn_impl::auipc::auipc(insn, self, bus) }
        else if bits & 0x63 == 0x707f { insn_impl::beq::beq(insn, self, bus) }
        else if bits & 0x5063 == 0x707f { insn_impl::bge::bge(insn, self, bus) }
        else if bits & 0x7063 == 0x707f { insn_impl::bgeu::bgeu(insn, self, bus) }
        else if bits & 0x4063 == 0x707f { insn_impl::blt::blt(insn, self, bus) }
        else if bits & 0x6063 == 0x707f { insn_impl::bltu::bltu(insn, self, bus) }
        else if bits & 0x1063 == 0x707f { insn_impl::bne::bne(insn, self, bus) }
        else if bits & 0x3073 == 0x707f { insn_impl::csrrc::csrrc(insn, self, bus) }
        else if bits & 0x7073 == 0x707f { insn_impl::csrrci::csrrci(insn, self, bus) }
        else if bits & 0x2073 == 0x707f { insn_impl::csrrs::csrrs(insn, self, bus) }
        else if bits & 0x6073 == 0x707f { insn_impl::csrrsi::csrrsi(insn, self, bus) }
        else if bits & 0x1073 == 0x707f { insn_impl::csrrw::csrrw(insn, self, bus) }
        else if bits & 0x5073 == 0x707f { insn_impl::csrrwi::csrrwi(insn, self, bus) }
        else if bits & 0x100073 == 0xffffffff { insn_impl::ebreak::ebreak(insn, self, bus) }
        else if bits & 0x73 == 0xffffffff { insn_impl::ecall::ecall(insn, self, bus) }
        else if bits & 0xf == 0x707f { insn_impl::fence::fence(insn, self, bus) }
        else if bits & 0x6f == 0x7f { insn_impl::jal::jal(insn, self, bus) }
        else if bits & 0x67 == 0x707f { insn_impl::jalr::jalr(insn, self, bus) }
        else if bits & 0x3 == 0x707f { insn_impl::lb::lb(insn, self, bus) }
        else if bits & 0x4003 == 0x707f { insn_impl::lbu::lbu(insn, self, bus) }
        else if bits & 0x3003 == 0x707f { insn_impl::ld::ld(insn, self, bus) }
        else if bits & 0x1003 == 0x707f { insn_impl::lh::lh(insn, self, bus) }
        else if bits & 0x5003 == 0x707f { insn_impl::lhu::lhu(insn, self, bus) }
        else if bits & 0x37 == 0x7f { insn_impl::lui::lui(insn, self, bus) }
        else if bits & 0x2003 == 0x707f { insn_impl::lw::lw(insn, self, bus) }
        else if bits & 0x6003 == 0x707f { insn_impl::lwu::lwu(insn, self, bus) }
        else if bits & 0x30200073 == 0xffffffff { insn_impl::mret::mret(insn, self, bus) }
        else if bits & 0x6033 == 0xfe00707f { insn_impl::or::or(insn, self, bus) }
        else if bits & 0x6013 == 0x707f { insn_impl::ori::ori(insn, self, bus) }
        else if bits & 0x23 == 0x707f { insn_impl::sb::sb(insn, self, bus) }
        else if bits & 0x3023 == 0x707f { insn_impl::sd::sd(insn, self, bus) }
        else if bits & 0x1023 == 0x707f { insn_impl::sh::sh(insn, self, bus) }
        else if bits & 0x1033 == 0xfe00707f { insn_impl::sll::sll(insn, self, bus) }
        else if bits & 0x1013 == 0xfc00707f { insn_impl::slli::slli(insn, self, bus) }
        else if bits & 0x101b == 0xfe00707f { insn_impl::slliw::slliw(insn, self, bus) }
        else if bits & 0x103b == 0xfe00707f { insn_impl::sllw::sllw(insn, self, bus) }
        else if bits & 0x2033 == 0xfe00707f { insn_impl::slt::slt(insn, self, bus) }
        else if bits & 0x2013 == 0x707f { insn_impl::slti::slti(insn, self, bus) }
        else if bits & 0x3013 == 0x707f { insn_impl::sltiu::sltiu(insn, self, bus) }
        else if bits & 0x3033 == 0xfe00707f { insn_impl::sltu::sltu(insn, self, bus) }
        else if bits & 0x40005033 == 0xfe00707f { insn_impl::sra::sra(insn, self, bus) }
        else if bits & 0x40005013 == 0xfc00707f { insn_impl::srai::srai(insn, self, bus) }
        else if bits & 0x4000501b == 0xfe00707f { insn_impl::sraiw::sraiw(insn, self, bus) }
        else if bits & 0x4000503b == 0xfe00707f { insn_impl::sraw::sraw(insn, self, bus) }
        else if bits & 0x5033 == 0xfe00707f { insn_impl::srl::srl(insn, self, bus) }
        else if bits & 0x5013 == 0xfc00707f { insn_impl::srli::srli(insn, self, bus) }
        else if bits & 0x501b == 0xfe00707f { insn_impl::srliw::srliw(insn, self, bus) }
        else if bits & 0x503b == 0xfe00707f { insn_impl::srlw::srlw(insn, self, bus) }
        else if bits & 0x40000033 == 0xfe00707f { insn_impl::sub::sub(insn, self, bus) }
        else if bits & 0x4000003b == 0xfe00707f { insn_impl::subw::subw(insn, self, bus) }
        else if bits & 0x2023 == 0x707f { insn_impl::sw::sw(insn, self, bus) }
        else if bits & 0x10500073 == 0xffffffff { insn_impl::wfi::wfi(insn, self, bus) }
        else if bits & 0x4033 == 0xfe00707f { insn_impl::xor::xor(insn, self, bus) }
        else if bits & 0x4013 == 0x707f { insn_impl::xori::xori(insn, self, bus) }

        else { Err(Exception::IllegalInstruction) }
    }
}
