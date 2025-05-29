use crate::{bus::Bus, cpu::{self, Cpu}, insn_impl::insn_cached};
use super::uop_cache::UopCacheEntry;

impl UopCacheEntry {
    pub fn set_cached_insn(bits: u64) -> Option<usize> {

        if bits & 0x7f == 0x37 {
            Some(0)
        }

        else if bits & 0x7f == 0x17 {
            Some(1)
        }

        else if bits & 0x7f == 0x6f {
            Some(2)
        }

        else if bits & 0x707f == 0x67 {
            Some(3)
        }

        else if bits & 0x707f == 0x63 {
            Some(4)
        }

        else if bits & 0x707f == 0x1063 {
            Some(5)
        }

        else if bits & 0x707f == 0x4063 {
            Some(6)
        }

        else if bits & 0x707f == 0x5063 {
            Some(7)
        }

        else if bits & 0x707f == 0x6063 {
            Some(8)
        }

        else if bits & 0x707f == 0x7063 {
            Some(9)
        }

        else if bits & 0x707f == 0x3 {
            Some(10)
        }

        else if bits & 0x707f == 0x1003 {
            Some(11)
        }

        else if bits & 0x707f == 0x2003 {
            Some(12)
        }

        else if bits & 0x707f == 0x4003 {
            Some(13)
        }

        else if bits & 0x707f == 0x5003 {
            Some(14)
        }

        else if bits & 0x707f == 0x23 {
            Some(15)
        }

        else if bits & 0x707f == 0x1023 {
            Some(16)
        }

        else if bits & 0x707f == 0x2023 {
            Some(17)
        }

        else if bits & 0x707f == 0x13 {
            Some(18)
        }

        else if bits & 0x707f == 0x2013 {
            Some(19)
        }

        else if bits & 0x707f == 0x3013 {
            Some(20)
        }

        else if bits & 0x707f == 0x4013 {
            Some(21)
        }

        else if bits & 0x707f == 0x6013 {
            Some(22)
        }

        else if bits & 0x707f == 0x7013 {
            Some(23)
        }

        else if bits & 0xfe00707f == 0x33 {
            Some(24)
        }

        else if bits & 0xfe00707f == 0x40000033 {
            Some(25)
        }

        else if bits & 0xfe00707f == 0x1033 {
            Some(26)
        }

        else if bits & 0xfe00707f == 0x2033 {
            Some(27)
        }

        else if bits & 0xfe00707f == 0x3033 {
            Some(28)
        }

        else if bits & 0xfe00707f == 0x4033 {
            Some(29)
        }

        else if bits & 0xfe00707f == 0x5033 {
            Some(30)
        }

        else if bits & 0xfe00707f == 0x40005033 {
            Some(31)
        }

        else if bits & 0xfe00707f == 0x6033 {
            Some(32)
        }

        else if bits & 0xfe00707f == 0x7033 {
            Some(33)
        }

        else if bits & 0x707f == 0xf {
            Some(34)
        }

        else if bits & 0xffffffff == 0x73 {
            Some(35)
        }

        else if bits & 0xffffffff == 0x100073 {
            Some(36)
        }

        else if bits & 0x707f == 0x6003 {
            Some(37)
        }

        else if bits & 0x707f == 0x3003 {
            Some(38)
        }

        else if bits & 0x707f == 0x3023 {
            Some(39)
        }

        else if bits & 0xfc00707f == 0x1013 {
            Some(40)
        }

        else if bits & 0xfc00707f == 0x5013 {
            Some(41)
        }

        else if bits & 0xfc00707f == 0x40005013 {
            Some(42)
        }

        else if bits & 0x707f == 0x1b {
            Some(43)
        }

        else if bits & 0xfe00707f == 0x101b {
            Some(44)
        }

        else if bits & 0xfe00707f == 0x501b {
            Some(45)
        }

        else if bits & 0xfe00707f == 0x4000501b {
            Some(46)
        }

        else if bits & 0xfe00707f == 0x3b {
            Some(47)
        }

        else if bits & 0xfe00707f == 0x4000003b {
            Some(48)
        }

        else if bits & 0xfe00707f == 0x103b {
            Some(49)
        }

        else if bits & 0xfe00707f == 0x503b {
            Some(50)
        }

        else if bits & 0xfe00707f == 0x4000503b {
            Some(51)
        }

        else if bits & 0x707f == 0x1073 {
            Some(52)
        }

        else if bits & 0x707f == 0x2073 {
            Some(53)
        }

        else if bits & 0x707f == 0x3073 {
            Some(54)
        }

        else if bits & 0x707f == 0x5073 {
            Some(55)
        }

        else if bits & 0x707f == 0x6073 {
            Some(56)
        }

        else if bits & 0x707f == 0x7073 {
            Some(57)
        }

        else if bits & 0xffffffff == 0x30200073 {
            Some(58)
        }

        else if bits & 0xffffffff == 0x10500073 {
            Some(59)
        }

        else if bits & 0xe003 == 0x0 {
            Some(60)
        }

        else if bits & 0xe003 == 0x4000 {
            Some(61)
        }

        else if bits & 0xe003 == 0xc000 {
            Some(62)
        }

        else if bits & 0xef83 == 0x1 {
            Some(63)
        }

        else if bits & 0xe003 == 0x1 {
            Some(64)
        }

        else if bits & 0xe003 == 0x4001 {
            Some(65)
        }

        else if bits & 0xef83 == 0x6101 {
            Some(66)
        }

        else if bits & 0xe003 == 0x6001 {
            Some(67)
        }

        else if bits & 0xec03 == 0x8801 {
            Some(68)
        }

        else if bits & 0xfc63 == 0x8c01 {
            Some(69)
        }

        else if bits & 0xfc63 == 0x8c21 {
            Some(70)
        }

        else if bits & 0xfc63 == 0x8c41 {
            Some(71)
        }

        else if bits & 0xfc63 == 0x8c61 {
            Some(72)
        }

        else if bits & 0xe003 == 0xa001 {
            Some(73)
        }

        else if bits & 0xe003 == 0xc001 {
            Some(74)
        }

        else if bits & 0xe003 == 0xe001 {
            Some(75)
        }

        else if bits & 0xe003 == 0x4002 {
            Some(76)
        }

        else if bits & 0xf07f == 0x8002 {
            Some(77)
        }

        else if bits & 0xf003 == 0x8002 {
            Some(78)
        }

        else if bits & 0xffff == 0x9002 {
            Some(79)
        }

        else if bits & 0xf07f == 0x9002 {
            Some(80)
        }

        else if bits & 0xf003 == 0x9002 {
            Some(81)
        }

        else if bits & 0xe003 == 0xc002 {
            Some(82)
        }

        else if bits & 0xe003 == 0x6000 {
            Some(83)
        }

        else if bits & 0xe003 == 0xe000 {
            Some(84)
        }

        else if bits & 0xe003 == 0x2001 {
            Some(85)
        }

        else if bits & 0xec03 == 0x8001 {
            Some(86)
        }

        else if bits & 0xec03 == 0x8401 {
            Some(87)
        }

        else if bits & 0xfc63 == 0x9c01 {
            Some(88)
        }

        else if bits & 0xfc63 == 0x9c21 {
            Some(89)
        }

        else if bits & 0xe003 == 0x2 {
            Some(90)
        }

        else if bits & 0xe003 == 0x6002 {
            Some(91)
        }

        else if bits & 0xe003 == 0xe002 {
            Some(92)
        }

        else if bits & 0x707f == 0x2007 {
            Some(93)
        }

        else if bits & 0x707f == 0x2027 {
            Some(94)
        }

        else if bits & 0x600007f == 0x43 {
            Some(95)
        }

        else if bits & 0x600007f == 0x47 {
            Some(96)
        }

        else if bits & 0x600007f == 0x4b {
            Some(97)
        }

        else if bits & 0x600007f == 0x4f {
            Some(98)
        }

        else if bits & 0xfe00007f == 0x53 {
            Some(99)
        }

        else if bits & 0xfe00007f == 0x8000053 {
            Some(100)
        }

        else if bits & 0xfe00007f == 0x10000053 {
            Some(101)
        }

        else if bits & 0xfe00007f == 0x18000053 {
            Some(102)
        }

        else if bits & 0xfff0007f == 0x58000053 {
            Some(103)
        }

        else if bits & 0xfe00707f == 0x20000053 {
            Some(104)
        }

        else if bits & 0xfe00707f == 0x20001053 {
            Some(105)
        }

        else if bits & 0xfe00707f == 0x20002053 {
            Some(106)
        }

        else if bits & 0xfe00707f == 0x28000053 {
            Some(107)
        }

        else if bits & 0xfe00707f == 0x28001053 {
            Some(108)
        }

        else if bits & 0xfff0007f == 0xc0000053 {
            Some(109)
        }

        else if bits & 0xfff0007f == 0xc0100053 {
            Some(110)
        }

        else if bits & 0xfff0707f == 0xe0000053 {
            Some(111)
        }

        else if bits & 0xfe00707f == 0xa0002053 {
            Some(112)
        }

        else if bits & 0xfe00707f == 0xa0001053 {
            Some(113)
        }

        else if bits & 0xfe00707f == 0xa0000053 {
            Some(114)
        }

        else if bits & 0xfff0707f == 0xe0001053 {
            Some(115)
        }

        else if bits & 0xfff0007f == 0xd0000053 {
            Some(116)
        }

        else if bits & 0xfff0007f == 0xd0100053 {
            Some(117)
        }

        else if bits & 0xfff0707f == 0xf0000053 {
            Some(118)
        }

        else if bits & 0xfff0007f == 0xc0200053 {
            Some(119)
        }

        else if bits & 0xfff0007f == 0xc0300053 {
            Some(120)
        }

        else if bits & 0xfff0007f == 0xd0200053 {
            Some(121)
        }

        else if bits & 0xfff0007f == 0xd0300053 {
            Some(122)
        }

        else if bits & 0x707f == 0x3007 {
            Some(123)
        }

        else if bits & 0x707f == 0x3027 {
            Some(124)
        }

        else if bits & 0x600007f == 0x2000043 {
            Some(125)
        }

        else if bits & 0x600007f == 0x2000047 {
            Some(126)
        }

        else if bits & 0x600007f == 0x200004b {
            Some(127)
        }

        else if bits & 0x600007f == 0x200004f {
            Some(128)
        }

        else if bits & 0xfe00007f == 0x2000053 {
            Some(129)
        }

        else if bits & 0xfe00007f == 0xa000053 {
            Some(130)
        }

        else if bits & 0xfe00007f == 0x12000053 {
            Some(131)
        }

        else if bits & 0xfe00007f == 0x1a000053 {
            Some(132)
        }

        else if bits & 0xfff0007f == 0x5a000053 {
            Some(133)
        }

        else if bits & 0xfe00707f == 0x22000053 {
            Some(134)
        }

        else if bits & 0xfe00707f == 0x22001053 {
            Some(135)
        }

        else if bits & 0xfe00707f == 0x22002053 {
            Some(136)
        }

        else if bits & 0xfe00707f == 0x2a000053 {
            Some(137)
        }

        else if bits & 0xfe00707f == 0x2a001053 {
            Some(138)
        }

        else if bits & 0xfff0007f == 0x40100053 {
            Some(139)
        }

        else if bits & 0xfff0007f == 0x42000053 {
            Some(140)
        }

        else if bits & 0xfe00707f == 0xa2002053 {
            Some(141)
        }

        else if bits & 0xfe00707f == 0xa2001053 {
            Some(142)
        }

        else if bits & 0xfe00707f == 0xa2000053 {
            Some(143)
        }

        else if bits & 0xfff0707f == 0xe2001053 {
            Some(144)
        }

        else if bits & 0xfff0007f == 0xc2000053 {
            Some(145)
        }

        else if bits & 0xfff0007f == 0xc2100053 {
            Some(146)
        }

        else if bits & 0xfff0007f == 0xd2000053 {
            Some(147)
        }

        else if bits & 0xfff0007f == 0xd2100053 {
            Some(148)
        }

        else if bits & 0xfff0007f == 0xc2200053 {
            Some(149)
        }

        else if bits & 0xfff0007f == 0xc2300053 {
            Some(150)
        }

        else if bits & 0xfff0707f == 0xe2000053 {
            Some(151)
        }

        else if bits & 0xfff0007f == 0xd2200053 {
            Some(152)
        }

        else if bits & 0xfff0007f == 0xd2300053 {
            Some(153)
        }

        else if bits & 0xfff0707f == 0xf2000053 {
            Some(154)
        }

        else if bits & 0xfe00707f == 0x2000033 {
            Some(155)
        }

        else if bits & 0xfe00707f == 0x2001033 {
            Some(156)
        }

        else if bits & 0xfe00707f == 0x2002033 {
            Some(157)
        }

        else if bits & 0xfe00707f == 0x2003033 {
            Some(158)
        }

        else if bits & 0xfe00707f == 0x2004033 {
            Some(159)
        }

        else if bits & 0xfe00707f == 0x2005033 {
            Some(160)
        }

        else if bits & 0xfe00707f == 0x2006033 {
            Some(161)
        }

        else if bits & 0xfe00707f == 0x2007033 {
            Some(162)
        }

        else if bits & 0xfe00707f == 0x200003b {
            Some(163)
        }

        else if bits & 0xfe00707f == 0x200403b {
            Some(164)
        }

        else if bits & 0xfe00707f == 0x200503b {
            Some(165)
        }

        else if bits & 0xfe00707f == 0x200603b {
            Some(166)
        }

        else if bits & 0xfe00707f == 0x200703b {
            Some(167)
        }

        else if bits & 0xe003 == 0x2000 {
            Some(168)
        }

        else if bits & 0xe003 == 0xa000 {
            Some(169)
        }

        else if bits & 0xe003 == 0x2002 {
            Some(170)
        }

        else if bits & 0xe003 == 0xa002 {
            Some(171)
        }

        else if bits & 0x707f == 0x100f {
            Some(172)
        }

        else {
            None
        }
    }
}

