#![cfg_attr(rustfmt, rustfmt_skip)]
use modular_bitfield::bitfield;

#[bitfield]
pub struct HgeieRaw {
    data: B64
}


#[bitfield]
pub struct SatpRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter24hRaw {
    data: B64
}


#[bitfield]
pub struct HedelegRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr6Raw {
    data: B64
}


#[bitfield]
pub struct UscratchcswlRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter16hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter31Raw {
    data: B64
}


#[bitfield]
pub struct StvalRaw {
    data: B64
}


#[bitfield]
pub struct UepcRaw {
    data: B64
}


#[bitfield]
pub struct McounterenRaw {
    cy: B1,
    tm: B1,
    ir: B1,
    hpm: B29,
    #[skip] __: B32
}


#[bitfield]
pub struct HcontextRaw {
    data: B64
}


#[bitfield]
pub struct VsstatusRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter4Raw {
    data: B64
}


#[bitfield]
pub struct MinstrethRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent3Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter4hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter30Raw {
    data: B64
}


#[bitfield]
pub struct MtimeRaw {
    data: B64
}


#[bitfield]
pub struct UscratchcswRaw {
    data: B64
}


#[bitfield]
pub struct CycleRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter23Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter28Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter15hRaw {
    data: B64
}


#[bitfield]
pub struct Dscratch0Raw {
    data: B64
}


#[bitfield]
pub struct HgatpRaw {
    data: B64
}


#[bitfield]
pub struct MenvcfghRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter27Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter27Raw {
    data: B64
}


#[bitfield]
pub struct SidelegRaw {
    data: B64
}


#[bitfield]
pub struct SscratchcswlRaw {
    data: B64
}


#[bitfield]
pub struct SscratchRaw {
    data: B64
}


#[bitfield]
pub struct StvtRaw {
    data: B64
}


#[bitfield]
pub struct MdboundRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter19Raw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr15Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent25Raw {
    data: B64
}


#[bitfield]
pub struct VxsatRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter6Raw {
    data: B64
}


#[bitfield]
pub struct MibaseRaw {
    data: B64
}


#[bitfield]
pub struct VsipRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter11hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter15hRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter31hRaw {
    data: B64
}


#[bitfield]
pub struct HstatusRaw {
    data: B64
}


#[bitfield]
pub struct SnxtiRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent20Raw {
    data: B64
}


#[bitfield]
pub struct FflagsRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter12hRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter3hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent10Raw {
    data: B64
}


#[bitfield]
pub struct Dscratch1Raw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr2Raw {
    data: B64
}


#[bitfield]
pub struct VsscratchRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr0Raw {
    data: B64
}


#[bitfield]
pub struct VsatpRaw {
    data: B64
}


#[bitfield]
pub struct MscratchcswlRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr3Raw {
    data: B64
}


#[bitfield]
pub struct Tdata2Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter18hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter19hRaw {
    data: B64
}


#[bitfield]
pub struct UscratchRaw {
    data: B64
}


#[bitfield]
pub struct MhartidRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr7Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter22hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter28hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent18Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter11Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter3Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter5Raw {
    data: B64
}


#[bitfield]
pub struct FrmRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter18Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent5Raw {
    data: B64
}


#[bitfield]
pub struct McauseRaw {
    exception_code: McauseExceptionCode,
    interrupt_code: McauseInterruptCode,
    interrupt: McauseInterrupt,
}

