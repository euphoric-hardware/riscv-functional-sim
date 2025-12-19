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

        else {
            None
        }
    }
}

