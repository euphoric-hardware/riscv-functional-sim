use crate::Result;
use object::{
    elf::FileHeader64,
    read::elf::{FileHeader, SectionHeader, SectionTable},
    Endianness,
};

// wrapper for object's elf, which is quite annoying
pub struct RiscvElf {
    pub data: Vec<u8>,
    inner: FileHeader64<Endianness>, // owned fileheader
}

impl RiscvElf {
    pub fn try_new(data: Vec<u8>) -> object::Result<Self> {
        Ok(Self {
            inner: FileHeader64::<object::Endianness>::parse(&*data)?.to_owned(),
            data,
        })
    }

    pub fn endianness(&self) -> Endianness {
        self.inner.endian().expect("valid endianness")
    }

    pub fn sections(&self) -> object::Result<ElfSectionTable64> {
        self.inner.sections(self.endianness(), &self.data)
    }

    pub fn extract_htif_base(&self) -> Result<u64> {
        const HTIF_SECTION_NAME: &str = ".htif";
        const HTIF_BASE_ADDR: u64 = 0x80000000;

        let e = self.endianness(); // maybe make a macro for this lol
        let sections = self.sections()?;

        let htif_section = sections.iter().find(|s| {
            String::from_utf8_lossy(sections.section_name(e, s).unwrap_or_default())
                == HTIF_SECTION_NAME
        });

        Ok(htif_section.map_or(HTIF_BASE_ADDR, |hs| hs.sh_addr(e) as u64))
    }
}

pub(crate) type ElfSectionTable64<'a> = SectionTable<'a, FileHeader64<Endianness>>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn elf_implicit_htif() {
        let data = fs::read("tests/elf-implicit/elf-implicit").unwrap();
        let elf = RiscvElf::try_new(data).unwrap();
        let ptr = elf.extract_htif_base().unwrap();
        assert_eq!(ptr, 0x80000000);
    }

    #[test]
    fn elf_explicit_htif() {
        let data = fs::read("tests/elf-htif/elf-htif").unwrap();
        let elf = RiscvElf::try_new(data).unwrap();
        let ptr = elf.extract_htif_base().unwrap();
        assert_eq!(ptr, 0x80000100);
    }
}