#[derive(BitfieldSpecifier)]
#[bits = 63]
pub enum McauseExceptionCode {
    RESERVED21 = 21, INSTRUCTION_ADDRESS_MISALIGNED = 0, RESERVED17 = 17, CUSTOM27 = 27, STORE_AMO_ACCESS_FAULT = 7, RESERVED59 = 59, CUSTOM31 = 31, ENVIRONMENT_CALL_FROM_U_MODE = 8, RESERVED36 = 36, RESERVED43 = 43, RESERVED55 = 55, RESERVED19 = 19, RESERVED45 = 45, RESERVED49 = 49, LOAD_ADDRESS_MISALIGNED = 4, RESERVED53 = 53, RESERVED51 = 51, RESERVED38 = 38, RESERVED52 = 52, RESERVED34 = 34, RESERVED37 = 37, LOAD_ACCESS_FAULT = 5, STORE_AMO_ADDRESS_MISALIGNED = 6, BREAKPOINT = 3, RESERVED58 = 58, INSTRUCTION_ACCESS_FAULT = 1, RESERVED56 = 56, RESERVED40 = 40, RESERVED22 = 22, RESERVED47 = 47, RESERVED20 = 20, RESERVED54 = 54, CUSTOM25 = 25, RESERVED50 = 50, RESERVED62 = 62, CUSTOM30 = 30, RESERVED48 = 48, RESERVED33 = 33, RESERVED41 = 41, RESERVED18 = 18, RESERVED61 = 61, INSTRUCTION_PAGE_FAULT = 12, RESERVED57 = 57, CUSTOM29 = 29, ENVIRONMENT_CALL_FROM_M_MODE = 11, RESERVED44 = 44, RESERVED42 = 42, RESERVED46 = 46, RESERVED60 = 60, RESERVED16 = 16, RESERVED63 = 63, ENVIRONMENT_CALL_FROM_S_MODE = 9, LOAD_PAGE_FAULT = 13, RESERVED39 = 39, ILLEGAL_INSTRUCTION = 2, RESERVED23 = 23, CUSTOM28 = 28, RESERVED32 = 32, RESERVED35 = 35, STORE_AMO_PAGE_FAULT = 15, CUSTOM26 = 26, RESERVED10 = 10, CUSTOM24 = 24, RESERVED14 = 14, 
}

#[derive(BitfieldSpecifier)]
#[bits = 63]
pub enum McauseInterruptCode {
    MTI = 7, SSI = 1, PLATFORM_DEFINED24 = 24, STI = 5, PLATFORM_DEFINED25 = 25, PLATFORM_DEFINED28 = 28, MSI = 3, UTI = 4, PLATFORM_DEFINED20 = 20, PLATFORM_DEFINED17 = 17, PLATFORM_DEFINED26 = 26, PLATFORM_DEFINED18 = 18, PLATFORM_DEFINED19 = 19, PLATFORM_DEFINED27 = 27, SEI = 9, PLATFORM_DEFINED29 = 29, PLATFORM_DEFINED22 = 22, USI = 0, MEI = 11, PLATFORM_DEFINED16 = 16, UEI = 8, PLATFORM_DEFINED23 = 23, PLATFORM_DEFINED30 = 30, PLATFORM_DEFINED31 = 31, PLATFORM_DEFINED21 = 21, 
}

#[derive(BitfieldSpecifier)]
#[bits = 1]
pub enum McauseInterrupt {
    TRAP = 0, INTERRUPT = 1, 
}


#[bitfield]
pub struct Mhpmcounter14Raw {
    data: B64
}


#[bitfield]
pub struct SepcRaw {
    data: B64
}


#[bitfield]
pub struct DcsrRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent29Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter25hRaw {
    data: B64
}


#[bitfield]
pub struct MtvecRaw {
    mode: B2,
    base: B62,
}


#[bitfield]
pub struct Hpmcounter15Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter17Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter13Raw {
    data: B64
}


#[bitfield]
pub struct VstvalRaw {
    data: B64
}


#[bitfield]
pub struct CyclehRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter12hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent14Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter13Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter23hRaw {
    data: B64
}


#[bitfield]
pub struct SipRaw {
    usi: B1,
    ssi: B1,
    #[skip] __: B2,
    uti: B1,
    sti: B1,
    #[skip] __: B2,
    uei: B1,
    sei: B1,
    #[skip] __: B54
}


#[bitfield]
pub struct UtvalRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter6hRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter10hRaw {
    data: B64
}


#[bitfield]
pub struct VxrmRaw {
    data: B64
}


#[bitfield]
pub struct BsepcRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr12Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter26hRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter31Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter31hRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter4Raw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr63Raw {
    data: B64
}


#[bitfield]
pub struct BsatpRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter16Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter10Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent26Raw {
    data: B64
}


#[bitfield]
pub struct SenvcfgRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter8hRaw {
    data: B64
}


#[bitfield]
pub struct Mtval2Raw {
    data: B64
}


#[bitfield]
pub struct MintstatusRaw {
    data: B64
}


