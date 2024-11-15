use crate::{
    cpu::{Cpu, Insn, Instruction},
    insn_impl,
    fpu_inst,
};

impl Cpu {
    pub fn execute(&mut self, insn: Insn) {
        let bits = insn.bits();

        if bits & 0xfe00707f == 0x33 {
            insn_impl::add::add(insn, self);
        }

        if bits & 0x707f == 0x13 {
            insn_impl::addi::addi(insn, self);
        }

        if bits & 0x707f == 0x1b {
            insn_impl::addiw::addiw(insn, self);
        }

        if bits & 0xfe00707f == 0x3b {
            insn_impl::addw::addw(insn, self);
        }

        if bits & 0xfe00707f == 0x7033 {
            insn_impl::and::and(insn, self);
        }

        if bits & 0x707f == 0x7013 {
            insn_impl::andi::andi(insn, self);
        }

        if bits & 0x7f == 0x17 {
            insn_impl::auipc::auipc(insn, self);
        }

        if bits & 0x707f == 0x63 {
            insn_impl::beq::beq(insn, self);
        }

        if bits & 0x1f0707f == 0x63 {
            insn_impl::beqz::beqz(insn, self);
        }

        if bits & 0x707f == 0x5063 {
            insn_impl::bge::bge(insn, self);
        }

        if bits & 0x707f == 0x7063 {
            insn_impl::bgeu::bgeu(insn, self);
        }

        if bits & 0x1f0707f == 0x5063 {
            insn_impl::bgez::bgez(insn, self);
        }

        if bits & 0x707f == 0x4063 {
            insn_impl::bgt::bgt(insn, self);
        }

        if bits & 0x707f == 0x6063 {
            insn_impl::bgtu::bgtu(insn, self);
        }

        if bits & 0xff07f == 0x4063 {
            insn_impl::bgtz::bgtz(insn, self);
        }

        if bits & 0x707f == 0x5063 {
            insn_impl::ble::ble(insn, self);
        }

        if bits & 0x707f == 0x7063 {
            insn_impl::bleu::bleu(insn, self);
        }

        if bits & 0xff07f == 0x5063 {
            insn_impl::blez::blez(insn, self);
        }

        if bits & 0x707f == 0x4063 {
            insn_impl::blt::blt(insn, self);
        }

        if bits & 0x707f == 0x6063 {
            insn_impl::bltu::bltu(insn, self);
        }

        if bits & 0x1f0707f == 0x4063 {
            insn_impl::bltz::bltz(insn, self);
        }

        if bits & 0x707f == 0x1063 {
            insn_impl::bne::bne(insn, self);
        }

        if bits & 0x1f0707f == 0x1063 {
            insn_impl::bnez::bnez(insn, self);
        }

        if bits & 0xffffffff == 0x100073 {
            insn_impl::ebreak::ebreak(insn, self);
        }

        if bits & 0xffffffff == 0x73 {
            insn_impl::ecall::ecall(insn, self);
        }

        if bits & 0x707f == 0xf {
            insn_impl::fence::fence(insn, self);
        }

        if bits & 0xfff0707f == 0x8330000f {
            insn_impl::fence_tso::fence_tso(insn, self);
        }

        if bits & 0xfff == 0x6f {
            insn_impl::j::j(insn, self);
        }

        if bits & 0x7f == 0x6f {
            insn_impl::jal::jal(insn, self);
        }

        if bits & 0xfff == 0xef {
            insn_impl::jal_pseudo::jal_pseudo(insn, self);
        }

        if bits & 0x707f == 0x67 {
            insn_impl::jalr::jalr(insn, self);
        }

        if bits & 0xfff07fff == 0xe7 {
            insn_impl::jalr_pseudo::jalr_pseudo(insn, self);
        }

        if bits & 0xfff07fff == 0x67 {
            insn_impl::jr::jr(insn, self);
        }

        if bits & 0x707f == 0x3 {
            insn_impl::lb::lb(insn, self);
        }

        if bits & 0x707f == 0x4003 {
            insn_impl::lbu::lbu(insn, self);
        }

        if bits & 0x707f == 0x3003 {
            insn_impl::ld::ld(insn, self);
        }

        if bits & 0x707f == 0x1003 {
            insn_impl::lh::lh(insn, self);
        }

        if bits & 0x707f == 0x5003 {
            insn_impl::lhu::lhu(insn, self);
        }

        if bits & 0x7f == 0x37 {
            insn_impl::lui::lui(insn, self);
        }

        if bits & 0x707f == 0x2003 {
            insn_impl::lw::lw(insn, self);
        }

        if bits & 0x707f == 0x6003 {
            insn_impl::lwu::lwu(insn, self);
        }

        if bits & 0xfff0707f == 0x13 {
            insn_impl::mv::mv(insn, self);
        }

        if bits & 0xfff0707f == 0x40000033 {
            insn_impl::neg::neg(insn, self);
        }

        if bits & 0xffffffff == 0x13 {
            insn_impl::nop::nop(insn, self);
        }

        if bits & 0xfe00707f == 0x6033 {
            insn_impl::or::or(insn, self);
        }

        if bits & 0x707f == 0x6013 {
            insn_impl::ori::ori(insn, self);
        }

        if bits & 0xffffffff == 0x100000f {
            insn_impl::pause::pause(insn, self);
        }

        if bits & 0xffffffff == 0x8067 {
            insn_impl::ret::ret(insn, self);
        }

        if bits & 0x707f == 0x23 {
            insn_impl::sb::sb(insn, self);
        }

        if bits & 0xffffffff == 0x100073 {
            insn_impl::sbreak::sbreak(insn, self);
        }

        if bits & 0xffffffff == 0x73 {
            insn_impl::scall::scall(insn, self);
        }

        if bits & 0x707f == 0x3023 {
            insn_impl::sd::sd(insn, self);
        }

        if bits & 0xfff0707f == 0x103013 {
            insn_impl::seqz::seqz(insn, self);
        }

        if bits & 0xfff0707f == 0x1b {
            insn_impl::sext_w::sext_w(insn, self);
        }

        if bits & 0xfe0ff07f == 0x2033 {
            insn_impl::sgtz::sgtz(insn, self);
        }

        if bits & 0x707f == 0x1023 {
            insn_impl::sh::sh(insn, self);
        }

        if bits & 0xfe00707f == 0x1033 {
            insn_impl::sll::sll(insn, self);
        }

        if bits & 0xfc00707f == 0x1013 {
            insn_impl::slli::slli(insn, self);
        }

        if bits & 0xfe00707f == 0x101b {
            insn_impl::slliw::slliw(insn, self);
        }

        if bits & 0xfe00707f == 0x103b {
            insn_impl::sllw::sllw(insn, self);
        }

        if bits & 0xfe00707f == 0x2033 {
            insn_impl::slt::slt(insn, self);
        }

        if bits & 0x707f == 0x2013 {
            insn_impl::slti::slti(insn, self);
        }

        if bits & 0x707f == 0x3013 {
            insn_impl::sltiu::sltiu(insn, self);
        }

        if bits & 0xfe00707f == 0x3033 {
            insn_impl::sltu::sltu(insn, self);
        }

        if bits & 0xfff0707f == 0x2033 {
            insn_impl::sltz::sltz(insn, self);
        }

        if bits & 0xfe0ff07f == 0x3033 {
            insn_impl::snez::snez(insn, self);
        }

        if bits & 0xfe00707f == 0x40005033 {
            insn_impl::sra::sra(insn, self);
        }

        if bits & 0xfc00707f == 0x40005013 {
            insn_impl::srai::srai(insn, self);
        }

        if bits & 0xfe00707f == 0x4000501b {
            insn_impl::sraiw::sraiw(insn, self);
        }

        if bits & 0xfe00707f == 0x4000503b {
            insn_impl::sraw::sraw(insn, self);
        }

        if bits & 0xfe00707f == 0x5033 {
            insn_impl::srl::srl(insn, self);
        }

        if bits & 0xfc00707f == 0x5013 {
            insn_impl::srli::srli(insn, self);
        }

        if bits & 0xfe00707f == 0x501b {
            insn_impl::srliw::srliw(insn, self);
        }

        if bits & 0xfe00707f == 0x503b {
            insn_impl::srlw::srlw(insn, self);
        }

        if bits & 0xfe00707f == 0x40000033 {
            insn_impl::sub::sub(insn, self);
        }

        if bits & 0xfe00707f == 0x4000003b {
            insn_impl::subw::subw(insn, self);
        }

        if bits & 0x707f == 0x2023 {
            insn_impl::sw::sw(insn, self);
        }

        if bits & 0xfe00707f == 0x4033 {
            insn_impl::xor::xor(insn, self);
        }

        if bits & 0x707f == 0x4013 {
            insn_impl::xori::xori(insn, self);
        }

        if bits & 0xfff0707f == 0x7013 {
            insn_impl::zext_b::zext_b(insn, self);
        }

        else {
            panic!("unknown instruction!")
        }
    }

