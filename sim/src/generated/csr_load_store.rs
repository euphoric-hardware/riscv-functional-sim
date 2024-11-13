use crate::{csrs::Csrs, cpu::{Error, Result}};

impl Csrs {
    pub const FFLAGS: u64 = 0x001;
    pub const FRM: u64 = 0x002;
    pub const FCSR: u64 = 0x003;
    pub const VSTART: u64 = 0x008;
    pub const VXSAT: u64 = 0x009;
    pub const VXRM: u64 = 0x00A;
    pub const VCSR: u64 = 0x00F;
    pub const SSP: u64 = 0x011;
    pub const SEED: u64 = 0x015;
    pub const JVT: u64 = 0x017;
    pub const CYCLE: u64 = 0xC00;
    pub const TIME: u64 = 0xC01;
    pub const INSTRET: u64 = 0xC02;
    pub const HPMCOUNTER3: u64 = 0xC03;
    pub const HPMCOUNTER4: u64 = 0xC04;
    pub const HPMCOUNTER5: u64 = 0xC05;
    pub const HPMCOUNTER6: u64 = 0xC06;
    pub const HPMCOUNTER7: u64 = 0xC07;
    pub const HPMCOUNTER8: u64 = 0xC08;
    pub const HPMCOUNTER9: u64 = 0xC09;
    pub const HPMCOUNTER10: u64 = 0xC0A;
    pub const HPMCOUNTER11: u64 = 0xC0B;
    pub const HPMCOUNTER12: u64 = 0xC0C;
    pub const HPMCOUNTER13: u64 = 0xC0D;
    pub const HPMCOUNTER14: u64 = 0xC0E;
    pub const HPMCOUNTER15: u64 = 0xC0F;
    pub const HPMCOUNTER16: u64 = 0xC10;
    pub const HPMCOUNTER17: u64 = 0xC11;
    pub const HPMCOUNTER18: u64 = 0xC12;
    pub const HPMCOUNTER19: u64 = 0xC13;
    pub const HPMCOUNTER20: u64 = 0xC14;
    pub const HPMCOUNTER21: u64 = 0xC15;
    pub const HPMCOUNTER22: u64 = 0xC16;
    pub const HPMCOUNTER23: u64 = 0xC17;
    pub const HPMCOUNTER24: u64 = 0xC18;
    pub const HPMCOUNTER25: u64 = 0xC19;
    pub const HPMCOUNTER26: u64 = 0xC1A;
    pub const HPMCOUNTER27: u64 = 0xC1B;
    pub const HPMCOUNTER28: u64 = 0xC1C;
    pub const HPMCOUNTER29: u64 = 0xC1D;
    pub const HPMCOUNTER30: u64 = 0xC1E;
    pub const HPMCOUNTER31: u64 = 0xC1F;
    pub const VL: u64 = 0xC20;
    pub const VTYPE: u64 = 0xC21;
    pub const VLENB: u64 = 0xC22;
    pub const SSTATUS: u64 = 0x100;
    pub const SEDELEG: u64 = 0x102;
    pub const SIDELEG: u64 = 0x103;
    pub const SIE: u64 = 0x104;
    pub const STVEC: u64 = 0x105;
    pub const SCOUNTEREN: u64 = 0x106;
    pub const SENVCFG: u64 = 0x10A;
    pub const SSTATEEN0: u64 = 0x10C;
    pub const SSTATEEN1: u64 = 0x10D;
    pub const SSTATEEN2: u64 = 0x10E;
    pub const SSTATEEN3: u64 = 0x10F;
    pub const SCOUNTINHIBIT: u64 = 0x120;
    pub const SSCRATCH: u64 = 0x140;
    pub const SEPC: u64 = 0x141;
    pub const SCAUSE: u64 = 0x142;
    pub const STVAL: u64 = 0x143;
    pub const SIP: u64 = 0x144;
    pub const STIMECMP: u64 = 0x14D;
    pub const SCTRCTL: u64 = 0x14E;
    pub const SCTRSTATUS: u64 = 0x14F;
    pub const SISELECT: u64 = 0x150;
    pub const SIREG: u64 = 0x151;
    pub const SIREG2: u64 = 0x152;
    pub const SIREG3: u64 = 0x153;
    pub const SIREG4: u64 = 0x155;
    pub const SIREG5: u64 = 0x156;
    pub const SIREG6: u64 = 0x157;
    pub const STOPEI: u64 = 0x15C;
    pub const SCTRDEPTH: u64 = 0x15F;
    pub const SATP: u64 = 0x180;
    pub const SRMCFG: u64 = 0x181;
    pub const SCONTEXT: u64 = 0x5A8;
    pub const VSSTATUS: u64 = 0x200;
    pub const VSIE: u64 = 0x204;
    pub const VSTVEC: u64 = 0x205;
    pub const VSSCRATCH: u64 = 0x240;
    pub const VSEPC: u64 = 0x241;
    pub const VSCAUSE: u64 = 0x242;
    pub const VSTVAL: u64 = 0x243;
    pub const VSIP: u64 = 0x244;
    pub const VSTIMECMP: u64 = 0x24D;
    pub const VSCTRCTL: u64 = 0x24E;
    pub const VSISELECT: u64 = 0x250;
    pub const VSIREG: u64 = 0x251;
    pub const VSIREG2: u64 = 0x252;
    pub const VSIREG3: u64 = 0x253;
    pub const VSIREG4: u64 = 0x255;
    pub const VSIREG5: u64 = 0x256;
    pub const VSIREG6: u64 = 0x257;
    pub const VSTOPEI: u64 = 0x25C;
    pub const VSATP: u64 = 0x280;
    pub const HSTATUS: u64 = 0x600;
    pub const HEDELEG: u64 = 0x602;
    pub const HIDELEG: u64 = 0x603;
    pub const HIE: u64 = 0x604;
    pub const HTIMEDELTA: u64 = 0x605;
    pub const HCOUNTEREN: u64 = 0x606;
    pub const HGEIE: u64 = 0x607;
    pub const HVIEN: u64 = 0x608;
    pub const HVICTL: u64 = 0x609;
    pub const HENVCFG: u64 = 0x60A;
    pub const HSTATEEN0: u64 = 0x60C;
    pub const HSTATEEN1: u64 = 0x60D;
    pub const HSTATEEN2: u64 = 0x60E;
    pub const HSTATEEN3: u64 = 0x60F;
    pub const HTVAL: u64 = 0x643;
    pub const HIP: u64 = 0x644;
    pub const HVIP: u64 = 0x645;
    pub const HVIPRIO1: u64 = 0x646;
    pub const HVIPRIO2: u64 = 0x647;
    pub const HTINST: u64 = 0x64A;
    pub const HGATP: u64 = 0x680;
    pub const HCONTEXT: u64 = 0x6A8;
    pub const HGEIP: u64 = 0xE12;
    pub const VSTOPI: u64 = 0xEB0;
    pub const SCOUNTOVF: u64 = 0xDA0;
    pub const STOPI: u64 = 0xDB0;
    pub const UTVT: u64 = 0x007;
    pub const UNXTI: u64 = 0x045;
    pub const UINTSTATUS: u64 = 0x046;
    pub const USCRATCHCSW: u64 = 0x048;
    pub const USCRATCHCSWL: u64 = 0x049;
    pub const STVT: u64 = 0x107;
    pub const SNXTI: u64 = 0x145;
    pub const SINTSTATUS: u64 = 0x146;
    pub const SSCRATCHCSW: u64 = 0x148;
    pub const SSCRATCHCSWL: u64 = 0x149;
    pub const MTVT: u64 = 0x307;
    pub const MNXTI: u64 = 0x345;
    pub const MINTSTATUS: u64 = 0x346;
    pub const MSCRATCHCSW: u64 = 0x348;
    pub const MSCRATCHCSWL: u64 = 0x349;
    pub const MSTATUS: u64 = 0x300;
    pub const MISA: u64 = 0x301;
    pub const MEDELEG: u64 = 0x302;
    pub const MIDELEG: u64 = 0x303;
    pub const MIE: u64 = 0x304;
    pub const MTVEC: u64 = 0x305;
    pub const MCOUNTEREN: u64 = 0x306;
    pub const MVIEN: u64 = 0x308;
    pub const MVIP: u64 = 0x309;
    pub const MENVCFG: u64 = 0x30a;
    pub const MSTATEEN0: u64 = 0x30C;
    pub const MSTATEEN1: u64 = 0x30D;
    pub const MSTATEEN2: u64 = 0x30E;
    pub const MSTATEEN3: u64 = 0x30F;
    pub const MCOUNTINHIBIT: u64 = 0x320;
    pub const MSCRATCH: u64 = 0x340;
    pub const MEPC: u64 = 0x341;
    pub const MCAUSE: u64 = 0x342;
    pub const MTVAL: u64 = 0x343;
    pub const MIP: u64 = 0x344;
    pub const MTINST: u64 = 0x34a;
    pub const MTVAL2: u64 = 0x34b;
    pub const MCTRCTL: u64 = 0x34E;
    pub const MISELECT: u64 = 0x350;
    pub const MIREG: u64 = 0x351;
    pub const MIREG2: u64 = 0x352;
    pub const MIREG3: u64 = 0x353;
    pub const MIREG4: u64 = 0x355;
    pub const MIREG5: u64 = 0x356;
    pub const MIREG6: u64 = 0x357;
    pub const MTOPEI: u64 = 0x35c;
    pub const PMPCFG0: u64 = 0x3a0;
    pub const PMPCFG1: u64 = 0x3a1;
    pub const PMPCFG2: u64 = 0x3a2;
    pub const PMPCFG3: u64 = 0x3a3;
    pub const PMPCFG4: u64 = 0x3a4;
    pub const PMPCFG5: u64 = 0x3a5;
    pub const PMPCFG6: u64 = 0x3a6;
    pub const PMPCFG7: u64 = 0x3a7;
    pub const PMPCFG8: u64 = 0x3a8;
    pub const PMPCFG9: u64 = 0x3a9;
    pub const PMPCFG10: u64 = 0x3aa;
    pub const PMPCFG11: u64 = 0x3ab;
    pub const PMPCFG12: u64 = 0x3ac;
    pub const PMPCFG13: u64 = 0x3ad;
    pub const PMPCFG14: u64 = 0x3ae;
    pub const PMPCFG15: u64 = 0x3af;
    pub const PMPADDR0: u64 = 0x3b0;
    pub const PMPADDR1: u64 = 0x3b1;
    pub const PMPADDR2: u64 = 0x3b2;
    pub const PMPADDR3: u64 = 0x3b3;
    pub const PMPADDR4: u64 = 0x3b4;
    pub const PMPADDR5: u64 = 0x3b5;
    pub const PMPADDR6: u64 = 0x3b6;
    pub const PMPADDR7: u64 = 0x3b7;
    pub const PMPADDR8: u64 = 0x3b8;
    pub const PMPADDR9: u64 = 0x3b9;
    pub const PMPADDR10: u64 = 0x3ba;
    pub const PMPADDR11: u64 = 0x3bb;
    pub const PMPADDR12: u64 = 0x3bc;
    pub const PMPADDR13: u64 = 0x3bd;
    pub const PMPADDR14: u64 = 0x3be;
    pub const PMPADDR15: u64 = 0x3bf;
    pub const PMPADDR16: u64 = 0x3c0;
    pub const PMPADDR17: u64 = 0x3c1;
    pub const PMPADDR18: u64 = 0x3c2;
    pub const PMPADDR19: u64 = 0x3c3;
    pub const PMPADDR20: u64 = 0x3c4;
    pub const PMPADDR21: u64 = 0x3c5;
    pub const PMPADDR22: u64 = 0x3c6;
    pub const PMPADDR23: u64 = 0x3c7;
    pub const PMPADDR24: u64 = 0x3c8;
    pub const PMPADDR25: u64 = 0x3c9;
    pub const PMPADDR26: u64 = 0x3ca;
    pub const PMPADDR27: u64 = 0x3cb;
    pub const PMPADDR28: u64 = 0x3cc;
    pub const PMPADDR29: u64 = 0x3cd;
    pub const PMPADDR30: u64 = 0x3ce;
    pub const PMPADDR31: u64 = 0x3cf;
    pub const PMPADDR32: u64 = 0x3d0;
    pub const PMPADDR33: u64 = 0x3d1;
    pub const PMPADDR34: u64 = 0x3d2;
    pub const PMPADDR35: u64 = 0x3d3;
    pub const PMPADDR36: u64 = 0x3d4;
    pub const PMPADDR37: u64 = 0x3d5;
    pub const PMPADDR38: u64 = 0x3d6;
    pub const PMPADDR39: u64 = 0x3d7;
    pub const PMPADDR40: u64 = 0x3d8;
    pub const PMPADDR41: u64 = 0x3d9;
    pub const PMPADDR42: u64 = 0x3da;
    pub const PMPADDR43: u64 = 0x3db;
    pub const PMPADDR44: u64 = 0x3dc;
    pub const PMPADDR45: u64 = 0x3dd;
    pub const PMPADDR46: u64 = 0x3de;
    pub const PMPADDR47: u64 = 0x3df;
    pub const PMPADDR48: u64 = 0x3e0;
    pub const PMPADDR49: u64 = 0x3e1;
    pub const PMPADDR50: u64 = 0x3e2;
    pub const PMPADDR51: u64 = 0x3e3;
    pub const PMPADDR52: u64 = 0x3e4;
    pub const PMPADDR53: u64 = 0x3e5;
    pub const PMPADDR54: u64 = 0x3e6;
    pub const PMPADDR55: u64 = 0x3e7;
    pub const PMPADDR56: u64 = 0x3e8;
    pub const PMPADDR57: u64 = 0x3e9;
    pub const PMPADDR58: u64 = 0x3ea;
    pub const PMPADDR59: u64 = 0x3eb;
    pub const PMPADDR60: u64 = 0x3ec;
    pub const PMPADDR61: u64 = 0x3ed;
    pub const PMPADDR62: u64 = 0x3ee;
    pub const PMPADDR63: u64 = 0x3ef;
    pub const MSECCFG: u64 = 0x747;
    pub const TSELECT: u64 = 0x7a0;
    pub const TDATA1: u64 = 0x7a1;
    pub const TDATA2: u64 = 0x7a2;
    pub const TDATA3: u64 = 0x7a3;
    pub const TINFO: u64 = 0x7a4;
    pub const TCONTROL: u64 = 0x7a5;
    pub const MCONTEXT: u64 = 0x7a8;
    pub const MSCONTEXT: u64 = 0x7aa;
    pub const DCSR: u64 = 0x7b0;
    pub const DPC: u64 = 0x7b1;
    pub const DSCRATCH0: u64 = 0x7b2;
    pub const DSCRATCH1: u64 = 0x7b3;
    pub const MCYCLE: u64 = 0xB00;
    pub const MINSTRET: u64 = 0xB02;
    pub const MHPMCOUNTER3: u64 = 0xB03;
    pub const MHPMCOUNTER4: u64 = 0xB04;
    pub const MHPMCOUNTER5: u64 = 0xB05;
    pub const MHPMCOUNTER6: u64 = 0xB06;
    pub const MHPMCOUNTER7: u64 = 0xB07;
    pub const MHPMCOUNTER8: u64 = 0xB08;
    pub const MHPMCOUNTER9: u64 = 0xB09;
    pub const MHPMCOUNTER10: u64 = 0xB0A;
    pub const MHPMCOUNTER11: u64 = 0xB0B;
    pub const MHPMCOUNTER12: u64 = 0xB0C;
    pub const MHPMCOUNTER13: u64 = 0xB0D;
    pub const MHPMCOUNTER14: u64 = 0xB0E;
    pub const MHPMCOUNTER15: u64 = 0xB0F;
    pub const MHPMCOUNTER16: u64 = 0xB10;
    pub const MHPMCOUNTER17: u64 = 0xB11;
    pub const MHPMCOUNTER18: u64 = 0xB12;
    pub const MHPMCOUNTER19: u64 = 0xB13;
    pub const MHPMCOUNTER20: u64 = 0xB14;
    pub const MHPMCOUNTER21: u64 = 0xB15;
    pub const MHPMCOUNTER22: u64 = 0xB16;
    pub const MHPMCOUNTER23: u64 = 0xB17;
    pub const MHPMCOUNTER24: u64 = 0xB18;
    pub const MHPMCOUNTER25: u64 = 0xB19;
    pub const MHPMCOUNTER26: u64 = 0xB1A;
    pub const MHPMCOUNTER27: u64 = 0xB1B;
    pub const MHPMCOUNTER28: u64 = 0xB1C;
    pub const MHPMCOUNTER29: u64 = 0xB1D;
    pub const MHPMCOUNTER30: u64 = 0xB1E;
    pub const MHPMCOUNTER31: u64 = 0xB1F;
    pub const MCYCLECFG: u64 = 0x321;
    pub const MINSTRETCFG: u64 = 0x322;
    pub const MHPMEVENT3: u64 = 0x323;
    pub const MHPMEVENT4: u64 = 0x324;
    pub const MHPMEVENT5: u64 = 0x325;
    pub const MHPMEVENT6: u64 = 0x326;
    pub const MHPMEVENT7: u64 = 0x327;
    pub const MHPMEVENT8: u64 = 0x328;
    pub const MHPMEVENT9: u64 = 0x329;
    pub const MHPMEVENT10: u64 = 0x32A;
    pub const MHPMEVENT11: u64 = 0x32B;
    pub const MHPMEVENT12: u64 = 0x32C;
    pub const MHPMEVENT13: u64 = 0x32D;
    pub const MHPMEVENT14: u64 = 0x32E;
    pub const MHPMEVENT15: u64 = 0x32F;
    pub const MHPMEVENT16: u64 = 0x330;
    pub const MHPMEVENT17: u64 = 0x331;
    pub const MHPMEVENT18: u64 = 0x332;
    pub const MHPMEVENT19: u64 = 0x333;
    pub const MHPMEVENT20: u64 = 0x334;
    pub const MHPMEVENT21: u64 = 0x335;
    pub const MHPMEVENT22: u64 = 0x336;
    pub const MHPMEVENT23: u64 = 0x337;
    pub const MHPMEVENT24: u64 = 0x338;
    pub const MHPMEVENT25: u64 = 0x339;
    pub const MHPMEVENT26: u64 = 0x33A;
    pub const MHPMEVENT27: u64 = 0x33B;
    pub const MHPMEVENT28: u64 = 0x33C;
    pub const MHPMEVENT29: u64 = 0x33D;
    pub const MHPMEVENT30: u64 = 0x33E;
    pub const MHPMEVENT31: u64 = 0x33F;
    pub const MVENDORID: u64 = 0xF11;
    pub const MARCHID: u64 = 0xF12;
    pub const MIMPID: u64 = 0xF13;
    pub const MHARTID: u64 = 0xF14;
    pub const MCONFIGPTR: u64 = 0xF15;
    pub const MTOPI: u64 = 0xFB0;