#[bitfield]
pub struct UtvecRaw {
    mode: B2,
    base: B62,
}


#[bitfield]
pub struct InstrethRaw {
    data: B64
}


#[bitfield]
pub struct MseccfgRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter4hRaw {
    data: B64
}


#[bitfield]
pub struct HipRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr11Raw {
    data: B64
}


#[bitfield]
pub struct DpcRaw {
    data: B64
}


#[bitfield]
pub struct MscratchRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter8Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter24hRaw {
    data: B64
}


#[bitfield]
pub struct MenvcfgRaw {
    data: B64
}


#[bitfield]
pub struct SieRaw {
    usi: B1,
    ssi: B1,
    #[skip] __: B2,
    uti: B1,
    sti: B1,
    #[skip] __: B2,
    uei: B1,
    sei: B1,
    #[skip] __: B54
}


#[bitfield]
pub struct Mhpmcounter11hRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr8Raw {
    data: B64
}


#[bitfield]
pub struct Tdata3Raw {
    data: B64
}


#[bitfield]
pub struct VstvecRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter17Raw {
    data: B64
}


#[bitfield]
pub struct MvendoridRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent15Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter21hRaw {
    data: B64
}


#[bitfield]
pub struct MdbaseRaw {
    data: B64
}


#[bitfield]
pub struct HtimedeltaRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter5hRaw {
    data: B64
}


#[bitfield]
pub struct HtinstRaw {
    data: B64
}


#[bitfield]
pub struct UstatusRaw {
    #[skip] __: B1,
    uie: B1,
    #[skip] __: B1,
    upie: B1,
    #[skip] __: B60
}


#[bitfield]
pub struct Mhpmevent23Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter13hRaw {
    data: B64
}


#[bitfield]
pub struct MiboundRaw {
    data: B64
}


#[bitfield]
pub struct HenvcfghRaw {
    data: B64
}


#[bitfield]
pub struct BscauseRaw {
    data: B64
}


#[bitfield]
pub struct HenvcfgRaw {
    data: B64
}


#[bitfield]
pub struct Pmpcfg0Raw {
    data: B64
}


#[bitfield]
pub struct SintstatusRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent30Raw {
    data: B64
}


#[bitfield]
pub struct BstvalRaw {
    data: B64
}


#[bitfield]
pub struct Pmpcfg15Raw {
    data: B64
}


#[bitfield]
pub struct McontextRaw {
    data: B64
}


#[bitfield]
pub struct MisaRaw {
    extensions: B26,
    #[skip] __: B36,
    mxl: B2,
}


#[bitfield]
pub struct MimpidRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr1Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter21hRaw {
    data: B64
}


#[bitfield]
pub struct McyclehRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent21Raw {
    data: B64
}


#[bitfield]
pub struct SscratchcswRaw {
    data: B64
}


#[bitfield]
pub struct MstatusRaw {
    #[skip] __: B2,
    sie: B1,
    mie: B1,
    #[skip] __: B1,
    spie: B1,
    ube: B1,
    mpie: B1,
    spp: B1,
    vs: B2,
    mpp: B2,
    fs: B2,
    xs: B2,
    mprv: B1,
    sum: B1,
    mxr: B1,
    tvm: B1,
    tw: B1,
    tsr: B1,
    #[skip] __: B9,
    uxl: B2,
    sxl: B2,
    sbe: B1,
    mbe: B1,
    #[skip] __: B25,
    sd: B1,
}


#[bitfield]
pub struct Mhpmcounter22Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter20hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter3Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent22Raw {
    data: B64
}


#[bitfield]
pub struct VsieRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter12Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter14hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent12Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter29Raw {
    data: B64
}


#[bitfield]
pub struct MtinstRaw {
    data: B64
}


#[bitfield]
pub struct Pmpcfg2Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter23hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter3hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter5Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter28Raw {
    data: B64
}


#[bitfield]
pub struct TimehRaw {
    data: B64
}


#[bitfield]
pub struct MbaseRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr13Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter17hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent7Raw {
    data: B64
}


#[bitfield]
pub struct HgeipRaw {
    data: B64
}


#[bitfield]
pub struct VsepcRaw {
    data: B64
}


#[bitfield]
pub struct BstvecRaw {
    data: B64
}


