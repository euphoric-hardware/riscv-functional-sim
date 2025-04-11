use crate::{bus::Bus, cpu::{self, Cpu}, insn_impl::insn_cached};
use super::uop_cache::UopCacheEntry;

impl UopCacheEntry {
    pub fn set_cached_insn(bits: u64) -> Option<fn(cpu: &mut Cpu, bus: &mut Bus, &UopCacheEntry) -> cpu::Result<u64>> {

        if bits & 0x7f == 0x37 {
            Some(insn_cached::lui_cached::lui_cached)
        }

        else if bits & 0x7f == 0x17 {
            Some(insn_cached::auipc_cached::auipc_cached)
        }

        else if bits & 0x7f == 0x6f {
            Some(insn_cached::jal_cached::jal_cached)
        }

        else if bits & 0x707f == 0x67 {
            Some(insn_cached::jalr_cached::jalr_cached)
        }

        else if bits & 0x707f == 0x63 {
            Some(insn_cached::beq_cached::beq_cached)
        }

        else if bits & 0x707f == 0x1063 {
            Some(insn_cached::bne_cached::bne_cached)
        }

        else if bits & 0x707f == 0x4063 {
            Some(insn_cached::blt_cached::blt_cached)
        }

        else if bits & 0x707f == 0x5063 {
            Some(insn_cached::bge_cached::bge_cached)
        }

        else if bits & 0x707f == 0x6063 {
            Some(insn_cached::bltu_cached::bltu_cached)
        }

        else if bits & 0x707f == 0x7063 {
            Some(insn_cached::bgeu_cached::bgeu_cached)
        }

        else if bits & 0x707f == 0x3 {
            Some(insn_cached::lb_cached::lb_cached)
        }

        else if bits & 0x707f == 0x1003 {
            Some(insn_cached::lh_cached::lh_cached)
        }

        else if bits & 0x707f == 0x2003 {
            Some(insn_cached::lw_cached::lw_cached)
        }

        else if bits & 0x707f == 0x4003 {
            Some(insn_cached::lbu_cached::lbu_cached)
        }

        else if bits & 0x707f == 0x5003 {
            Some(insn_cached::lhu_cached::lhu_cached)
        }

        else if bits & 0x707f == 0x23 {
            Some(insn_cached::sb_cached::sb_cached)
        }

        else if bits & 0x707f == 0x1023 {
            Some(insn_cached::sh_cached::sh_cached)
        }

        else if bits & 0x707f == 0x2023 {
            Some(insn_cached::sw_cached::sw_cached)
        }

        else if bits & 0x707f == 0x13 {
            Some(insn_cached::addi_cached::addi_cached)
        }

        else if bits & 0x707f == 0x2013 {
            Some(insn_cached::slti_cached::slti_cached)
        }

        else if bits & 0x707f == 0x3013 {
            Some(insn_cached::sltiu_cached::sltiu_cached)
        }

        else if bits & 0x707f == 0x4013 {
            Some(insn_cached::xori_cached::xori_cached)
        }

        else if bits & 0x707f == 0x6013 {
            Some(insn_cached::ori_cached::ori_cached)
        }

        else if bits & 0x707f == 0x7013 {
            Some(insn_cached::andi_cached::andi_cached)
        }

        else if bits & 0xfe00707f == 0x33 {
            Some(insn_cached::add_cached::add_cached)
        }

        else if bits & 0xfe00707f == 0x40000033 {
            Some(insn_cached::sub_cached::sub_cached)
        }

        else if bits & 0xfe00707f == 0x1033 {
            Some(insn_cached::sll_cached::sll_cached)
        }

        else if bits & 0xfe00707f == 0x2033 {
            Some(insn_cached::slt_cached::slt_cached)
        }

        else if bits & 0xfe00707f == 0x3033 {
            Some(insn_cached::sltu_cached::sltu_cached)
        }

        else if bits & 0xfe00707f == 0x4033 {
            Some(insn_cached::xor_cached::xor_cached)
        }

        else if bits & 0xfe00707f == 0x5033 {
            Some(insn_cached::srl_cached::srl_cached)
        }

        else if bits & 0xfe00707f == 0x40005033 {
            Some(insn_cached::sra_cached::sra_cached)
        }

        else if bits & 0xfe00707f == 0x6033 {
            Some(insn_cached::or_cached::or_cached)
        }

        else if bits & 0xfe00707f == 0x7033 {
            Some(insn_cached::and_cached::and_cached)
        }

        else if bits & 0x707f == 0xf {
            Some(insn_cached::fence_cached::fence_cached)
        }

        else if bits & 0xffffffff == 0x73 {
            Some(insn_cached::ecall_cached::ecall_cached)
        }

        else if bits & 0xffffffff == 0x100073 {
            Some(insn_cached::ebreak_cached::ebreak_cached)
        }

        else if bits & 0x707f == 0x6003 {
            Some(insn_cached::lwu_cached::lwu_cached)
        }

        else if bits & 0x707f == 0x3003 {
            Some(insn_cached::ld_cached::ld_cached)
        }

        else if bits & 0x707f == 0x3023 {
            Some(insn_cached::sd_cached::sd_cached)
        }

        else if bits & 0xfc00707f == 0x1013 {
            Some(insn_cached::slli_cached::slli_cached)
        }

        else if bits & 0xfc00707f == 0x5013 {
            Some(insn_cached::srli_cached::srli_cached)
        }

        else if bits & 0xfc00707f == 0x40005013 {
            Some(insn_cached::srai_cached::srai_cached)
        }

        else if bits & 0x707f == 0x1b {
            Some(insn_cached::addiw_cached::addiw_cached)
        }

        else if bits & 0xfe00707f == 0x101b {
            Some(insn_cached::slliw_cached::slliw_cached)
        }

        else if bits & 0xfe00707f == 0x501b {
            Some(insn_cached::srliw_cached::srliw_cached)
        }

        else if bits & 0xfe00707f == 0x4000501b {
            Some(insn_cached::sraiw_cached::sraiw_cached)
        }

        else if bits & 0xfe00707f == 0x3b {
            Some(insn_cached::addw_cached::addw_cached)
        }

        else if bits & 0xfe00707f == 0x4000003b {
            Some(insn_cached::subw_cached::subw_cached)
        }

        else if bits & 0xfe00707f == 0x103b {
            Some(insn_cached::sllw_cached::sllw_cached)
        }

        else if bits & 0xfe00707f == 0x503b {
            Some(insn_cached::srlw_cached::srlw_cached)
        }

        else if bits & 0xfe00707f == 0x4000503b {
            Some(insn_cached::sraw_cached::sraw_cached)
        }

        else if bits & 0x707f == 0x1073 {
            Some(insn_cached::csrrw_cached::csrrw_cached)
        }

        else if bits & 0x707f == 0x2073 {
            Some(insn_cached::csrrs_cached::csrrs_cached)
        }

        else if bits & 0x707f == 0x3073 {
            Some(insn_cached::csrrc_cached::csrrc_cached)
        }

        else if bits & 0x707f == 0x5073 {
            Some(insn_cached::csrrwi_cached::csrrwi_cached)
        }

        else if bits & 0x707f == 0x6073 {
            Some(insn_cached::csrrsi_cached::csrrsi_cached)
        }

        else if bits & 0x707f == 0x7073 {
            Some(insn_cached::csrrci_cached::csrrci_cached)
        }

        else if bits & 0xffffffff == 0x30200073 {
            Some(insn_cached::mret_cached::mret_cached)
        }

        else if bits & 0xffffffff == 0x10500073 {
            Some(insn_cached::wfi_cached::wfi_cached)
        }

        else if bits & 0xe003 == 0x0 {
            Some(insn_cached::c_addi4spn_cached::c_addi4spn_cached)
        }

        else if bits & 0xe003 == 0x4000 {
            Some(insn_cached::c_lw_cached::c_lw_cached)
        }

        else if bits & 0xe003 == 0xc000 {
            Some(insn_cached::c_sw_cached::c_sw_cached)
        }

        else if bits & 0xef83 == 0x1 {
            Some(insn_cached::c_nop_cached::c_nop_cached)
        }

        else if bits & 0xe003 == 0x1 {
            Some(insn_cached::c_addi_cached::c_addi_cached)
        }

        else if bits & 0xe003 == 0x4001 {
            Some(insn_cached::c_li_cached::c_li_cached)
        }

        else if bits & 0xef83 == 0x6101 {
            Some(insn_cached::c_addi16sp_cached::c_addi16sp_cached)
        }

        else if bits & 0xe003 == 0x6001 {
            Some(insn_cached::c_lui_cached::c_lui_cached)
        }

        else if bits & 0xec03 == 0x8801 {
            Some(insn_cached::c_andi_cached::c_andi_cached)
        }

        else if bits & 0xfc63 == 0x8c01 {
            Some(insn_cached::c_sub_cached::c_sub_cached)
        }

        else if bits & 0xfc63 == 0x8c21 {
            Some(insn_cached::c_xor_cached::c_xor_cached)
        }

        else if bits & 0xfc63 == 0x8c41 {
            Some(insn_cached::c_or_cached::c_or_cached)
        }

        else if bits & 0xfc63 == 0x8c61 {
            Some(insn_cached::c_and_cached::c_and_cached)
        }

        else if bits & 0xe003 == 0xa001 {
            Some(insn_cached::c_j_cached::c_j_cached)
        }

        else if bits & 0xe003 == 0xc001 {
            Some(insn_cached::c_beqz_cached::c_beqz_cached)
        }

        else if bits & 0xe003 == 0xe001 {
            Some(insn_cached::c_bnez_cached::c_bnez_cached)
        }

        else if bits & 0xe003 == 0x4002 {
            Some(insn_cached::c_lwsp_cached::c_lwsp_cached)
        }

        else if bits & 0xf07f == 0x8002 {
            Some(insn_cached::c_jr_cached::c_jr_cached)
        }

        else if bits & 0xf003 == 0x8002 {
            Some(insn_cached::c_mv_cached::c_mv_cached)
        }

        else if bits & 0xffff == 0x9002 {
            Some(insn_cached::c_ebreak_cached::c_ebreak_cached)
        }

        else if bits & 0xf07f == 0x9002 {
            Some(insn_cached::c_jalr_cached::c_jalr_cached)
        }

        else if bits & 0xf003 == 0x9002 {
            Some(insn_cached::c_add_cached::c_add_cached)
        }

        else if bits & 0xe003 == 0xc002 {
            Some(insn_cached::c_swsp_cached::c_swsp_cached)
        }

        else if bits & 0xe003 == 0x6000 {
            Some(insn_cached::c_ld_cached::c_ld_cached)
        }

        else if bits & 0xe003 == 0xe000 {
            Some(insn_cached::c_sd_cached::c_sd_cached)
        }

        else if bits & 0xe003 == 0x2001 {
            Some(insn_cached::c_addiw_cached::c_addiw_cached)
        }

        else if bits & 0xec03 == 0x8001 {
            Some(insn_cached::c_srli_cached::c_srli_cached)
        }

        else if bits & 0xec03 == 0x8401 {
            Some(insn_cached::c_srai_cached::c_srai_cached)
        }

        else if bits & 0xfc63 == 0x9c01 {
            Some(insn_cached::c_subw_cached::c_subw_cached)
        }

        else if bits & 0xfc63 == 0x9c21 {
            Some(insn_cached::c_addw_cached::c_addw_cached)
        }

        else if bits & 0xe003 == 0x2 {
            Some(insn_cached::c_slli_cached::c_slli_cached)
        }

        else if bits & 0xe003 == 0x6002 {
            Some(insn_cached::c_ldsp_cached::c_ldsp_cached)
        }

        else if bits & 0xe003 == 0xe002 {
            Some(insn_cached::c_sdsp_cached::c_sdsp_cached)
        }

        else if bits & 0x707f == 0x2007 {
            Some(insn_cached::flw_cached::flw_cached)
        }

        else if bits & 0x707f == 0x2027 {
            Some(insn_cached::fsw_cached::fsw_cached)
        }

        else if bits & 0x600007f == 0x43 {
            Some(insn_cached::fmadd_s_cached::fmadd_s_cached)
        }

        else if bits & 0x600007f == 0x47 {
            Some(insn_cached::fmsub_s_cached::fmsub_s_cached)
        }

        else if bits & 0x600007f == 0x4b {
            Some(insn_cached::fnmsub_s_cached::fnmsub_s_cached)
        }

        else if bits & 0x600007f == 0x4f {
            Some(insn_cached::fnmadd_s_cached::fnmadd_s_cached)
        }

        else if bits & 0xfe00007f == 0x53 {
            Some(insn_cached::fadd_s_cached::fadd_s_cached)
        }

        else if bits & 0xfe00007f == 0x8000053 {
            Some(insn_cached::fsub_s_cached::fsub_s_cached)
        }

        else if bits & 0xfe00007f == 0x10000053 {
            Some(insn_cached::fmul_s_cached::fmul_s_cached)
        }

        else if bits & 0xfe00007f == 0x18000053 {
            Some(insn_cached::fdiv_s_cached::fdiv_s_cached)
        }

        else if bits & 0xfff0007f == 0x58000053 {
            Some(insn_cached::fsqrt_s_cached::fsqrt_s_cached)
        }

        else if bits & 0xfe00707f == 0x20000053 {
            Some(insn_cached::fsgnj_s_cached::fsgnj_s_cached)
        }

        else if bits & 0xfe00707f == 0x20001053 {
            Some(insn_cached::fsgnjn_s_cached::fsgnjn_s_cached)
        }

        else if bits & 0xfe00707f == 0x20002053 {
            Some(insn_cached::fsgnjx_s_cached::fsgnjx_s_cached)
        }

        else if bits & 0xfe00707f == 0x28000053 {
            Some(insn_cached::fmin_s_cached::fmin_s_cached)
        }

        else if bits & 0xfe00707f == 0x28001053 {
            Some(insn_cached::fmax_s_cached::fmax_s_cached)
        }

        else if bits & 0xfff0007f == 0xc0000053 {
            Some(insn_cached::fcvt_w_s_cached::fcvt_w_s_cached)
        }

        else if bits & 0xfff0007f == 0xc0100053 {
            Some(insn_cached::fcvt_wu_s_cached::fcvt_wu_s_cached)
        }

        else if bits & 0xfff0707f == 0xe0000053 {
            Some(insn_cached::fmv_x_w_cached::fmv_x_w_cached)
        }

        else if bits & 0xfe00707f == 0xa0002053 {
            Some(insn_cached::feq_s_cached::feq_s_cached)
        }

        else if bits & 0xfe00707f == 0xa0001053 {
            Some(insn_cached::flt_s_cached::flt_s_cached)
        }

        else if bits & 0xfe00707f == 0xa0000053 {
            Some(insn_cached::fle_s_cached::fle_s_cached)
        }

        else if bits & 0xfff0707f == 0xe0001053 {
            Some(insn_cached::fclass_s_cached::fclass_s_cached)
        }

        else if bits & 0xfff0007f == 0xd0000053 {
            Some(insn_cached::fcvt_s_w_cached::fcvt_s_w_cached)
        }

        else if bits & 0xfff0007f == 0xd0100053 {
            Some(insn_cached::fcvt_s_wu_cached::fcvt_s_wu_cached)
        }

        else if bits & 0xfff0707f == 0xf0000053 {
            Some(insn_cached::fmv_w_x_cached::fmv_w_x_cached)
        }

        else if bits & 0xfff0007f == 0xc0200053 {
            Some(insn_cached::fcvt_l_s_cached::fcvt_l_s_cached)
        }

        else if bits & 0xfff0007f == 0xc0300053 {
            Some(insn_cached::fcvt_lu_s_cached::fcvt_lu_s_cached)
        }

        else if bits & 0xfff0007f == 0xd0200053 {
            Some(insn_cached::fcvt_s_l_cached::fcvt_s_l_cached)
        }

        else if bits & 0xfff0007f == 0xd0300053 {
            Some(insn_cached::fcvt_s_lu_cached::fcvt_s_lu_cached)
        }

        else if bits & 0x707f == 0x3007 {
            Some(insn_cached::fld_cached::fld_cached)
        }

        else if bits & 0x707f == 0x3027 {
            Some(insn_cached::fsd_cached::fsd_cached)
        }

        else if bits & 0x600007f == 0x2000043 {
            Some(insn_cached::fmadd_d_cached::fmadd_d_cached)
        }

        else if bits & 0x600007f == 0x2000047 {
            Some(insn_cached::fmsub_d_cached::fmsub_d_cached)
        }

        else if bits & 0x600007f == 0x200004b {
            Some(insn_cached::fnmsub_d_cached::fnmsub_d_cached)
        }

        else if bits & 0x600007f == 0x200004f {
            Some(insn_cached::fnmadd_d_cached::fnmadd_d_cached)
        }

        else if bits & 0xfe00007f == 0x2000053 {
            Some(insn_cached::fadd_d_cached::fadd_d_cached)
        }

        else if bits & 0xfe00007f == 0xa000053 {
            Some(insn_cached::fsub_d_cached::fsub_d_cached)
        }

        else if bits & 0xfe00007f == 0x12000053 {
            Some(insn_cached::fmul_d_cached::fmul_d_cached)
        }

        else if bits & 0xfe00007f == 0x1a000053 {
            Some(insn_cached::fdiv_d_cached::fdiv_d_cached)
        }

        else if bits & 0xfff0007f == 0x5a000053 {
            Some(insn_cached::fsqrt_d_cached::fsqrt_d_cached)
        }

        else if bits & 0xfe00707f == 0x22000053 {
            Some(insn_cached::fsgnj_d_cached::fsgnj_d_cached)
        }

        else if bits & 0xfe00707f == 0x22001053 {
            Some(insn_cached::fsgnjn_d_cached::fsgnjn_d_cached)
        }

        else if bits & 0xfe00707f == 0x22002053 {
            Some(insn_cached::fsgnjx_d_cached::fsgnjx_d_cached)
        }

        else if bits & 0xfe00707f == 0x2a000053 {
            Some(insn_cached::fmin_d_cached::fmin_d_cached)
        }

        else if bits & 0xfe00707f == 0x2a001053 {
            Some(insn_cached::fmax_d_cached::fmax_d_cached)
        }

        else if bits & 0xfff0007f == 0x40100053 {
            Some(insn_cached::fcvt_s_d_cached::fcvt_s_d_cached)
        }

        else if bits & 0xfff0007f == 0x42000053 {
            Some(insn_cached::fcvt_d_s_cached::fcvt_d_s_cached)
        }

        else if bits & 0xfe00707f == 0xa2002053 {
            Some(insn_cached::feq_d_cached::feq_d_cached)
        }

        else if bits & 0xfe00707f == 0xa2001053 {
            Some(insn_cached::flt_d_cached::flt_d_cached)
        }

        else if bits & 0xfe00707f == 0xa2000053 {
            Some(insn_cached::fle_d_cached::fle_d_cached)
        }

        else if bits & 0xfff0707f == 0xe2001053 {
            Some(insn_cached::fclass_d_cached::fclass_d_cached)
        }

        else if bits & 0xfff0007f == 0xc2000053 {
            Some(insn_cached::fcvt_w_d_cached::fcvt_w_d_cached)
        }

        else if bits & 0xfff0007f == 0xc2100053 {
            Some(insn_cached::fcvt_wu_d_cached::fcvt_wu_d_cached)
        }

        else if bits & 0xfff0007f == 0xd2000053 {
            Some(insn_cached::fcvt_d_w_cached::fcvt_d_w_cached)
        }

        else if bits & 0xfff0007f == 0xd2100053 {
            Some(insn_cached::fcvt_d_wu_cached::fcvt_d_wu_cached)
        }

        else if bits & 0xfff0007f == 0xc2200053 {
            Some(insn_cached::fcvt_l_d_cached::fcvt_l_d_cached)
        }

        else if bits & 0xfff0007f == 0xc2300053 {
            Some(insn_cached::fcvt_lu_d_cached::fcvt_lu_d_cached)
        }

        else if bits & 0xfff0707f == 0xe2000053 {
            Some(insn_cached::fmv_x_d_cached::fmv_x_d_cached)
        }

        else if bits & 0xfff0007f == 0xd2200053 {
            Some(insn_cached::fcvt_d_l_cached::fcvt_d_l_cached)
        }

        else if bits & 0xfff0007f == 0xd2300053 {
            Some(insn_cached::fcvt_d_lu_cached::fcvt_d_lu_cached)
        }

        else if bits & 0xfff0707f == 0xf2000053 {
            Some(insn_cached::fmv_d_x_cached::fmv_d_x_cached)
        }

        else if bits & 0xfe00707f == 0x2000033 {
            Some(insn_cached::mul_cached::mul_cached)
        }

        else if bits & 0xfe00707f == 0x2001033 {
            Some(insn_cached::mulh_cached::mulh_cached)
        }

        else if bits & 0xfe00707f == 0x2002033 {
            Some(insn_cached::mulhsu_cached::mulhsu_cached)
        }

        else if bits & 0xfe00707f == 0x2003033 {
            Some(insn_cached::mulhu_cached::mulhu_cached)
        }

        else if bits & 0xfe00707f == 0x2004033 {
            Some(insn_cached::div_cached::div_cached)
        }

        else if bits & 0xfe00707f == 0x2005033 {
            Some(insn_cached::divu_cached::divu_cached)
        }

        else if bits & 0xfe00707f == 0x2006033 {
            Some(insn_cached::rem_cached::rem_cached)
        }

        else if bits & 0xfe00707f == 0x2007033 {
            Some(insn_cached::remu_cached::remu_cached)
        }

        else if bits & 0xfe00707f == 0x200003b {
            Some(insn_cached::mulw_cached::mulw_cached)
        }

        else if bits & 0xfe00707f == 0x200403b {
            Some(insn_cached::divw_cached::divw_cached)
        }

        else if bits & 0xfe00707f == 0x200503b {
            Some(insn_cached::divuw_cached::divuw_cached)
        }

        else if bits & 0xfe00707f == 0x200603b {
            Some(insn_cached::remw_cached::remw_cached)
        }

        else if bits & 0xfe00707f == 0x200703b {
            Some(insn_cached::remuw_cached::remuw_cached)
        }

        else if bits & 0xe003 == 0x2000 {
            Some(insn_cached::c_fld_cached::c_fld_cached)
        }

        else if bits & 0xe003 == 0xa000 {
            Some(insn_cached::c_fsd_cached::c_fsd_cached)
        }

        else if bits & 0xe003 == 0x2002 {
            Some(insn_cached::c_fldsp_cached::c_fldsp_cached)
        }

        else if bits & 0xe003 == 0xa002 {
            Some(insn_cached::c_fsdsp_cached::c_fsdsp_cached)
        }

        else {
            None
        }
    }
}