    fn load(&self, address: u64) -> Result<u64> {
        match address {
            Self::FFLAGS => Ok(self.regs[Self::FFLAGS as usize]),
            Self::FRM => Ok(self.regs[Self::FRM as usize]),
            Self::FCSR => Ok(self.regs[Self::FCSR as usize]),
            Self::VSTART => Ok(self.regs[Self::VSTART as usize]),
            Self::VXSAT => Ok(self.regs[Self::VXSAT as usize]),
            Self::VXRM => Ok(self.regs[Self::VXRM as usize]),
            Self::VCSR => Ok(self.regs[Self::VCSR as usize]),
            Self::SSP => Ok(self.regs[Self::SSP as usize]),
            Self::SEED => Ok(self.regs[Self::SEED as usize]),
            Self::JVT => Ok(self.regs[Self::JVT as usize]),
            Self::CYCLE => Ok(self.regs[Self::CYCLE as usize]),
            Self::TIME => Ok(self.regs[Self::TIME as usize]),
            Self::INSTRET => Ok(self.regs[Self::INSTRET as usize]),
            Self::HPMCOUNTER3 => Ok(self.regs[Self::HPMCOUNTER3 as usize]),
            Self::HPMCOUNTER4 => Ok(self.regs[Self::HPMCOUNTER4 as usize]),
            Self::HPMCOUNTER5 => Ok(self.regs[Self::HPMCOUNTER5 as usize]),
            Self::HPMCOUNTER6 => Ok(self.regs[Self::HPMCOUNTER6 as usize]),
            Self::HPMCOUNTER7 => Ok(self.regs[Self::HPMCOUNTER7 as usize]),
            Self::HPMCOUNTER8 => Ok(self.regs[Self::HPMCOUNTER8 as usize]),
            Self::HPMCOUNTER9 => Ok(self.regs[Self::HPMCOUNTER9 as usize]),
            Self::HPMCOUNTER10 => Ok(self.regs[Self::HPMCOUNTER10 as usize]),
            Self::HPMCOUNTER11 => Ok(self.regs[Self::HPMCOUNTER11 as usize]),
            Self::HPMCOUNTER12 => Ok(self.regs[Self::HPMCOUNTER12 as usize]),
            Self::HPMCOUNTER13 => Ok(self.regs[Self::HPMCOUNTER13 as usize]),
            Self::HPMCOUNTER14 => Ok(self.regs[Self::HPMCOUNTER14 as usize]),
            Self::HPMCOUNTER15 => Ok(self.regs[Self::HPMCOUNTER15 as usize]),
            Self::HPMCOUNTER16 => Ok(self.regs[Self::HPMCOUNTER16 as usize]),
            Self::HPMCOUNTER17 => Ok(self.regs[Self::HPMCOUNTER17 as usize]),
            Self::HPMCOUNTER18 => Ok(self.regs[Self::HPMCOUNTER18 as usize]),
            Self::HPMCOUNTER19 => Ok(self.regs[Self::HPMCOUNTER19 as usize]),
            Self::HPMCOUNTER20 => Ok(self.regs[Self::HPMCOUNTER20 as usize]),
            Self::HPMCOUNTER21 => Ok(self.regs[Self::HPMCOUNTER21 as usize]),
            Self::HPMCOUNTER22 => Ok(self.regs[Self::HPMCOUNTER22 as usize]),
            Self::HPMCOUNTER23 => Ok(self.regs[Self::HPMCOUNTER23 as usize]),
            Self::HPMCOUNTER24 => Ok(self.regs[Self::HPMCOUNTER24 as usize]),
            Self::HPMCOUNTER25 => Ok(self.regs[Self::HPMCOUNTER25 as usize]),
            Self::HPMCOUNTER26 => Ok(self.regs[Self::HPMCOUNTER26 as usize]),
            Self::HPMCOUNTER27 => Ok(self.regs[Self::HPMCOUNTER27 as usize]),
            Self::HPMCOUNTER28 => Ok(self.regs[Self::HPMCOUNTER28 as usize]),
            Self::HPMCOUNTER29 => Ok(self.regs[Self::HPMCOUNTER29 as usize]),
            Self::HPMCOUNTER30 => Ok(self.regs[Self::HPMCOUNTER30 as usize]),
            Self::HPMCOUNTER31 => Ok(self.regs[Self::HPMCOUNTER31 as usize]),
            Self::VL => Ok(self.regs[Self::VL as usize]),
            Self::VTYPE => Ok(self.regs[Self::VTYPE as usize]),
            Self::VLENB => Ok(self.regs[Self::VLENB as usize]),
            Self::SSTATUS => Ok(self.regs[Self::SSTATUS as usize]),
            Self::SEDELEG => Ok(self.regs[Self::SEDELEG as usize]),
            Self::SIDELEG => Ok(self.regs[Self::SIDELEG as usize]),
            Self::SIE => Ok(self.regs[Self::SIE as usize]),
            Self::STVEC => Ok(self.regs[Self::STVEC as usize]),
            Self::SCOUNTEREN => Ok(self.regs[Self::SCOUNTEREN as usize]),
            Self::SENVCFG => Ok(self.regs[Self::SENVCFG as usize]),
            Self::SSTATEEN0 => Ok(self.regs[Self::SSTATEEN0 as usize]),
            Self::SSTATEEN1 => Ok(self.regs[Self::SSTATEEN1 as usize]),
            Self::SSTATEEN2 => Ok(self.regs[Self::SSTATEEN2 as usize]),
            Self::SSTATEEN3 => Ok(self.regs[Self::SSTATEEN3 as usize]),
            Self::SCOUNTINHIBIT => Ok(self.regs[Self::SCOUNTINHIBIT as usize]),
            Self::SSCRATCH => Ok(self.regs[Self::SSCRATCH as usize]),
            Self::SEPC => Ok(self.regs[Self::SEPC as usize]),
            Self::SCAUSE => Ok(self.regs[Self::SCAUSE as usize]),
            Self::STVAL => Ok(self.regs[Self::STVAL as usize]),
            Self::SIP => Ok(self.regs[Self::SIP as usize]),
            Self::STIMECMP => Ok(self.regs[Self::STIMECMP as usize]),
            Self::SCTRCTL => Ok(self.regs[Self::SCTRCTL as usize]),
            Self::SCTRSTATUS => Ok(self.regs[Self::SCTRSTATUS as usize]),
            Self::SISELECT => Ok(self.regs[Self::SISELECT as usize]),
            Self::SIREG => Ok(self.regs[Self::SIREG as usize]),
            Self::SIREG2 => Ok(self.regs[Self::SIREG2 as usize]),
            Self::SIREG3 => Ok(self.regs[Self::SIREG3 as usize]),
            Self::SIREG4 => Ok(self.regs[Self::SIREG4 as usize]),
            Self::SIREG5 => Ok(self.regs[Self::SIREG5 as usize]),
            Self::SIREG6 => Ok(self.regs[Self::SIREG6 as usize]),
            Self::STOPEI => Ok(self.regs[Self::STOPEI as usize]),
            Self::SCTRDEPTH => Ok(self.regs[Self::SCTRDEPTH as usize]),
            Self::SATP => Ok(self.regs[Self::SATP as usize]),
            Self::SRMCFG => Ok(self.regs[Self::SRMCFG as usize]),
            Self::SCONTEXT => Ok(self.regs[Self::SCONTEXT as usize]),
            Self::VSSTATUS => Ok(self.regs[Self::VSSTATUS as usize]),
            Self::VSIE => Ok(self.regs[Self::VSIE as usize]),
            Self::VSTVEC => Ok(self.regs[Self::VSTVEC as usize]),
            Self::VSSCRATCH => Ok(self.regs[Self::VSSCRATCH as usize]),
            Self::VSEPC => Ok(self.regs[Self::VSEPC as usize]),
            Self::VSCAUSE => Ok(self.regs[Self::VSCAUSE as usize]),
            Self::VSTVAL => Ok(self.regs[Self::VSTVAL as usize]),
            Self::VSIP => Ok(self.regs[Self::VSIP as usize]),
            Self::VSTIMECMP => Ok(self.regs[Self::VSTIMECMP as usize]),
            Self::VSCTRCTL => Ok(self.regs[Self::VSCTRCTL as usize]),
            Self::VSISELECT => Ok(self.regs[Self::VSISELECT as usize]),
            Self::VSIREG => Ok(self.regs[Self::VSIREG as usize]),
            Self::VSIREG2 => Ok(self.regs[Self::VSIREG2 as usize]),
            Self::VSIREG3 => Ok(self.regs[Self::VSIREG3 as usize]),
            Self::VSIREG4 => Ok(self.regs[Self::VSIREG4 as usize]),
            Self::VSIREG5 => Ok(self.regs[Self::VSIREG5 as usize]),
            Self::VSIREG6 => Ok(self.regs[Self::VSIREG6 as usize]),
            Self::VSTOPEI => Ok(self.regs[Self::VSTOPEI as usize]),
            Self::VSATP => Ok(self.regs[Self::VSATP as usize]),
            Self::HSTATUS => Ok(self.regs[Self::HSTATUS as usize]),
            Self::HEDELEG => Ok(self.regs[Self::HEDELEG as usize]),
            Self::HIDELEG => Ok(self.regs[Self::HIDELEG as usize]),
            Self::HIE => Ok(self.regs[Self::HIE as usize]),
            Self::HTIMEDELTA => Ok(self.regs[Self::HTIMEDELTA as usize]),
            Self::HCOUNTEREN => Ok(self.regs[Self::HCOUNTEREN as usize]),
            Self::HGEIE => Ok(self.regs[Self::HGEIE as usize]),
            Self::HVIEN => Ok(self.regs[Self::HVIEN as usize]),
            Self::HVICTL => Ok(self.regs[Self::HVICTL as usize]),
            Self::HENVCFG => Ok(self.regs[Self::HENVCFG as usize]),
            Self::HSTATEEN0 => Ok(self.regs[Self::HSTATEEN0 as usize]),
            Self::HSTATEEN1 => Ok(self.regs[Self::HSTATEEN1 as usize]),
            Self::HSTATEEN2 => Ok(self.regs[Self::HSTATEEN2 as usize]),
            Self::HSTATEEN3 => Ok(self.regs[Self::HSTATEEN3 as usize]),
            Self::HTVAL => Ok(self.regs[Self::HTVAL as usize]),
            Self::HIP => Ok(self.regs[Self::HIP as usize]),
            Self::HVIP => Ok(self.regs[Self::HVIP as usize]),
            Self::HVIPRIO1 => Ok(self.regs[Self::HVIPRIO1 as usize]),
            Self::HVIPRIO2 => Ok(self.regs[Self::HVIPRIO2 as usize]),
            Self::HTINST => Ok(self.regs[Self::HTINST as usize]),
            Self::HGATP => Ok(self.regs[Self::HGATP as usize]),
            Self::HCONTEXT => Ok(self.regs[Self::HCONTEXT as usize]),
            Self::HGEIP => Ok(self.regs[Self::HGEIP as usize]),
            Self::VSTOPI => Ok(self.regs[Self::VSTOPI as usize]),
            Self::SCOUNTOVF => Ok(self.regs[Self::SCOUNTOVF as usize]),
            Self::STOPI => Ok(self.regs[Self::STOPI as usize]),
            Self::UTVT => Ok(self.regs[Self::UTVT as usize]),
            Self::UNXTI => Ok(self.regs[Self::UNXTI as usize]),
            Self::UINTSTATUS => Ok(self.regs[Self::UINTSTATUS as usize]),
            Self::USCRATCHCSW => Ok(self.regs[Self::USCRATCHCSW as usize]),
            Self::USCRATCHCSWL => Ok(self.regs[Self::USCRATCHCSWL as usize]),
            Self::STVT => Ok(self.regs[Self::STVT as usize]),
            Self::SNXTI => Ok(self.regs[Self::SNXTI as usize]),
            Self::SINTSTATUS => Ok(self.regs[Self::SINTSTATUS as usize]),
            Self::SSCRATCHCSW => Ok(self.regs[Self::SSCRATCHCSW as usize]),
            Self::SSCRATCHCSWL => Ok(self.regs[Self::SSCRATCHCSWL as usize]),
            Self::MTVT => Ok(self.regs[Self::MTVT as usize]),
            Self::MNXTI => Ok(self.regs[Self::MNXTI as usize]),
            Self::MINTSTATUS => Ok(self.regs[Self::MINTSTATUS as usize]),
            Self::MSCRATCHCSW => Ok(self.regs[Self::MSCRATCHCSW as usize]),
            Self::MSCRATCHCSWL => Ok(self.regs[Self::MSCRATCHCSWL as usize]),
            Self::MSTATUS => Ok(self.regs[Self::MSTATUS as usize]),
            Self::MISA => Ok(self.regs[Self::MISA as usize]),
            Self::MEDELEG => Ok(self.regs[Self::MEDELEG as usize]),
            Self::MIDELEG => Ok(self.regs[Self::MIDELEG as usize]),
            Self::MIE => Ok(self.regs[Self::MIE as usize]),
            Self::MTVEC => Ok(self.regs[Self::MTVEC as usize]),
            Self::MCOUNTEREN => Ok(self.regs[Self::MCOUNTEREN as usize]),
            Self::MVIEN => Ok(self.regs[Self::MVIEN as usize]),
            Self::MVIP => Ok(self.regs[Self::MVIP as usize]),
            Self::MENVCFG => Ok(self.regs[Self::MENVCFG as usize]),
            Self::MSTATEEN0 => Ok(self.regs[Self::MSTATEEN0 as usize]),
            Self::MSTATEEN1 => Ok(self.regs[Self::MSTATEEN1 as usize]),
            Self::MSTATEEN2 => Ok(self.regs[Self::MSTATEEN2 as usize]),
            Self::MSTATEEN3 => Ok(self.regs[Self::MSTATEEN3 as usize]),
            Self::MCOUNTINHIBIT => Ok(self.regs[Self::MCOUNTINHIBIT as usize]),
            Self::MSCRATCH => Ok(self.regs[Self::MSCRATCH as usize]),
            Self::MEPC => Ok(self.regs[Self::MEPC as usize]),
            Self::MCAUSE => Ok(self.regs[Self::MCAUSE as usize]),
            Self::MTVAL => Ok(self.regs[Self::MTVAL as usize]),
            Self::MIP => Ok(self.regs[Self::MIP as usize]),
            Self::MTINST => Ok(self.regs[Self::MTINST as usize]),
            Self::MTVAL2 => Ok(self.regs[Self::MTVAL2 as usize]),
            Self::MCTRCTL => Ok(self.regs[Self::MCTRCTL as usize]),
            Self::MISELECT => Ok(self.regs[Self::MISELECT as usize]),
            Self::MIREG => Ok(self.regs[Self::MIREG as usize]),
            Self::MIREG2 => Ok(self.regs[Self::MIREG2 as usize]),
            Self::MIREG3 => Ok(self.regs[Self::MIREG3 as usize]),
            Self::MIREG4 => Ok(self.regs[Self::MIREG4 as usize]),
            Self::MIREG5 => Ok(self.regs[Self::MIREG5 as usize]),
            Self::MIREG6 => Ok(self.regs[Self::MIREG6 as usize]),
            Self::MTOPEI => Ok(self.regs[Self::MTOPEI as usize]),
            Self::PMPCFG0 => Ok(self.regs[Self::PMPCFG0 as usize]),
            Self::PMPCFG1 => Ok(self.regs[Self::PMPCFG1 as usize]),
            Self::PMPCFG2 => Ok(self.regs[Self::PMPCFG2 as usize]),
            Self::PMPCFG3 => Ok(self.regs[Self::PMPCFG3 as usize]),
            Self::PMPCFG4 => Ok(self.regs[Self::PMPCFG4 as usize]),
            Self::PMPCFG5 => Ok(self.regs[Self::PMPCFG5 as usize]),
            Self::PMPCFG6 => Ok(self.regs[Self::PMPCFG6 as usize]),
            Self::PMPCFG7 => Ok(self.regs[Self::PMPCFG7 as usize]),
            Self::PMPCFG8 => Ok(self.regs[Self::PMPCFG8 as usize]),
            Self::PMPCFG9 => Ok(self.regs[Self::PMPCFG9 as usize]),
            Self::PMPCFG10 => Ok(self.regs[Self::PMPCFG10 as usize]),
            Self::PMPCFG11 => Ok(self.regs[Self::PMPCFG11 as usize]),
            Self::PMPCFG12 => Ok(self.regs[Self::PMPCFG12 as usize]),
            Self::PMPCFG13 => Ok(self.regs[Self::PMPCFG13 as usize]),
            Self::PMPCFG14 => Ok(self.regs[Self::PMPCFG14 as usize]),
            Self::PMPCFG15 => Ok(self.regs[Self::PMPCFG15 as usize]),
            Self::PMPADDR0 => Ok(self.regs[Self::PMPADDR0 as usize]),
            Self::PMPADDR1 => Ok(self.regs[Self::PMPADDR1 as usize]),
            Self::PMPADDR2 => Ok(self.regs[Self::PMPADDR2 as usize]),
            Self::PMPADDR3 => Ok(self.regs[Self::PMPADDR3 as usize]),
            Self::PMPADDR4 => Ok(self.regs[Self::PMPADDR4 as usize]),
            Self::PMPADDR5 => Ok(self.regs[Self::PMPADDR5 as usize]),
            Self::PMPADDR6 => Ok(self.regs[Self::PMPADDR6 as usize]),
            Self::PMPADDR7 => Ok(self.regs[Self::PMPADDR7 as usize]),
            Self::PMPADDR8 => Ok(self.regs[Self::PMPADDR8 as usize]),
            Self::PMPADDR9 => Ok(self.regs[Self::PMPADDR9 as usize]),
            Self::PMPADDR10 => Ok(self.regs[Self::PMPADDR10 as usize]),
            Self::PMPADDR11 => Ok(self.regs[Self::PMPADDR11 as usize]),
            Self::PMPADDR12 => Ok(self.regs[Self::PMPADDR12 as usize]),
            Self::PMPADDR13 => Ok(self.regs[Self::PMPADDR13 as usize]),
            Self::PMPADDR14 => Ok(self.regs[Self::PMPADDR14 as usize]),
            Self::PMPADDR15 => Ok(self.regs[Self::PMPADDR15 as usize]),
            Self::PMPADDR16 => Ok(self.regs[Self::PMPADDR16 as usize]),
            Self::PMPADDR17 => Ok(self.regs[Self::PMPADDR17 as usize]),
            Self::PMPADDR18 => Ok(self.regs[Self::PMPADDR18 as usize]),
            Self::PMPADDR19 => Ok(self.regs[Self::PMPADDR19 as usize]),
            Self::PMPADDR20 => Ok(self.regs[Self::PMPADDR20 as usize]),
            Self::PMPADDR21 => Ok(self.regs[Self::PMPADDR21 as usize]),
            Self::PMPADDR22 => Ok(self.regs[Self::PMPADDR22 as usize]),
            Self::PMPADDR23 => Ok(self.regs[Self::PMPADDR23 as usize]),
            Self::PMPADDR24 => Ok(self.regs[Self::PMPADDR24 as usize]),
            Self::PMPADDR25 => Ok(self.regs[Self::PMPADDR25 as usize]),
            Self::PMPADDR26 => Ok(self.regs[Self::PMPADDR26 as usize]),
            Self::PMPADDR27 => Ok(self.regs[Self::PMPADDR27 as usize]),
            Self::PMPADDR28 => Ok(self.regs[Self::PMPADDR28 as usize]),
            Self::PMPADDR29 => Ok(self.regs[Self::PMPADDR29 as usize]),
            Self::PMPADDR30 => Ok(self.regs[Self::PMPADDR30 as usize]),
            Self::PMPADDR31 => Ok(self.regs[Self::PMPADDR31 as usize]),
            Self::PMPADDR32 => Ok(self.regs[Self::PMPADDR32 as usize]),
            Self::PMPADDR33 => Ok(self.regs[Self::PMPADDR33 as usize]),
            Self::PMPADDR34 => Ok(self.regs[Self::PMPADDR34 as usize]),
            Self::PMPADDR35 => Ok(self.regs[Self::PMPADDR35 as usize]),
            Self::PMPADDR36 => Ok(self.regs[Self::PMPADDR36 as usize]),
            Self::PMPADDR37 => Ok(self.regs[Self::PMPADDR37 as usize]),
            Self::PMPADDR38 => Ok(self.regs[Self::PMPADDR38 as usize]),
            Self::PMPADDR39 => Ok(self.regs[Self::PMPADDR39 as usize]),
            Self::PMPADDR40 => Ok(self.regs[Self::PMPADDR40 as usize]),
            Self::PMPADDR41 => Ok(self.regs[Self::PMPADDR41 as usize]),
            Self::PMPADDR42 => Ok(self.regs[Self::PMPADDR42 as usize]),
            Self::PMPADDR43 => Ok(self.regs[Self::PMPADDR43 as usize]),
            Self::PMPADDR44 => Ok(self.regs[Self::PMPADDR44 as usize]),
            Self::PMPADDR45 => Ok(self.regs[Self::PMPADDR45 as usize]),
            Self::PMPADDR46 => Ok(self.regs[Self::PMPADDR46 as usize]),
            Self::PMPADDR47 => Ok(self.regs[Self::PMPADDR47 as usize]),
            Self::PMPADDR48 => Ok(self.regs[Self::PMPADDR48 as usize]),
            Self::PMPADDR49 => Ok(self.regs[Self::PMPADDR49 as usize]),
            Self::PMPADDR50 => Ok(self.regs[Self::PMPADDR50 as usize]),
            Self::PMPADDR51 => Ok(self.regs[Self::PMPADDR51 as usize]),
            Self::PMPADDR52 => Ok(self.regs[Self::PMPADDR52 as usize]),
            Self::PMPADDR53 => Ok(self.regs[Self::PMPADDR53 as usize]),
            Self::PMPADDR54 => Ok(self.regs[Self::PMPADDR54 as usize]),
            Self::PMPADDR55 => Ok(self.regs[Self::PMPADDR55 as usize]),
            Self::PMPADDR56 => Ok(self.regs[Self::PMPADDR56 as usize]),
            Self::PMPADDR57 => Ok(self.regs[Self::PMPADDR57 as usize]),
            Self::PMPADDR58 => Ok(self.regs[Self::PMPADDR58 as usize]),
            Self::PMPADDR59 => Ok(self.regs[Self::PMPADDR59 as usize]),
            Self::PMPADDR60 => Ok(self.regs[Self::PMPADDR60 as usize]),
            Self::PMPADDR61 => Ok(self.regs[Self::PMPADDR61 as usize]),
            Self::PMPADDR62 => Ok(self.regs[Self::PMPADDR62 as usize]),
            Self::PMPADDR63 => Ok(self.regs[Self::PMPADDR63 as usize]),
            Self::MSECCFG => Ok(self.regs[Self::MSECCFG as usize]),
            Self::TSELECT => Ok(self.regs[Self::TSELECT as usize]),
            Self::TDATA1 => Ok(self.regs[Self::TDATA1 as usize]),
            Self::TDATA2 => Ok(self.regs[Self::TDATA2 as usize]),
            Self::TDATA3 => Ok(self.regs[Self::TDATA3 as usize]),
            Self::TINFO => Ok(self.regs[Self::TINFO as usize]),
            Self::TCONTROL => Ok(self.regs[Self::TCONTROL as usize]),
            Self::MCONTEXT => Ok(self.regs[Self::MCONTEXT as usize]),
            Self::MSCONTEXT => Ok(self.regs[Self::MSCONTEXT as usize]),
            Self::DCSR => Ok(self.regs[Self::DCSR as usize]),
            Self::DPC => Ok(self.regs[Self::DPC as usize]),
            Self::DSCRATCH0 => Ok(self.regs[Self::DSCRATCH0 as usize]),
            Self::DSCRATCH1 => Ok(self.regs[Self::DSCRATCH1 as usize]),
            Self::MCYCLE => Ok(self.regs[Self::MCYCLE as usize]),
            Self::MINSTRET => Ok(self.regs[Self::MINSTRET as usize]),
            Self::MHPMCOUNTER3 => Ok(self.regs[Self::MHPMCOUNTER3 as usize]),
            Self::MHPMCOUNTER4 => Ok(self.regs[Self::MHPMCOUNTER4 as usize]),
            Self::MHPMCOUNTER5 => Ok(self.regs[Self::MHPMCOUNTER5 as usize]),
            Self::MHPMCOUNTER6 => Ok(self.regs[Self::MHPMCOUNTER6 as usize]),
            Self::MHPMCOUNTER7 => Ok(self.regs[Self::MHPMCOUNTER7 as usize]),
            Self::MHPMCOUNTER8 => Ok(self.regs[Self::MHPMCOUNTER8 as usize]),
            Self::MHPMCOUNTER9 => Ok(self.regs[Self::MHPMCOUNTER9 as usize]),
            Self::MHPMCOUNTER10 => Ok(self.regs[Self::MHPMCOUNTER10 as usize]),
            Self::MHPMCOUNTER11 => Ok(self.regs[Self::MHPMCOUNTER11 as usize]),
            Self::MHPMCOUNTER12 => Ok(self.regs[Self::MHPMCOUNTER12 as usize]),
            Self::MHPMCOUNTER13 => Ok(self.regs[Self::MHPMCOUNTER13 as usize]),
            Self::MHPMCOUNTER14 => Ok(self.regs[Self::MHPMCOUNTER14 as usize]),
            Self::MHPMCOUNTER15 => Ok(self.regs[Self::MHPMCOUNTER15 as usize]),
            Self::MHPMCOUNTER16 => Ok(self.regs[Self::MHPMCOUNTER16 as usize]),
            Self::MHPMCOUNTER17 => Ok(self.regs[Self::MHPMCOUNTER17 as usize]),
            Self::MHPMCOUNTER18 => Ok(self.regs[Self::MHPMCOUNTER18 as usize]),
            Self::MHPMCOUNTER19 => Ok(self.regs[Self::MHPMCOUNTER19 as usize]),
            Self::MHPMCOUNTER20 => Ok(self.regs[Self::MHPMCOUNTER20 as usize]),
            Self::MHPMCOUNTER21 => Ok(self.regs[Self::MHPMCOUNTER21 as usize]),
            Self::MHPMCOUNTER22 => Ok(self.regs[Self::MHPMCOUNTER22 as usize]),
            Self::MHPMCOUNTER23 => Ok(self.regs[Self::MHPMCOUNTER23 as usize]),
            Self::MHPMCOUNTER24 => Ok(self.regs[Self::MHPMCOUNTER24 as usize]),
            Self::MHPMCOUNTER25 => Ok(self.regs[Self::MHPMCOUNTER25 as usize]),
            Self::MHPMCOUNTER26 => Ok(self.regs[Self::MHPMCOUNTER26 as usize]),
            Self::MHPMCOUNTER27 => Ok(self.regs[Self::MHPMCOUNTER27 as usize]),
            Self::MHPMCOUNTER28 => Ok(self.regs[Self::MHPMCOUNTER28 as usize]),
            Self::MHPMCOUNTER29 => Ok(self.regs[Self::MHPMCOUNTER29 as usize]),
            Self::MHPMCOUNTER30 => Ok(self.regs[Self::MHPMCOUNTER30 as usize]),
            Self::MHPMCOUNTER31 => Ok(self.regs[Self::MHPMCOUNTER31 as usize]),
            Self::MCYCLECFG => Ok(self.regs[Self::MCYCLECFG as usize]),
            Self::MINSTRETCFG => Ok(self.regs[Self::MINSTRETCFG as usize]),
            Self::MHPMEVENT3 => Ok(self.regs[Self::MHPMEVENT3 as usize]),
            Self::MHPMEVENT4 => Ok(self.regs[Self::MHPMEVENT4 as usize]),
            Self::MHPMEVENT5 => Ok(self.regs[Self::MHPMEVENT5 as usize]),
            Self::MHPMEVENT6 => Ok(self.regs[Self::MHPMEVENT6 as usize]),
            Self::MHPMEVENT7 => Ok(self.regs[Self::MHPMEVENT7 as usize]),
            Self::MHPMEVENT8 => Ok(self.regs[Self::MHPMEVENT8 as usize]),
            Self::MHPMEVENT9 => Ok(self.regs[Self::MHPMEVENT9 as usize]),
            Self::MHPMEVENT10 => Ok(self.regs[Self::MHPMEVENT10 as usize]),
            Self::MHPMEVENT11 => Ok(self.regs[Self::MHPMEVENT11 as usize]),
            Self::MHPMEVENT12 => Ok(self.regs[Self::MHPMEVENT12 as usize]),
            Self::MHPMEVENT13 => Ok(self.regs[Self::MHPMEVENT13 as usize]),
            Self::MHPMEVENT14 => Ok(self.regs[Self::MHPMEVENT14 as usize]),
            Self::MHPMEVENT15 => Ok(self.regs[Self::MHPMEVENT15 as usize]),
            Self::MHPMEVENT16 => Ok(self.regs[Self::MHPMEVENT16 as usize]),
            Self::MHPMEVENT17 => Ok(self.regs[Self::MHPMEVENT17 as usize]),
            Self::MHPMEVENT18 => Ok(self.regs[Self::MHPMEVENT18 as usize]),
            Self::MHPMEVENT19 => Ok(self.regs[Self::MHPMEVENT19 as usize]),
            Self::MHPMEVENT20 => Ok(self.regs[Self::MHPMEVENT20 as usize]),
            Self::MHPMEVENT21 => Ok(self.regs[Self::MHPMEVENT21 as usize]),
            Self::MHPMEVENT22 => Ok(self.regs[Self::MHPMEVENT22 as usize]),
            Self::MHPMEVENT23 => Ok(self.regs[Self::MHPMEVENT23 as usize]),
            Self::MHPMEVENT24 => Ok(self.regs[Self::MHPMEVENT24 as usize]),
            Self::MHPMEVENT25 => Ok(self.regs[Self::MHPMEVENT25 as usize]),
            Self::MHPMEVENT26 => Ok(self.regs[Self::MHPMEVENT26 as usize]),
            Self::MHPMEVENT27 => Ok(self.regs[Self::MHPMEVENT27 as usize]),
            Self::MHPMEVENT28 => Ok(self.regs[Self::MHPMEVENT28 as usize]),
            Self::MHPMEVENT29 => Ok(self.regs[Self::MHPMEVENT29 as usize]),
            Self::MHPMEVENT30 => Ok(self.regs[Self::MHPMEVENT30 as usize]),
            Self::MHPMEVENT31 => Ok(self.regs[Self::MHPMEVENT31 as usize]),
            Self::MVENDORID => Ok(self.regs[Self::MVENDORID as usize]),
            Self::MARCHID => Ok(self.regs[Self::MARCHID as usize]),
            Self::MIMPID => Ok(self.regs[Self::MIMPID as usize]),
            Self::MHARTID => Ok(self.regs[Self::MHARTID as usize]),
            Self::MCONFIGPTR => Ok(self.regs[Self::MCONFIGPTR as usize]),
            Self::MTOPI => Ok(self.regs[Self::MTOPI as usize]),
            _ => Err(Error::UnknownCsr)
        }
    }