#[bitfield]
pub struct UintstatusRaw {
    data: B64
}


#[bitfield]
pub struct VtypeRaw {
    data: B64
}


#[bitfield]
pub struct HieRaw {
    data: B64
}


#[bitfield]
pub struct UnxtiRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter11Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter25Raw {
    data: B64
}


#[bitfield]
pub struct InstretRaw {
    data: B64
}


#[bitfield]
pub struct Pmpcfg1Raw {
    data: B64
}


#[bitfield]
pub struct Pmpcfg14Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter9Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent9Raw {
    data: B64
}


#[bitfield]
pub struct TimeRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter9hRaw {
    data: B64
}


#[bitfield]
pub struct UieRaw {
    usi: B1,
    #[skip] __: B3,
    uti: B1,
    #[skip] __: B3,
    uei: B1,
    #[skip] __: B55
}


#[bitfield]
pub struct BsieRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter30hRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter7hRaw {
    data: B64
}


#[bitfield]
pub struct BsstatusRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter22Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter29hRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter28hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter21Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter27hRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr14Raw {
    data: B64
}


#[bitfield]
pub struct UipRaw {
    usi: B1,
    #[skip] __: B3,
    uti: B1,
    #[skip] __: B3,
    uei: B1,
    #[skip] __: B55
}


#[bitfield]
pub struct Mhpmcounter26Raw {
    data: B64
}


#[bitfield]
pub struct McountinhibitRaw {
    cy: B1,
    #[skip] __: B1,
    ir: B1,
    hpm: B29,
    #[skip] __: B32
}


#[bitfield]
pub struct Mhpmcounter20Raw {
    data: B64
}


#[bitfield]
pub struct UtvtRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter6hRaw {
    data: B64
}


#[bitfield]
pub struct ScounterenRaw {
    data: B64
}


#[bitfield]
pub struct MtvalRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter21Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter24Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter17hRaw {
    data: B64
}


#[bitfield]
pub struct HcounterenRaw {
    data: B64
}


#[bitfield]
pub struct HvipRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter10Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent27Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter5hRaw {
    data: B64
}


#[bitfield]
pub struct MidelegRaw {
    data: B64
}


#[bitfield]
pub struct HtimedeltahRaw {
    data: B64
}


#[bitfield]
pub struct VscauseRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent28Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent11Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter18hRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr4Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter16Raw {
    data: B64
}


#[bitfield]
pub struct MconfigptrRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent8Raw {
    data: B64
}


#[bitfield]
pub struct DscratchRaw {
    data: B64
}


#[bitfield]
pub struct SedelegRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent19Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter24Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter7hRaw {
    data: B64
}


#[bitfield]
pub struct VstartRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr10Raw {
    data: B64
}


#[bitfield]
pub struct MedelegRaw {
    data: B64
}


#[bitfield]
pub struct MtvtRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter14Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter26hRaw {
    data: B64
}


#[bitfield]
pub struct FcsrRaw {
    data: B64
}


#[bitfield]
pub struct MnxtiRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent16Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter19hRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter7Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter23Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter7Raw {
    data: B64
}


#[bitfield]
pub struct MinstretRaw {
    data: B64
}


#[bitfield]
pub struct BsipRaw {
    data: B64
}


#[bitfield]
pub struct MscratchcswRaw {
    data: B64
}


#[bitfield]
pub struct ScauseRaw {
    exception_code: ScauseExceptionCode,
    interrupt_code: ScauseInterruptCode,
    interrupt: B1,
}

