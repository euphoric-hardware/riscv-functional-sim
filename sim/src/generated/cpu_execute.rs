use crate::{
    cpu::{self, Cpu, Insn},
    bus::Bus,
    insn_impl,
};

impl Cpu {
    pub fn execute_insn(&mut self, insn: Insn, bus: &mut Bus) -> cpu::Result<u64> {
        let bits = insn.bits();
        let result: cpu::Result<u64>;

        if bits & 0xfe00707f == 0x33 {
            result = insn_impl::add::add(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x13 {
            result = insn_impl::addi::addi(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x1b {
            result = insn_impl::addiw::addiw(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x3b {
            result = insn_impl::addw::addw(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x7033 {
            result = insn_impl::and::and(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x7013 {
            result = insn_impl::andi::andi(insn, self, bus);
            
        }

        else if bits & 0x7f == 0x17 {
            result = insn_impl::auipc::auipc(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x63 {
            result = insn_impl::beq::beq(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x5063 {
            result = insn_impl::bge::bge(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x7063 {
            result = insn_impl::bgeu::bgeu(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x4063 {
            result = insn_impl::blt::blt(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x6063 {
            result = insn_impl::bltu::bltu(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x1063 {
            result = insn_impl::bne::bne(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x3073 {
            result = insn_impl::csrrc::csrrc(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x7073 {
            result = insn_impl::csrrci::csrrci(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x2073 {
            result = insn_impl::csrrs::csrrs(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x6073 {
            result = insn_impl::csrrsi::csrrsi(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x1073 {
            result = insn_impl::csrrw::csrrw(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x5073 {
            result = insn_impl::csrrwi::csrrwi(insn, self, bus);
            
        }

        else if bits & 0xffffffff == 0x100073 {
            result = insn_impl::ebreak::ebreak(insn, self, bus);
            
        }

        else if bits & 0xffffffff == 0x73 {
            result = insn_impl::ecall::ecall(insn, self, bus);
            
        }

        else if bits & 0x707f == 0xf {
            result = insn_impl::fence::fence(insn, self, bus);
            
        }

        else if bits & 0xfff0707f == 0x8330000f {
            result = insn_impl::fence_tso::fence_tso(insn, self, bus);
            
        }

        else if bits & 0x7f == 0x6f {
            result = insn_impl::jal::jal(insn, self, bus);
            
        }

        else if bits & 0xfff == 0xef {
            result = insn_impl::jal_pseudo::jal_pseudo(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x67 {
            result = insn_impl::jalr::jalr(insn, self, bus);
            
        }

        else if bits & 0xfff07fff == 0xe7 {
            result = insn_impl::jalr_pseudo::jalr_pseudo(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x3 {
            result = insn_impl::lb::lb(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x4003 {
            result = insn_impl::lbu::lbu(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x3003 {
            result = insn_impl::ld::ld(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x1003 {
            result = insn_impl::lh::lh(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x5003 {
            result = insn_impl::lhu::lhu(insn, self, bus);
            
        }

        else if bits & 0x7f == 0x37 {
            result = insn_impl::lui::lui(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x2003 {
            result = insn_impl::lw::lw(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x6003 {
            result = insn_impl::lwu::lwu(insn, self, bus);
            
        }

        else if bits & 0xffffffff == 0x30200073 {
            result = insn_impl::mret::mret(insn, self, bus);
            
        }

        else if bits & 0xffffffff == 0x30200073 {
            insn_impl::mret::mret(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x6033 {
            result = insn_impl::or::or(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x6013 {
            result = insn_impl::ori::ori(insn, self, bus);
            
        }

        else if bits & 0xffffffff == 0x100000f {
            result = insn_impl::pause::pause(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x23 {
            result = insn_impl::sb::sb(insn, self, bus);
            
        }

        else if bits & 0xffffffff == 0x100073 {
            result = insn_impl::sbreak::sbreak(insn, self, bus);
            
        }

        else if bits & 0xffffffff == 0x73 {
            result = insn_impl::scall::scall(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x3023 {
            result = insn_impl::sd::sd(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x1023 {
            result = insn_impl::sh::sh(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x1033 {
            result = insn_impl::sll::sll(insn, self, bus);
            
        }

        else if bits & 0xfc00707f == 0x1013 {
            result = insn_impl::slli::slli(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x101b {
            result = insn_impl::slliw::slliw(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x103b {
            result = insn_impl::sllw::sllw(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x2033 {
            result = insn_impl::slt::slt(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x2013 {
            result = insn_impl::slti::slti(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x3013 {
            result = insn_impl::sltiu::sltiu(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x3033 {
            result = insn_impl::sltu::sltu(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x40005033 {
            result = insn_impl::sra::sra(insn, self, bus);
            
        }

        else if bits & 0xfc00707f == 0x40005013 {
            result = insn_impl::srai::srai(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x4000501b {
            result = insn_impl::sraiw::sraiw(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x4000503b {
            result = insn_impl::sraw::sraw(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x5033 {
            result = insn_impl::srl::srl(insn, self, bus);
            
        }

        else if bits & 0xfc00707f == 0x5013 {
            result = insn_impl::srli::srli(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x501b {
            result = insn_impl::srliw::srliw(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x503b {
            result = insn_impl::srlw::srlw(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x40000033 {
            result = insn_impl::sub::sub(insn, self, bus);
            
        }

        else if bits & 0xfe00707f == 0x4000003b {
            result = insn_impl::subw::subw(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x2023 {
            result = insn_impl::sw::sw(insn, self, bus);
            
        }

        else if bits & 0xffffffff == 0x10500073 {
            result = insn_impl::wfi::wfi(insn, self, bus);
            
        }

        else if bits & 0xffffffff == 0x10500073 {
            insn_impl::wfi::wfi(insn, self, bus)
        }

        else if bits & 0xfe00707f == 0x4033 {
            result = insn_impl::xor::xor(insn, self, bus);
            
        }

        else if bits & 0x707f == 0x4013 {
            result = insn_impl::xori::xori(insn, self, bus);
            
        }

        else {
            result = Err(cpu::Error::UnknownInsn);
        }
        self.regs[0] = 0;
        return result;
    }
}