    fn store(&mut self, address: u64, value: u64) -> Result<()> {
        match address {
            Self::FFLAGS => Ok(self.regs[Self::FFLAGS as usize] = value),
            Self::FRM => Ok(self.regs[Self::FRM as usize] = value),
            Self::FCSR => Ok(self.regs[Self::FCSR as usize] = value),
            Self::VSTART => Ok(self.regs[Self::VSTART as usize] = value),
            Self::VXSAT => Ok(self.regs[Self::VXSAT as usize] = value),
            Self::VXRM => Ok(self.regs[Self::VXRM as usize] = value),
            Self::VCSR => Ok(self.regs[Self::VCSR as usize] = value),
            Self::SSP => Ok(self.regs[Self::SSP as usize] = value),
            Self::SEED => Ok(self.regs[Self::SEED as usize] = value),
            Self::JVT => Ok(self.regs[Self::JVT as usize] = value),
            Self::CYCLE => Ok(self.regs[Self::CYCLE as usize] = value),
            Self::TIME => Ok(self.regs[Self::TIME as usize] = value),
            Self::INSTRET => Ok(self.regs[Self::INSTRET as usize] = value),
            Self::HPMCOUNTER3 => Ok(self.regs[Self::HPMCOUNTER3 as usize] = value),
            Self::HPMCOUNTER4 => Ok(self.regs[Self::HPMCOUNTER4 as usize] = value),
            Self::HPMCOUNTER5 => Ok(self.regs[Self::HPMCOUNTER5 as usize] = value),
            Self::HPMCOUNTER6 => Ok(self.regs[Self::HPMCOUNTER6 as usize] = value),
            Self::HPMCOUNTER7 => Ok(self.regs[Self::HPMCOUNTER7 as usize] = value),
            Self::HPMCOUNTER8 => Ok(self.regs[Self::HPMCOUNTER8 as usize] = value),
            Self::HPMCOUNTER9 => Ok(self.regs[Self::HPMCOUNTER9 as usize] = value),
            Self::HPMCOUNTER10 => Ok(self.regs[Self::HPMCOUNTER10 as usize] = value),
            Self::HPMCOUNTER11 => Ok(self.regs[Self::HPMCOUNTER11 as usize] = value),
            Self::HPMCOUNTER12 => Ok(self.regs[Self::HPMCOUNTER12 as usize] = value),
            Self::HPMCOUNTER13 => Ok(self.regs[Self::HPMCOUNTER13 as usize] = value),
            Self::HPMCOUNTER14 => Ok(self.regs[Self::HPMCOUNTER14 as usize] = value),
            Self::HPMCOUNTER15 => Ok(self.regs[Self::HPMCOUNTER15 as usize] = value),
            Self::HPMCOUNTER16 => Ok(self.regs[Self::HPMCOUNTER16 as usize] = value),
            Self::HPMCOUNTER17 => Ok(self.regs[Self::HPMCOUNTER17 as usize] = value),
            Self::HPMCOUNTER18 => Ok(self.regs[Self::HPMCOUNTER18 as usize] = value),
            Self::HPMCOUNTER19 => Ok(self.regs[Self::HPMCOUNTER19 as usize] = value),
            Self::HPMCOUNTER20 => Ok(self.regs[Self::HPMCOUNTER20 as usize] = value),
            Self::HPMCOUNTER21 => Ok(self.regs[Self::HPMCOUNTER21 as usize] = value),
            Self::HPMCOUNTER22 => Ok(self.regs[Self::HPMCOUNTER22 as usize] = value),
            Self::HPMCOUNTER23 => Ok(self.regs[Self::HPMCOUNTER23 as usize] = value),
            Self::HPMCOUNTER24 => Ok(self.regs[Self::HPMCOUNTER24 as usize] = value),
            Self::HPMCOUNTER25 => Ok(self.regs[Self::HPMCOUNTER25 as usize] = value),
            Self::HPMCOUNTER26 => Ok(self.regs[Self::HPMCOUNTER26 as usize] = value),
            Self::HPMCOUNTER27 => Ok(self.regs[Self::HPMCOUNTER27 as usize] = value),
            Self::HPMCOUNTER28 => Ok(self.regs[Self::HPMCOUNTER28 as usize] = value),
            Self::HPMCOUNTER29 => Ok(self.regs[Self::HPMCOUNTER29 as usize] = value),
            Self::HPMCOUNTER30 => Ok(self.regs[Self::HPMCOUNTER30 as usize] = value),
            Self::HPMCOUNTER31 => Ok(self.regs[Self::HPMCOUNTER31 as usize] = value),
            Self::VL => Ok(self.regs[Self::VL as usize] = value),
            Self::VTYPE => Ok(self.regs[Self::VTYPE as usize] = value),
            Self::VLENB => Ok(self.regs[Self::VLENB as usize] = value),
            Self::SSTATUS => Ok(self.regs[Self::SSTATUS as usize] = value),
            Self::SEDELEG => Ok(self.regs[Self::SEDELEG as usize] = value),
            Self::SIDELEG => Ok(self.regs[Self::SIDELEG as usize] = value),
            Self::SIE => Ok(self.regs[Self::SIE as usize] = value),
            Self::STVEC => Ok(self.regs[Self::STVEC as usize] = value),
            Self::SCOUNTEREN => Ok(self.regs[Self::SCOUNTEREN as usize] = value),
            Self::SENVCFG => Ok(self.regs[Self::SENVCFG as usize] = value),
            Self::SSTATEEN0 => Ok(self.regs[Self::SSTATEEN0 as usize] = value),
            Self::SSTATEEN1 => Ok(self.regs[Self::SSTATEEN1 as usize] = value),
            Self::SSTATEEN2 => Ok(self.regs[Self::SSTATEEN2 as usize] = value),
            Self::SSTATEEN3 => Ok(self.regs[Self::SSTATEEN3 as usize] = value),
            Self::SCOUNTINHIBIT => Ok(self.regs[Self::SCOUNTINHIBIT as usize] = value),
            Self::SSCRATCH => Ok(self.regs[Self::SSCRATCH as usize] = value),
            Self::SEPC => Ok(self.regs[Self::SEPC as usize] = value),
            Self::SCAUSE => Ok(self.regs[Self::SCAUSE as usize] = value),
            Self::STVAL => Ok(self.regs[Self::STVAL as usize] = value),
            Self::SIP => Ok(self.regs[Self::SIP as usize] = value),
            Self::STIMECMP => Ok(self.regs[Self::STIMECMP as usize] = value),
            Self::SCTRCTL => Ok(self.regs[Self::SCTRCTL as usize] = value),
            Self::SCTRSTATUS => Ok(self.regs[Self::SCTRSTATUS as usize] = value),
            Self::SISELECT => Ok(self.regs[Self::SISELECT as usize] = value),
            Self::SIREG => Ok(self.regs[Self::SIREG as usize] = value),
            Self::SIREG2 => Ok(self.regs[Self::SIREG2 as usize] = value),
            Self::SIREG3 => Ok(self.regs[Self::SIREG3 as usize] = value),
            Self::SIREG4 => Ok(self.regs[Self::SIREG4 as usize] = value),
            Self::SIREG5 => Ok(self.regs[Self::SIREG5 as usize] = value),
            Self::SIREG6 => Ok(self.regs[Self::SIREG6 as usize] = value),
            Self::STOPEI => Ok(self.regs[Self::STOPEI as usize] = value),
            Self::SCTRDEPTH => Ok(self.regs[Self::SCTRDEPTH as usize] = value),
            Self::SATP => Ok(self.regs[Self::SATP as usize] = value),
            Self::SRMCFG => Ok(self.regs[Self::SRMCFG as usize] = value),
            Self::SCONTEXT => Ok(self.regs[Self::SCONTEXT as usize] = value),
            Self::VSSTATUS => Ok(self.regs[Self::VSSTATUS as usize] = value),
            Self::VSIE => Ok(self.regs[Self::VSIE as usize] = value),
            Self::VSTVEC => Ok(self.regs[Self::VSTVEC as usize] = value),
            Self::VSSCRATCH => Ok(self.regs[Self::VSSCRATCH as usize] = value),
            Self::VSEPC => Ok(self.regs[Self::VSEPC as usize] = value),
            Self::VSCAUSE => Ok(self.regs[Self::VSCAUSE as usize] = value),
            Self::VSTVAL => Ok(self.regs[Self::VSTVAL as usize] = value),
            Self::VSIP => Ok(self.regs[Self::VSIP as usize] = value),
            Self::VSTIMECMP => Ok(self.regs[Self::VSTIMECMP as usize] = value),
            Self::VSCTRCTL => Ok(self.regs[Self::VSCTRCTL as usize] = value),
            Self::VSISELECT => Ok(self.regs[Self::VSISELECT as usize] = value),
            Self::VSIREG => Ok(self.regs[Self::VSIREG as usize] = value),
            Self::VSIREG2 => Ok(self.regs[Self::VSIREG2 as usize] = value),
            Self::VSIREG3 => Ok(self.regs[Self::VSIREG3 as usize] = value),
            Self::VSIREG4 => Ok(self.regs[Self::VSIREG4 as usize] = value),
            Self::VSIREG5 => Ok(self.regs[Self::VSIREG5 as usize] = value),
            Self::VSIREG6 => Ok(self.regs[Self::VSIREG6 as usize] = value),
            Self::VSTOPEI => Ok(self.regs[Self::VSTOPEI as usize] = value),
            Self::VSATP => Ok(self.regs[Self::VSATP as usize] = value),
            Self::HSTATUS => Ok(self.regs[Self::HSTATUS as usize] = value),
            Self::HEDELEG => Ok(self.regs[Self::HEDELEG as usize] = value),
            Self::HIDELEG => Ok(self.regs[Self::HIDELEG as usize] = value),
            Self::HIE => Ok(self.regs[Self::HIE as usize] = value),
            Self::HTIMEDELTA => Ok(self.regs[Self::HTIMEDELTA as usize] = value),
            Self::HCOUNTEREN => Ok(self.regs[Self::HCOUNTEREN as usize] = value),
            Self::HGEIE => Ok(self.regs[Self::HGEIE as usize] = value),
            Self::HVIEN => Ok(self.regs[Self::HVIEN as usize] = value),
            Self::HVICTL => Ok(self.regs[Self::HVICTL as usize] = value),
            Self::HENVCFG => Ok(self.regs[Self::HENVCFG as usize] = value),
            Self::HSTATEEN0 => Ok(self.regs[Self::HSTATEEN0 as usize] = value),
            Self::HSTATEEN1 => Ok(self.regs[Self::HSTATEEN1 as usize] = value),
            Self::HSTATEEN2 => Ok(self.regs[Self::HSTATEEN2 as usize] = value),
            Self::HSTATEEN3 => Ok(self.regs[Self::HSTATEEN3 as usize] = value),
            Self::HTVAL => Ok(self.regs[Self::HTVAL as usize] = value),
            Self::HIP => Ok(self.regs[Self::HIP as usize] = value),
            Self::HVIP => Ok(self.regs[Self::HVIP as usize] = value),
            Self::HVIPRIO1 => Ok(self.regs[Self::HVIPRIO1 as usize] = value),
            Self::HVIPRIO2 => Ok(self.regs[Self::HVIPRIO2 as usize] = value),
            Self::HTINST => Ok(self.regs[Self::HTINST as usize] = value),
            Self::HGATP => Ok(self.regs[Self::HGATP as usize] = value),
            Self::HCONTEXT => Ok(self.regs[Self::HCONTEXT as usize] = value),
            Self::HGEIP => Ok(self.regs[Self::HGEIP as usize] = value),
            Self::VSTOPI => Ok(self.regs[Self::VSTOPI as usize] = value),
            Self::SCOUNTOVF => Ok(self.regs[Self::SCOUNTOVF as usize] = value),
            Self::STOPI => Ok(self.regs[Self::STOPI as usize] = value),
            Self::UTVT => Ok(self.regs[Self::UTVT as usize] = value),
            Self::UNXTI => Ok(self.regs[Self::UNXTI as usize] = value),
            Self::UINTSTATUS => Ok(self.regs[Self::UINTSTATUS as usize] = value),
            Self::USCRATCHCSW => Ok(self.regs[Self::USCRATCHCSW as usize] = value),
            Self::USCRATCHCSWL => Ok(self.regs[Self::USCRATCHCSWL as usize] = value),
            Self::STVT => Ok(self.regs[Self::STVT as usize] = value),
            Self::SNXTI => Ok(self.regs[Self::SNXTI as usize] = value),
            Self::SINTSTATUS => Ok(self.regs[Self::SINTSTATUS as usize] = value),
            Self::SSCRATCHCSW => Ok(self.regs[Self::SSCRATCHCSW as usize] = value),
            Self::SSCRATCHCSWL => Ok(self.regs[Self::SSCRATCHCSWL as usize] = value),
            Self::MTVT => Ok(self.regs[Self::MTVT as usize] = value),
            Self::MNXTI => Ok(self.regs[Self::MNXTI as usize] = value),
            Self::MINTSTATUS => Ok(self.regs[Self::MINTSTATUS as usize] = value),
            Self::MSCRATCHCSW => Ok(self.regs[Self::MSCRATCHCSW as usize] = value),
            Self::MSCRATCHCSWL => Ok(self.regs[Self::MSCRATCHCSWL as usize] = value),
            Self::MSTATUS => Ok(self.regs[Self::MSTATUS as usize] = value),
            Self::MISA => Ok(self.regs[Self::MISA as usize] = value),
            Self::MEDELEG => Ok(self.regs[Self::MEDELEG as usize] = value),
            Self::MIDELEG => Ok(self.regs[Self::MIDELEG as usize] = value),
            Self::MIE => Ok(self.regs[Self::MIE as usize] = value),
            Self::MTVEC => Ok(self.regs[Self::MTVEC as usize] = value),
            Self::MCOUNTEREN => Ok(self.regs[Self::MCOUNTEREN as usize] = value),
            Self::MVIEN => Ok(self.regs[Self::MVIEN as usize] = value),
            Self::MVIP => Ok(self.regs[Self::MVIP as usize] = value),
            Self::MENVCFG => Ok(self.regs[Self::MENVCFG as usize] = value),
            Self::MSTATEEN0 => Ok(self.regs[Self::MSTATEEN0 as usize] = value),
            Self::MSTATEEN1 => Ok(self.regs[Self::MSTATEEN1 as usize] = value),
            Self::MSTATEEN2 => Ok(self.regs[Self::MSTATEEN2 as usize] = value),
            Self::MSTATEEN3 => Ok(self.regs[Self::MSTATEEN3 as usize] = value),
            Self::MCOUNTINHIBIT => Ok(self.regs[Self::MCOUNTINHIBIT as usize] = value),
            Self::MSCRATCH => Ok(self.regs[Self::MSCRATCH as usize] = value),
            Self::MEPC => Ok(self.regs[Self::MEPC as usize] = value),
            Self::MCAUSE => Ok(self.regs[Self::MCAUSE as usize] = value),
            Self::MTVAL => Ok(self.regs[Self::MTVAL as usize] = value),
            Self::MIP => Ok(self.regs[Self::MIP as usize] = value),
            Self::MTINST => Ok(self.regs[Self::MTINST as usize] = value),
            Self::MTVAL2 => Ok(self.regs[Self::MTVAL2 as usize] = value),
            Self::MCTRCTL => Ok(self.regs[Self::MCTRCTL as usize] = value),
            Self::MISELECT => Ok(self.regs[Self::MISELECT as usize] = value),
            Self::MIREG => Ok(self.regs[Self::MIREG as usize] = value),
            Self::MIREG2 => Ok(self.regs[Self::MIREG2 as usize] = value),
            Self::MIREG3 => Ok(self.regs[Self::MIREG3 as usize] = value),
            Self::MIREG4 => Ok(self.regs[Self::MIREG4 as usize] = value),
            Self::MIREG5 => Ok(self.regs[Self::MIREG5 as usize] = value),
            Self::MIREG6 => Ok(self.regs[Self::MIREG6 as usize] = value),
            Self::MTOPEI => Ok(self.regs[Self::MTOPEI as usize] = value),
            Self::PMPCFG0 => Ok(self.regs[Self::PMPCFG0 as usize] = value),
            Self::PMPCFG1 => Ok(self.regs[Self::PMPCFG1 as usize] = value),
            Self::PMPCFG2 => Ok(self.regs[Self::PMPCFG2 as usize] = value),
            Self::PMPCFG3 => Ok(self.regs[Self::PMPCFG3 as usize] = value),
            Self::PMPCFG4 => Ok(self.regs[Self::PMPCFG4 as usize] = value),
            Self::PMPCFG5 => Ok(self.regs[Self::PMPCFG5 as usize] = value),
            Self::PMPCFG6 => Ok(self.regs[Self::PMPCFG6 as usize] = value),
            Self::PMPCFG7 => Ok(self.regs[Self::PMPCFG7 as usize] = value),
            Self::PMPCFG8 => Ok(self.regs[Self::PMPCFG8 as usize] = value),
            Self::PMPCFG9 => Ok(self.regs[Self::PMPCFG9 as usize] = value),
            Self::PMPCFG10 => Ok(self.regs[Self::PMPCFG10 as usize] = value),
            Self::PMPCFG11 => Ok(self.regs[Self::PMPCFG11 as usize] = value),
            Self::PMPCFG12 => Ok(self.regs[Self::PMPCFG12 as usize] = value),
            Self::PMPCFG13 => Ok(self.regs[Self::PMPCFG13 as usize] = value),
            Self::PMPCFG14 => Ok(self.regs[Self::PMPCFG14 as usize] = value),
            Self::PMPCFG15 => Ok(self.regs[Self::PMPCFG15 as usize] = value),
            Self::PMPADDR0 => Ok(self.regs[Self::PMPADDR0 as usize] = value),
            Self::PMPADDR1 => Ok(self.regs[Self::PMPADDR1 as usize] = value),
            Self::PMPADDR2 => Ok(self.regs[Self::PMPADDR2 as usize] = value),
            Self::PMPADDR3 => Ok(self.regs[Self::PMPADDR3 as usize] = value),
            Self::PMPADDR4 => Ok(self.regs[Self::PMPADDR4 as usize] = value),
            Self::PMPADDR5 => Ok(self.regs[Self::PMPADDR5 as usize] = value),
            Self::PMPADDR6 => Ok(self.regs[Self::PMPADDR6 as usize] = value),
            Self::PMPADDR7 => Ok(self.regs[Self::PMPADDR7 as usize] = value),
            Self::PMPADDR8 => Ok(self.regs[Self::PMPADDR8 as usize] = value),
            Self::PMPADDR9 => Ok(self.regs[Self::PMPADDR9 as usize] = value),
            Self::PMPADDR10 => Ok(self.regs[Self::PMPADDR10 as usize] = value),
            Self::PMPADDR11 => Ok(self.regs[Self::PMPADDR11 as usize] = value),
            Self::PMPADDR12 => Ok(self.regs[Self::PMPADDR12 as usize] = value),
            Self::PMPADDR13 => Ok(self.regs[Self::PMPADDR13 as usize] = value),
            Self::PMPADDR14 => Ok(self.regs[Self::PMPADDR14 as usize] = value),
            Self::PMPADDR15 => Ok(self.regs[Self::PMPADDR15 as usize] = value),
            Self::PMPADDR16 => Ok(self.regs[Self::PMPADDR16 as usize] = value),
            Self::PMPADDR17 => Ok(self.regs[Self::PMPADDR17 as usize] = value),
            Self::PMPADDR18 => Ok(self.regs[Self::PMPADDR18 as usize] = value),
            Self::PMPADDR19 => Ok(self.regs[Self::PMPADDR19 as usize] = value),
            Self::PMPADDR20 => Ok(self.regs[Self::PMPADDR20 as usize] = value),
            Self::PMPADDR21 => Ok(self.regs[Self::PMPADDR21 as usize] = value),
            Self::PMPADDR22 => Ok(self.regs[Self::PMPADDR22 as usize] = value),
            Self::PMPADDR23 => Ok(self.regs[Self::PMPADDR23 as usize] = value),
            Self::PMPADDR24 => Ok(self.regs[Self::PMPADDR24 as usize] = value),
            Self::PMPADDR25 => Ok(self.regs[Self::PMPADDR25 as usize] = value),
            Self::PMPADDR26 => Ok(self.regs[Self::PMPADDR26 as usize] = value),
            Self::PMPADDR27 => Ok(self.regs[Self::PMPADDR27 as usize] = value),
            Self::PMPADDR28 => Ok(self.regs[Self::PMPADDR28 as usize] = value),
            Self::PMPADDR29 => Ok(self.regs[Self::PMPADDR29 as usize] = value),
            Self::PMPADDR30 => Ok(self.regs[Self::PMPADDR30 as usize] = value),
            Self::PMPADDR31 => Ok(self.regs[Self::PMPADDR31 as usize] = value),
            Self::PMPADDR32 => Ok(self.regs[Self::PMPADDR32 as usize] = value),
            Self::PMPADDR33 => Ok(self.regs[Self::PMPADDR33 as usize] = value),
            Self::PMPADDR34 => Ok(self.regs[Self::PMPADDR34 as usize] = value),
            Self::PMPADDR35 => Ok(self.regs[Self::PMPADDR35 as usize] = value),
            Self::PMPADDR36 => Ok(self.regs[Self::PMPADDR36 as usize] = value),
            Self::PMPADDR37 => Ok(self.regs[Self::PMPADDR37 as usize] = value),
            Self::PMPADDR38 => Ok(self.regs[Self::PMPADDR38 as usize] = value),
            Self::PMPADDR39 => Ok(self.regs[Self::PMPADDR39 as usize] = value),
            Self::PMPADDR40 => Ok(self.regs[Self::PMPADDR40 as usize] = value),
            Self::PMPADDR41 => Ok(self.regs[Self::PMPADDR41 as usize] = value),
            Self::PMPADDR42 => Ok(self.regs[Self::PMPADDR42 as usize] = value),
            Self::PMPADDR43 => Ok(self.regs[Self::PMPADDR43 as usize] = value),
            Self::PMPADDR44 => Ok(self.regs[Self::PMPADDR44 as usize] = value),
            Self::PMPADDR45 => Ok(self.regs[Self::PMPADDR45 as usize] = value),
            Self::PMPADDR46 => Ok(self.regs[Self::PMPADDR46 as usize] = value),
            Self::PMPADDR47 => Ok(self.regs[Self::PMPADDR47 as usize] = value),
            Self::PMPADDR48 => Ok(self.regs[Self::PMPADDR48 as usize] = value),
            Self::PMPADDR49 => Ok(self.regs[Self::PMPADDR49 as usize] = value),
            Self::PMPADDR50 => Ok(self.regs[Self::PMPADDR50 as usize] = value),
            Self::PMPADDR51 => Ok(self.regs[Self::PMPADDR51 as usize] = value),
            Self::PMPADDR52 => Ok(self.regs[Self::PMPADDR52 as usize] = value),
            Self::PMPADDR53 => Ok(self.regs[Self::PMPADDR53 as usize] = value),
            Self::PMPADDR54 => Ok(self.regs[Self::PMPADDR54 as usize] = value),
            Self::PMPADDR55 => Ok(self.regs[Self::PMPADDR55 as usize] = value),
            Self::PMPADDR56 => Ok(self.regs[Self::PMPADDR56 as usize] = value),
            Self::PMPADDR57 => Ok(self.regs[Self::PMPADDR57 as usize] = value),
            Self::PMPADDR58 => Ok(self.regs[Self::PMPADDR58 as usize] = value),
            Self::PMPADDR59 => Ok(self.regs[Self::PMPADDR59 as usize] = value),
            Self::PMPADDR60 => Ok(self.regs[Self::PMPADDR60 as usize] = value),
            Self::PMPADDR61 => Ok(self.regs[Self::PMPADDR61 as usize] = value),
            Self::PMPADDR62 => Ok(self.regs[Self::PMPADDR62 as usize] = value),
            Self::PMPADDR63 => Ok(self.regs[Self::PMPADDR63 as usize] = value),
            Self::MSECCFG => Ok(self.regs[Self::MSECCFG as usize] = value),
            Self::TSELECT => Ok(self.regs[Self::TSELECT as usize] = value),
            Self::TDATA1 => Ok(self.regs[Self::TDATA1 as usize] = value),
            Self::TDATA2 => Ok(self.regs[Self::TDATA2 as usize] = value),
            Self::TDATA3 => Ok(self.regs[Self::TDATA3 as usize] = value),
            Self::TINFO => Ok(self.regs[Self::TINFO as usize] = value),
            Self::TCONTROL => Ok(self.regs[Self::TCONTROL as usize] = value),
            Self::MCONTEXT => Ok(self.regs[Self::MCONTEXT as usize] = value),
            Self::MSCONTEXT => Ok(self.regs[Self::MSCONTEXT as usize] = value),
            Self::DCSR => Ok(self.regs[Self::DCSR as usize] = value),
            Self::DPC => Ok(self.regs[Self::DPC as usize] = value),
            Self::DSCRATCH0 => Ok(self.regs[Self::DSCRATCH0 as usize] = value),
            Self::DSCRATCH1 => Ok(self.regs[Self::DSCRATCH1 as usize] = value),
            Self::MCYCLE => Ok(self.regs[Self::MCYCLE as usize] = value),
            Self::MINSTRET => Ok(self.regs[Self::MINSTRET as usize] = value),
            Self::MHPMCOUNTER3 => Ok(self.regs[Self::MHPMCOUNTER3 as usize] = value),
            Self::MHPMCOUNTER4 => Ok(self.regs[Self::MHPMCOUNTER4 as usize] = value),
            Self::MHPMCOUNTER5 => Ok(self.regs[Self::MHPMCOUNTER5 as usize] = value),
            Self::MHPMCOUNTER6 => Ok(self.regs[Self::MHPMCOUNTER6 as usize] = value),
            Self::MHPMCOUNTER7 => Ok(self.regs[Self::MHPMCOUNTER7 as usize] = value),
            Self::MHPMCOUNTER8 => Ok(self.regs[Self::MHPMCOUNTER8 as usize] = value),
            Self::MHPMCOUNTER9 => Ok(self.regs[Self::MHPMCOUNTER9 as usize] = value),
            Self::MHPMCOUNTER10 => Ok(self.regs[Self::MHPMCOUNTER10 as usize] = value),
            Self::MHPMCOUNTER11 => Ok(self.regs[Self::MHPMCOUNTER11 as usize] = value),
            Self::MHPMCOUNTER12 => Ok(self.regs[Self::MHPMCOUNTER12 as usize] = value),
            Self::MHPMCOUNTER13 => Ok(self.regs[Self::MHPMCOUNTER13 as usize] = value),
            Self::MHPMCOUNTER14 => Ok(self.regs[Self::MHPMCOUNTER14 as usize] = value),
            Self::MHPMCOUNTER15 => Ok(self.regs[Self::MHPMCOUNTER15 as usize] = value),
            Self::MHPMCOUNTER16 => Ok(self.regs[Self::MHPMCOUNTER16 as usize] = value),
            Self::MHPMCOUNTER17 => Ok(self.regs[Self::MHPMCOUNTER17 as usize] = value),
            Self::MHPMCOUNTER18 => Ok(self.regs[Self::MHPMCOUNTER18 as usize] = value),
            Self::MHPMCOUNTER19 => Ok(self.regs[Self::MHPMCOUNTER19 as usize] = value),
            Self::MHPMCOUNTER20 => Ok(self.regs[Self::MHPMCOUNTER20 as usize] = value),
            Self::MHPMCOUNTER21 => Ok(self.regs[Self::MHPMCOUNTER21 as usize] = value),
            Self::MHPMCOUNTER22 => Ok(self.regs[Self::MHPMCOUNTER22 as usize] = value),
            Self::MHPMCOUNTER23 => Ok(self.regs[Self::MHPMCOUNTER23 as usize] = value),
            Self::MHPMCOUNTER24 => Ok(self.regs[Self::MHPMCOUNTER24 as usize] = value),
            Self::MHPMCOUNTER25 => Ok(self.regs[Self::MHPMCOUNTER25 as usize] = value),
            Self::MHPMCOUNTER26 => Ok(self.regs[Self::MHPMCOUNTER26 as usize] = value),
            Self::MHPMCOUNTER27 => Ok(self.regs[Self::MHPMCOUNTER27 as usize] = value),
            Self::MHPMCOUNTER28 => Ok(self.regs[Self::MHPMCOUNTER28 as usize] = value),
            Self::MHPMCOUNTER29 => Ok(self.regs[Self::MHPMCOUNTER29 as usize] = value),
            Self::MHPMCOUNTER30 => Ok(self.regs[Self::MHPMCOUNTER30 as usize] = value),
            Self::MHPMCOUNTER31 => Ok(self.regs[Self::MHPMCOUNTER31 as usize] = value),
            Self::MCYCLECFG => Ok(self.regs[Self::MCYCLECFG as usize] = value),
            Self::MINSTRETCFG => Ok(self.regs[Self::MINSTRETCFG as usize] = value),
            Self::MHPMEVENT3 => Ok(self.regs[Self::MHPMEVENT3 as usize] = value),
            Self::MHPMEVENT4 => Ok(self.regs[Self::MHPMEVENT4 as usize] = value),
            Self::MHPMEVENT5 => Ok(self.regs[Self::MHPMEVENT5 as usize] = value),
            Self::MHPMEVENT6 => Ok(self.regs[Self::MHPMEVENT6 as usize] = value),
            Self::MHPMEVENT7 => Ok(self.regs[Self::MHPMEVENT7 as usize] = value),
            Self::MHPMEVENT8 => Ok(self.regs[Self::MHPMEVENT8 as usize] = value),
            Self::MHPMEVENT9 => Ok(self.regs[Self::MHPMEVENT9 as usize] = value),
            Self::MHPMEVENT10 => Ok(self.regs[Self::MHPMEVENT10 as usize] = value),
            Self::MHPMEVENT11 => Ok(self.regs[Self::MHPMEVENT11 as usize] = value),
            Self::MHPMEVENT12 => Ok(self.regs[Self::MHPMEVENT12 as usize] = value),
            Self::MHPMEVENT13 => Ok(self.regs[Self::MHPMEVENT13 as usize] = value),
            Self::MHPMEVENT14 => Ok(self.regs[Self::MHPMEVENT14 as usize] = value),
            Self::MHPMEVENT15 => Ok(self.regs[Self::MHPMEVENT15 as usize] = value),
            Self::MHPMEVENT16 => Ok(self.regs[Self::MHPMEVENT16 as usize] = value),
            Self::MHPMEVENT17 => Ok(self.regs[Self::MHPMEVENT17 as usize] = value),
            Self::MHPMEVENT18 => Ok(self.regs[Self::MHPMEVENT18 as usize] = value),
            Self::MHPMEVENT19 => Ok(self.regs[Self::MHPMEVENT19 as usize] = value),
            Self::MHPMEVENT20 => Ok(self.regs[Self::MHPMEVENT20 as usize] = value),
            Self::MHPMEVENT21 => Ok(self.regs[Self::MHPMEVENT21 as usize] = value),
            Self::MHPMEVENT22 => Ok(self.regs[Self::MHPMEVENT22 as usize] = value),
            Self::MHPMEVENT23 => Ok(self.regs[Self::MHPMEVENT23 as usize] = value),
            Self::MHPMEVENT24 => Ok(self.regs[Self::MHPMEVENT24 as usize] = value),
            Self::MHPMEVENT25 => Ok(self.regs[Self::MHPMEVENT25 as usize] = value),
            Self::MHPMEVENT26 => Ok(self.regs[Self::MHPMEVENT26 as usize] = value),
            Self::MHPMEVENT27 => Ok(self.regs[Self::MHPMEVENT27 as usize] = value),
            Self::MHPMEVENT28 => Ok(self.regs[Self::MHPMEVENT28 as usize] = value),
            Self::MHPMEVENT29 => Ok(self.regs[Self::MHPMEVENT29 as usize] = value),
            Self::MHPMEVENT30 => Ok(self.regs[Self::MHPMEVENT30 as usize] = value),
            Self::MHPMEVENT31 => Ok(self.regs[Self::MHPMEVENT31 as usize] = value),
            Self::MVENDORID => Ok(self.regs[Self::MVENDORID as usize] = value),
            Self::MARCHID => Ok(self.regs[Self::MARCHID as usize] = value),
            Self::MIMPID => Ok(self.regs[Self::MIMPID as usize] = value),
            Self::MHARTID => Ok(self.regs[Self::MHARTID as usize] = value),
            Self::MCONFIGPTR => Ok(self.regs[Self::MCONFIGPTR as usize] = value),
            Self::MTOPI => Ok(self.regs[Self::MTOPI as usize] = value),
            _ => Err(Error::UnknownCsr)
        }
    }
}