#[derive(BitfieldSpecifier)]
#[bits = 63]
pub enum ScauseExceptionCode {
    RESERVED36 = 36, RESERVED38 = 38, RESERVED47 = 47, RESERVED16 = 16, RESERVED59 = 59, RESERVED43 = 43, RESERVED56 = 56, RESERVED44 = 44, ENVIRONMENT_CALL_FROM_U_MODE = 8, RESERVED18 = 18, RESERVED19 = 19, STORE_AMO_ACCESS_FAULT = 7, RESERVED60 = 60, CUSTOM29 = 29, RESERVED37 = 37, INSTRUCTION_ADDRESS_MISALIGNED = 0, RESERVED17 = 17, RESERVED48 = 48, INSTRUCTION_PAGE_FAULT = 12, RESERVED52 = 52, LOAD_PAGE_FAULT = 13, RESERVED33 = 33, RESERVED63 = 63, STORE_AMO_ADDRESS_MISALIGNED = 6, RESERVED57 = 57, INSTRUCTION_ACCESS_FAULT = 1, RESERVED22 = 22, CUSTOM27 = 27, RESERVED35 = 35, RESERVED42 = 42, RESERVED55 = 55, RESERVED10 = 10, RESERVED53 = 53, RESERVED58 = 58, CUSTOM25 = 25, STORE_AMO_PAGE_FAULT = 15, RESERVED23 = 23, RESERVED62 = 62, RESERVED34 = 34, RESERVED50 = 50, RESERVED21 = 21, CUSTOM24 = 24, RESERVED41 = 41, CUSTOM28 = 28, RESERVED39 = 39, CUSTOM31 = 31, RESERVED32 = 32, RESERVED45 = 45, ILLEGAL_INSTRUCTION = 2, RESERVED51 = 51, RESERVED40 = 40, RESERVED14 = 14, RESERVED46 = 46, RESERVED49 = 49, CUSTOM26 = 26, ENVIRONMENT_CALL_FROM_S_MODE = 9, RESERVED61 = 61, CUSTOM30 = 30, LOAD_ACCESS_FAULT = 5, RESERVED20 = 20, LOAD_ADDRESS_MISALIGNED = 4, BREAKPOINT = 3, RESERVED54 = 54, 
}

#[derive(BitfieldSpecifier)]
#[bits = 63]
pub enum ScauseInterruptCode {
    UTI = 4, STI = 5, USI = 0, UEI = 8, SEI = 9, SSI = 1, 
}


#[bitfield]
pub struct Mhpmevent17Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter16hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent6Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter25Raw {
    data: B64
}


#[bitfield]
pub struct StvecRaw {
    mode: B2,
    base: B62,
}


#[bitfield]
pub struct Hpmcounter20Raw {
    data: B64
}


#[bitfield]
pub struct HidelegRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter15Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter9hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter20hRaw {
    data: B64
}


#[bitfield]
pub struct HtvalRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent31Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent4Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter30Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter8hRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter18Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter14hRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter22hRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter8Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter13hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter25hRaw {
    data: B64
}


#[bitfield]
pub struct MtimecmpRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter9Raw {
    data: B64
}


#[bitfield]
pub struct McycleRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter10hRaw {
    data: B64
}


#[bitfield]
pub struct MseccfghRaw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr5Raw {
    data: B64
}


#[bitfield]
pub struct ScontextRaw {
    data: B64
}


#[bitfield]
pub struct VlRaw {
    data: B64
}


#[bitfield]
pub struct MstatushRaw {
    #[skip] __: B4,
    sbe: B1,
    mbe: B1,
    #[skip] __: B58
}


#[bitfield]
pub struct Mhpmevent24Raw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter29Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter19Raw {
    data: B64
}


#[bitfield]
pub struct MarchidRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter26Raw {
    data: B64
}


#[bitfield]
pub struct Pmpcfg3Raw {
    data: B64
}


#[bitfield]
pub struct MipRaw {
    usi: B1,
    ssi: B1,
    #[skip] __: B1,
    msi: B1,
    uti: B1,
    sti: B1,
    #[skip] __: B1,
    mti: B1,
    uei: B1,
    sei: B1,
    #[skip] __: B1,
    mei: B1,
    #[skip] __: B4,
    platform_defined16: B1,
    platform_defined17: B1,
    platform_defined18: B1,
    platform_defined19: B1,
    platform_defined20: B1,
    platform_defined21: B1,
    platform_defined22: B1,
    platform_defined23: B1,
    platform_defined24: B1,
    platform_defined25: B1,
    platform_defined26: B1,
    platform_defined27: B1,
    platform_defined28: B1,
    platform_defined29: B1,
    platform_defined30: B1,
    platform_defined31: B1,
    #[skip] __: B32
}


#[bitfield]
pub struct MboundRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmevent13Raw {
    data: B64
}


#[bitfield]
pub struct Pmpaddr9Raw {
    data: B64
}


#[bitfield]
pub struct TselectRaw {
    data: B64
}