    pub fn execute(&mut self, instruction: Instruction) {
        match instruction.opcode() {
            0b1010011 => self.execute_floating_point(instruction),
            _ => panic!("Unsupported opcode: {:#b}", instruction.opcode()),
        }
    }

    fn execute_floating_point(&mut self, instruction: Instruction) {
        match instruction.funct7() {
            0b0000000 => match instruction.funct3() {
                0b000 => instruction::inst::fadd(instruction, self),
                0b001 => instruction::inst::fsub(instruction, self),
                0b010 => instruction::inst::fmul(instruction, self),
                0b011 => instruction::inst::fdiv(instruction, self),
                _ => panic!("Unsupported funct3: {:#b}", instruction.funct3()),
            },
            0b0000001 => match instruction.funct3() {
                0b000 => instruction::inst::feq(instruction, self),
                0b001 => instruction::inst::flt(instruction, self),
                0b010 => instruction::inst::fle(instruction, self),
                _ => panic!("Unsupported funct3: {:#b}", instruction.funct3()),
            },
            0b0000101 => match instruction.funct3() {
                0b000 => instruction::inst::fsqrt(instruction, self),
                _ => panic!("Unsupported funct3: {:#b}", instruction.funct3()),
            },
            0b0001000 => match instruction.funct3() {
                0b000 => instruction::inst::fcvt_w_s(instruction, self),
                0b001 => instruction::inst::fcvt_s_w(instruction, self),
                _ => panic!("Unsupported funct3: {:#b}", instruction.funct3()),
            },
            0b0010000 => match instruction.funct3() {
                0b000 => instruction::inst::fmadd(instruction, self),   
                0b001 => instruction::inst::fmsub(instruction, self),   
                0b010 => instruction::inst::fnmadd(instruction, self),
                0b011 => instruction::inst::fnmsub(instruction, self),
                _ => panic!("Unsupported funct3: {:#b}", instruction.funct3()),
            },
            _ => panic!("Unsupported funct7: {:#b}", instruction.funct7()),
        }
    }
    
}

