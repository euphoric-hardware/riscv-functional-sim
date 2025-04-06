use crate::{
    bus::Bus,
    cpu::{self, Cpu, Insn},
    insn_impl,
};

impl Cpu {
    pub fn execute_insn(&mut self, insn: Insn, bus: &mut Bus) -> cpu::Result<u64> {
        let cache_index = (self.pc.clone() - 0x80000000) / 4;
        let cache_entry = self.uop_cache.get(cache_index as usize).cloned();
        if let Some(cached_insn) = cache_entry {
            let result = cached_insn.execute_cached_insn(self);
            return result;
        } else {
            let bits = insn.bits();
            if bits & 0x7f == 0x37 {
                insn_impl::lui::lui(insn, self, bus)
            } else if bits & 0x7f == 0x17 {
                insn_impl::auipc::auipc(insn, self, bus)
            } else if bits & 0x7f == 0x6f {
                insn_impl::jal::jal(insn, self, bus)
            } else if bits & 0x707f == 0x67 {
                insn_impl::jalr::jalr(insn, self, bus)
            } else if bits & 0x707f == 0x63 {
                insn_impl::beq::beq(insn, self, bus)
            } else if bits & 0x707f == 0x1063 {
                insn_impl::bne::bne(insn, self, bus)
            } else if bits & 0x707f == 0x4063 {
                insn_impl::blt::blt(insn, self, bus)
            } else if bits & 0x707f == 0x5063 {
                insn_impl::bge::bge(insn, self, bus)
            } else if bits & 0x707f == 0x6063 {
                insn_impl::bltu::bltu(insn, self, bus)
            } else if bits & 0x707f == 0x7063 {
                insn_impl::bgeu::bgeu(insn, self, bus)
            } else if bits & 0x707f == 0x3 {
                insn_impl::lb::lb(insn, self, bus)
            } else if bits & 0x707f == 0x1003 {
                insn_impl::lh::lh(insn, self, bus)
            } else if bits & 0x707f == 0x2003 {
                insn_impl::lw::lw(insn, self, bus)
            } else if bits & 0x707f == 0x4003 {
                insn_impl::lbu::lbu(insn, self, bus)
            } else if bits & 0x707f == 0x5003 {
                insn_impl::lhu::lhu(insn, self, bus)
            } else if bits & 0x707f == 0x23 {
                insn_impl::sb::sb(insn, self, bus)
            } else if bits & 0x707f == 0x1023 {
                insn_impl::sh::sh(insn, self, bus)
            } else if bits & 0x707f == 0x2023 {
                insn_impl::sw::sw(insn, self, bus)
            } else if bits & 0x707f == 0x13 {
                insn_impl::addi::addi(insn, self, bus)
            } else if bits & 0x707f == 0x2013 {
                insn_impl::slti::slti(insn, self, bus)
            } else if bits & 0x707f == 0x3013 {
                insn_impl::sltiu::sltiu(insn, self, bus)
            } else if bits & 0x707f == 0x4013 {
                insn_impl::xori::xori(insn, self, bus)
            } else if bits & 0x707f == 0x6013 {
                insn_impl::ori::ori(insn, self, bus)
            } else if bits & 0x707f == 0x7013 {
                insn_impl::andi::andi(insn, self, bus)
            } else if bits & 0xfe00707f == 0x33 {
                insn_impl::add::add(insn, self, bus)
            } else if bits & 0xfe00707f == 0x40000033 {
                insn_impl::sub::sub(insn, self, bus)
            } else if bits & 0xfe00707f == 0x1033 {
                insn_impl::sll::sll(insn, self, bus)
            } else if bits & 0xfe00707f == 0x2033 {
                insn_impl::slt::slt(insn, self, bus)
            } else if bits & 0xfe00707f == 0x3033 {
                insn_impl::sltu::sltu(insn, self, bus)
            } else if bits & 0xfe00707f == 0x4033 {
                insn_impl::xor::xor(insn, self, bus)
            } else if bits & 0xfe00707f == 0x5033 {
                insn_impl::srl::srl(insn, self, bus)
            } else if bits & 0xfe00707f == 0x40005033 {
                insn_impl::sra::sra(insn, self, bus)
            } else if bits & 0xfe00707f == 0x6033 {
                insn_impl::or::or(insn, self, bus)
            } else if bits & 0xfe00707f == 0x7033 {
                insn_impl::and::and(insn, self, bus)
            } else if bits & 0x707f == 0xf {
                insn_impl::fence::fence(insn, self, bus)
            } else if bits & 0xffffffff == 0x73 {
                insn_impl::ecall::ecall(insn, self, bus)
            } else if bits & 0xffffffff == 0x100073 {
                insn_impl::ebreak::ebreak(insn, self, bus)
            } else if bits & 0x707f == 0x6003 {
                insn_impl::lwu::lwu(insn, self, bus)
            } else if bits & 0x707f == 0x3003 {
                insn_impl::ld::ld(insn, self, bus)
            } else if bits & 0x707f == 0x3023 {
                insn_impl::sd::sd(insn, self, bus)
            } else if bits & 0xfc00707f == 0x1013 {
                insn_impl::slli::slli(insn, self, bus)
            } else if bits & 0xfc00707f == 0x5013 {
                insn_impl::srli::srli(insn, self, bus)
            } else if bits & 0xfc00707f == 0x40005013 {
                insn_impl::srai::srai(insn, self, bus)
            } else if bits & 0x707f == 0x1b {
                insn_impl::addiw::addiw(insn, self, bus)
            } else if bits & 0xfe00707f == 0x101b {
                insn_impl::slliw::slliw(insn, self, bus)
            } else if bits & 0xfe00707f == 0x501b {
                insn_impl::srliw::srliw(insn, self, bus)
            } else if bits & 0xfe00707f == 0x4000501b {
                insn_impl::sraiw::sraiw(insn, self, bus)
            } else if bits & 0xfe00707f == 0x3b {
                insn_impl::addw::addw(insn, self, bus)
            } else if bits & 0xfe00707f == 0x4000003b {
                insn_impl::subw::subw(insn, self, bus)
            } else if bits & 0xfe00707f == 0x103b {
                insn_impl::sllw::sllw(insn, self, bus)
            } else if bits & 0xfe00707f == 0x503b {
                insn_impl::srlw::srlw(insn, self, bus)
            } else if bits & 0xfe00707f == 0x4000503b {
                insn_impl::sraw::sraw(insn, self, bus)
            } else if bits & 0x707f == 0x1073 {
                insn_impl::csrrw::csrrw(insn, self, bus)
            } else if bits & 0x707f == 0x2073 {
                insn_impl::csrrs::csrrs(insn, self, bus)
            } else if bits & 0x707f == 0x3073 {
                insn_impl::csrrc::csrrc(insn, self, bus)
            } else if bits & 0x707f == 0x5073 {
                insn_impl::csrrwi::csrrwi(insn, self, bus)
            } else if bits & 0x707f == 0x6073 {
                insn_impl::csrrsi::csrrsi(insn, self, bus)
            } else if bits & 0x707f == 0x7073 {
                insn_impl::csrrci::csrrci(insn, self, bus)
            } else if bits & 0xffffffff == 0x30200073 {
                insn_impl::mret::mret(insn, self, bus)
            } else if bits & 0xffffffff == 0x10500073 {
                insn_impl::wfi::wfi(insn, self, bus)
            } else if bits & 0xe003 == 0x0 {
                insn_impl::c_addi4spn::c_addi4spn(insn, self, bus)
            } else if bits & 0xe003 == 0x4000 {
                insn_impl::c_lw::c_lw(insn, self, bus)
            } else if bits & 0xe003 == 0xc000 {
                insn_impl::c_sw::c_sw(insn, self, bus)
            } else if bits & 0xef83 == 0x1 {
                insn_impl::c_nop::c_nop(insn, self, bus)
            } else if bits & 0xe003 == 0x1 {
                insn_impl::c_addi::c_addi(insn, self, bus)
            } else if bits & 0xe003 == 0x4001 {
                insn_impl::c_li::c_li(insn, self, bus)
            } else if bits & 0xef83 == 0x6101 {
                insn_impl::c_addi16sp::c_addi16sp(insn, self, bus)
            } else if bits & 0xe003 == 0x6001 {
                insn_impl::c_lui::c_lui(insn, self, bus)
            } else if bits & 0xec03 == 0x8801 {
                insn_impl::c_andi::c_andi(insn, self, bus)
            } else if bits & 0xfc63 == 0x8c01 {
                insn_impl::c_sub::c_sub(insn, self, bus)
            } else if bits & 0xfc63 == 0x8c21 {
                insn_impl::c_xor::c_xor(insn, self, bus)
            } else if bits & 0xfc63 == 0x8c41 {
                insn_impl::c_or::c_or(insn, self, bus)
            } else if bits & 0xfc63 == 0x8c61 {
                insn_impl::c_and::c_and(insn, self, bus)
            } else if bits & 0xe003 == 0xa001 {
                insn_impl::c_j::c_j(insn, self, bus)
            } else if bits & 0xe003 == 0xc001 {
                insn_impl::c_beqz::c_beqz(insn, self, bus)
            } else if bits & 0xe003 == 0xe001 {
                insn_impl::c_bnez::c_bnez(insn, self, bus)
            } else if bits & 0xe003 == 0x4002 {
                insn_impl::c_lwsp::c_lwsp(insn, self, bus)
            } else if bits & 0xf07f == 0x8002 {
                insn_impl::c_jr::c_jr(insn, self, bus)
            } else if bits & 0xf003 == 0x8002 {
                insn_impl::c_mv::c_mv(insn, self, bus)
            } else if bits & 0xffff == 0x9002 {
                insn_impl::c_ebreak::c_ebreak(insn, self, bus)
            } else if bits & 0xf07f == 0x9002 {
                insn_impl::c_jalr::c_jalr(insn, self, bus)
            } else if bits & 0xf003 == 0x9002 {
                insn_impl::c_add::c_add(insn, self, bus)
            } else if bits & 0xe003 == 0xc002 {
                insn_impl::c_swsp::c_swsp(insn, self, bus)
            } else if bits & 0xe003 == 0x6000 {
                insn_impl::c_ld::c_ld(insn, self, bus)
            } else if bits & 0xe003 == 0xe000 {
                insn_impl::c_sd::c_sd(insn, self, bus)
            } else if bits & 0xe003 == 0x2001 {
                insn_impl::c_addiw::c_addiw(insn, self, bus)
            } else if bits & 0xec03 == 0x8001 {
                insn_impl::c_srli::c_srli(insn, self, bus)
            } else if bits & 0xec03 == 0x8401 {
                insn_impl::c_srai::c_srai(insn, self, bus)
            } else if bits & 0xfc63 == 0x9c01 {
                insn_impl::c_subw::c_subw(insn, self, bus)
            } else if bits & 0xfc63 == 0x9c21 {
                insn_impl::c_addw::c_addw(insn, self, bus)
            } else if bits & 0xe003 == 0x2 {
                insn_impl::c_slli::c_slli(insn, self, bus)
            } else if bits & 0xe003 == 0x6002 {
                insn_impl::c_ldsp::c_ldsp(insn, self, bus)
            } else if bits & 0xe003 == 0xe002 {
                insn_impl::c_sdsp::c_sdsp(insn, self, bus)
            } else if bits & 0x707f == 0x2007 {
                insn_impl::flw::flw(insn, self, bus)
            } else if bits & 0x707f == 0x2027 {
                insn_impl::fsw::fsw(insn, self, bus)
            } else if bits & 0x600007f == 0x43 {
                insn_impl::fmadd_s::fmadd_s(insn, self, bus)
            } else if bits & 0x600007f == 0x47 {
                insn_impl::fmsub_s::fmsub_s(insn, self, bus)
            } else if bits & 0x600007f == 0x4b {
                insn_impl::fnmsub_s::fnmsub_s(insn, self, bus)
            } else if bits & 0x600007f == 0x4f {
                insn_impl::fnmadd_s::fnmadd_s(insn, self, bus)
            } else if bits & 0xfe00007f == 0x53 {
                insn_impl::fadd_s::fadd_s(insn, self, bus)
            } else if bits & 0xfe00007f == 0x8000053 {
                insn_impl::fsub_s::fsub_s(insn, self, bus)
            } else if bits & 0xfe00007f == 0x10000053 {
                insn_impl::fmul_s::fmul_s(insn, self, bus)
            } else if bits & 0xfe00007f == 0x18000053 {
                insn_impl::fdiv_s::fdiv_s(insn, self, bus)
            } else if bits & 0xfff0007f == 0x58000053 {
                insn_impl::fsqrt_s::fsqrt_s(insn, self, bus)
            } else if bits & 0xfe00707f == 0x20000053 {
                insn_impl::fsgnj_s::fsgnj_s(insn, self, bus)
            } else if bits & 0xfe00707f == 0x20001053 {
                insn_impl::fsgnjn_s::fsgnjn_s(insn, self, bus)
            } else if bits & 0xfe00707f == 0x20002053 {
                insn_impl::fsgnjx_s::fsgnjx_s(insn, self, bus)
            } else if bits & 0xfe00707f == 0x28000053 {
                insn_impl::fmin_s::fmin_s(insn, self, bus)
            } else if bits & 0xfe00707f == 0x28001053 {
                insn_impl::fmax_s::fmax_s(insn, self, bus)
            } else if bits & 0xfff0007f == 0xc0000053 {
                insn_impl::fcvt_w_s::fcvt_w_s(insn, self, bus)
            } else if bits & 0xfff0007f == 0xc0100053 {
                insn_impl::fcvt_wu_s::fcvt_wu_s(insn, self, bus)
            } else if bits & 0xfff0707f == 0xe0000053 {
                insn_impl::fmv_x_w::fmv_x_w(insn, self, bus)
            } else if bits & 0xfe00707f == 0xa0002053 {
                insn_impl::feq_s::feq_s(insn, self, bus)
            } else if bits & 0xfe00707f == 0xa0001053 {
                insn_impl::flt_s::flt_s(insn, self, bus)
            } else if bits & 0xfe00707f == 0xa0000053 {
                insn_impl::fle_s::fle_s(insn, self, bus)
            } else if bits & 0xfff0707f == 0xe0001053 {
                insn_impl::fclass_s::fclass_s(insn, self, bus)
            } else if bits & 0xfff0007f == 0xd0000053 {
                insn_impl::fcvt_s_w::fcvt_s_w(insn, self, bus)
            } else if bits & 0xfff0007f == 0xd0100053 {
                insn_impl::fcvt_s_wu::fcvt_s_wu(insn, self, bus)
            } else if bits & 0xfff0707f == 0xf0000053 {
                insn_impl::fmv_w_x::fmv_w_x(insn, self, bus)
            } else if bits & 0xfff0007f == 0xc0200053 {
                insn_impl::fcvt_l_s::fcvt_l_s(insn, self, bus)
            } else if bits & 0xfff0007f == 0xc0300053 {
                insn_impl::fcvt_lu_s::fcvt_lu_s(insn, self, bus)
            } else if bits & 0xfff0007f == 0xd0200053 {
                insn_impl::fcvt_s_l::fcvt_s_l(insn, self, bus)
            } else if bits & 0xfff0007f == 0xd0300053 {
                insn_impl::fcvt_s_lu::fcvt_s_lu(insn, self, bus)
            } else if bits & 0x707f == 0x3007 {
                insn_impl::fld::fld(insn, self, bus)
            } else if bits & 0x707f == 0x3027 {
                insn_impl::fsd::fsd(insn, self, bus)
            } else if bits & 0x600007f == 0x2000043 {
                insn_impl::fmadd_d::fmadd_d(insn, self, bus)
            } else if bits & 0x600007f == 0x2000047 {
                insn_impl::fmsub_d::fmsub_d(insn, self, bus)
            } else if bits & 0x600007f == 0x200004b {
                insn_impl::fnmsub_d::fnmsub_d(insn, self, bus)
            } else if bits & 0x600007f == 0x200004f {
                insn_impl::fnmadd_d::fnmadd_d(insn, self, bus)
            } else if bits & 0xfe00007f == 0x2000053 {
                insn_impl::fadd_d::fadd_d(insn, self, bus)
            } else if bits & 0xfe00007f == 0xa000053 {
                insn_impl::fsub_d::fsub_d(insn, self, bus)
            } else if bits & 0xfe00007f == 0x12000053 {
                insn_impl::fmul_d::fmul_d(insn, self, bus)
            } else if bits & 0xfe00007f == 0x1a000053 {
                insn_impl::fdiv_d::fdiv_d(insn, self, bus)
            } else if bits & 0xfff0007f == 0x5a000053 {
                insn_impl::fsqrt_d::fsqrt_d(insn, self, bus)
            } else if bits & 0xfe00707f == 0x22000053 {
                insn_impl::fsgnj_d::fsgnj_d(insn, self, bus)
            } else if bits & 0xfe00707f == 0x22001053 {
                insn_impl::fsgnjn_d::fsgnjn_d(insn, self, bus)
            } else if bits & 0xfe00707f == 0x22002053 {
                insn_impl::fsgnjx_d::fsgnjx_d(insn, self, bus)
            } else if bits & 0xfe00707f == 0x2a000053 {
                insn_impl::fmin_d::fmin_d(insn, self, bus)
            } else if bits & 0xfe00707f == 0x2a001053 {
                insn_impl::fmax_d::fmax_d(insn, self, bus)
            } else if bits & 0xfff0007f == 0x40100053 {
                insn_impl::fcvt_s_d::fcvt_s_d(insn, self, bus)
            } else if bits & 0xfff0007f == 0x42000053 {
                insn_impl::fcvt_d_s::fcvt_d_s(insn, self, bus)
            } else if bits & 0xfe00707f == 0xa2002053 {
                insn_impl::feq_d::feq_d(insn, self, bus)
            } else if bits & 0xfe00707f == 0xa2001053 {
                insn_impl::flt_d::flt_d(insn, self, bus)
            } else if bits & 0xfe00707f == 0xa2000053 {
                insn_impl::fle_d::fle_d(insn, self, bus)
            } else if bits & 0xfff0707f == 0xe2001053 {
                insn_impl::fclass_d::fclass_d(insn, self, bus)
            } else if bits & 0xfff0007f == 0xc2000053 {
                insn_impl::fcvt_w_d::fcvt_w_d(insn, self, bus)
            } else if bits & 0xfff0007f == 0xc2100053 {
                insn_impl::fcvt_wu_d::fcvt_wu_d(insn, self, bus)
            } else if bits & 0xfff0007f == 0xd2000053 {
                insn_impl::fcvt_d_w::fcvt_d_w(insn, self, bus)
            } else if bits & 0xfff0007f == 0xd2100053 {
                insn_impl::fcvt_d_wu::fcvt_d_wu(insn, self, bus)
            } else if bits & 0xfff0007f == 0xc2200053 {
                insn_impl::fcvt_l_d::fcvt_l_d(insn, self, bus)
            } else if bits & 0xfff0007f == 0xc2300053 {
                insn_impl::fcvt_lu_d::fcvt_lu_d(insn, self, bus)
            } else if bits & 0xfff0707f == 0xe2000053 {
                insn_impl::fmv_x_d::fmv_x_d(insn, self, bus)
            } else if bits & 0xfff0007f == 0xd2200053 {
                insn_impl::fcvt_d_l::fcvt_d_l(insn, self, bus)
            } else if bits & 0xfff0007f == 0xd2300053 {
                insn_impl::fcvt_d_lu::fcvt_d_lu(insn, self, bus)
            } else if bits & 0xfff0707f == 0xf2000053 {
                insn_impl::fmv_d_x::fmv_d_x(insn, self, bus)
            } else if bits & 0xfe00707f == 0x2000033 {
                insn_impl::mul::mul(insn, self, bus)
            } else if bits & 0xfe00707f == 0x2001033 {
                insn_impl::mulh::mulh(insn, self, bus)
            } else if bits & 0xfe00707f == 0x2002033 {
                insn_impl::mulhsu::mulhsu(insn, self, bus)
            } else if bits & 0xfe00707f == 0x2003033 {
                insn_impl::mulhu::mulhu(insn, self, bus)
            } else if bits & 0xfe00707f == 0x2004033 {
                insn_impl::div::div(insn, self, bus)
            } else if bits & 0xfe00707f == 0x2005033 {
                insn_impl::divu::divu(insn, self, bus)
            } else if bits & 0xfe00707f == 0x2006033 {
                insn_impl::rem::rem(insn, self, bus)
            } else if bits & 0xfe00707f == 0x2007033 {
                insn_impl::remu::remu(insn, self, bus)
            } else if bits & 0xfe00707f == 0x200003b {
                insn_impl::mulw::mulw(insn, self, bus)
            } else if bits & 0xfe00707f == 0x200403b {
                insn_impl::divw::divw(insn, self, bus)
            } else if bits & 0xfe00707f == 0x200503b {
                insn_impl::divuw::divuw(insn, self, bus)
            } else if bits & 0xfe00707f == 0x200603b {
                insn_impl::remw::remw(insn, self, bus)
            } else if bits & 0xfe00707f == 0x200703b {
                insn_impl::remuw::remuw(insn, self, bus)
            } else if bits & 0xe003 == 0x2000 {
                insn_impl::c_fld::c_fld(insn, self, bus)
            } else if bits & 0xe003 == 0xa000 {
                insn_impl::c_fsd::c_fsd(insn, self, bus)
            } else if bits & 0xe003 == 0x2002 {
                insn_impl::c_fldsp::c_fldsp(insn, self, bus)
            } else if bits & 0xe003 == 0xa002 {
                insn_impl::c_fsdsp::c_fsdsp(insn, self, bus)
            } else {
                Err(cpu::Exception::IllegalInstruction)
            }
        }
    }
}