#[bitfield]
pub struct UcauseRaw {
    interrupt_code: UcauseInterruptCode,
    exception_code: UcauseExceptionCode,
    interrupt: B1,
}

#[derive(BitfieldSpecifier)]
#[bits = 63]
pub enum UcauseInterruptCode {
    UTI = 4, USI = 0, UEI = 8, 
}

#[derive(BitfieldSpecifier)]
#[bits = 63]
pub enum UcauseExceptionCode {
    RESERVED61 = 61, BREAKPOINT = 3, RESERVED44 = 44, RESERVED57 = 57, STORE_AMO_ACCESS_FAULT = 7, RESERVED39 = 39, RESERVED10 = 10, RESERVED55 = 55, RESERVED63 = 63, CUSTOM24 = 24, RESERVED34 = 34, RESERVED53 = 53, ILLEGAL_INSTRUCTION = 2, RESERVED16 = 16, RESERVED62 = 62, ENVIRONMENT_CALL_FROM_U_MODE = 8, RESERVED37 = 37, RESERVED58 = 58, RESERVED51 = 51, CUSTOM27 = 27, RESERVED21 = 21, INSTRUCTION_ADDRESS_MISALIGNED = 0, RESERVED42 = 42, CUSTOM28 = 28, RESERVED32 = 32, RESERVED41 = 41, RESERVED23 = 23, RESERVED35 = 35, RESERVED56 = 56, RESERVED38 = 38, RESERVED17 = 17, RESERVED48 = 48, INSTRUCTION_ACCESS_FAULT = 1, RESERVED14 = 14, LOAD_ACCESS_FAULT = 5, RESERVED33 = 33, RESERVED20 = 20, CUSTOM26 = 26, RESERVED59 = 59, RESERVED52 = 52, RESERVED43 = 43, RESERVED46 = 46, RESERVED18 = 18, LOAD_PAGE_FAULT = 13, RESERVED50 = 50, RESERVED54 = 54, RESERVED22 = 22, CUSTOM30 = 30, RESERVED45 = 45, STORE_AMO_ADDRESS_MISALIGNED = 6, RESERVED47 = 47, STORE_AMO_PAGE_FAULT = 15, RESERVED36 = 36, ENVIRONMENT_CALL_FROM_S_MODE = 9, RESERVED49 = 49, LOAD_ADDRESS_MISALIGNED = 4, INSTRUCTION_PAGE_FAULT = 12, RESERVED40 = 40, RESERVED19 = 19, RESERVED60 = 60, CUSTOM31 = 31, CUSTOM25 = 25, CUSTOM29 = 29, 
}


#[bitfield]
pub struct MieRaw {
    usi: B1,
    ssi: B1,
    #[skip] __: B1,
    msi: B1,
    uti: B1,
    sti: B1,
    #[skip] __: B1,
    mti: B1,
    uei: B1,
    sei: B1,
    #[skip] __: B1,
    mei: B1,
    #[skip] __: B4,
    platform_defined16: B1,
    platform_defined17: B1,
    platform_defined18: B1,
    platform_defined19: B1,
    platform_defined20: B1,
    platform_defined21: B1,
    platform_defined22: B1,
    platform_defined23: B1,
    platform_defined24: B1,
    platform_defined25: B1,
    platform_defined26: B1,
    platform_defined27: B1,
    platform_defined28: B1,
    platform_defined29: B1,
    platform_defined30: B1,
    platform_defined31: B1,
    #[skip] __: B32
}


#[bitfield]
pub struct BsscratchRaw {
    data: B64
}


#[bitfield]
pub struct MepcRaw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter27hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter29hRaw {
    data: B64
}


#[bitfield]
pub struct Mhpmcounter12Raw {
    data: B64
}


#[bitfield]
pub struct Hpmcounter30hRaw {
    data: B64
}


#[bitfield]
pub struct Tdata1Raw {
    data: B64
}


#[bitfield]
pub struct SstatusRaw {
    #[skip] __: B2,
    sie: B1,
    #[skip] __: B2,
    spie: B1,
    #[skip] __: B2,
    spp: B1,
    #[skip] __: B23,
    uxl: B2,
    #[skip] __: B30
}


#[bitfield]
pub struct Mhpmcounter6Raw {
    data: B64
}


