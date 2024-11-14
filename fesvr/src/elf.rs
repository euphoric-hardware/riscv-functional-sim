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

    pub fn section_base_address(&self, name: &str) -> Option<u64> {
        let e = self.endianness();

        let sections = self.sections().ok()?;
        sections
            .iter()
            .find(|s| {
                String::from_utf8_lossy(sections.section_name(e, s).unwrap_or_default()) == name
            })
            .map(|s| s.sh_addr(e))
    }

    // tohost MUST exist; fromhost CAN exist
    pub fn extract_htif_addresses(&self) -> (u64, Option<u64>) {
        (
            self.section_base_address(".tohost")
                .expect("tohost not found in elf"),
            self.section_base_address(".fromhost"),
        )
    }
}

pub(crate) type ElfSectionTable64<'a> = SectionTable<'a, FileHeader64<Endianness>>;

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    // #[test]
    // fn elf_implicit_htif() {
    //     let data = fs::read("tests/elf-implicit/elf-implicit").unwrap();
    //     let elf = RiscvElf::try_new(data).unwrap();
    //     let ptr = elf.extract_htif_base().unwrap();
    //     assert_eq!(ptr, 0x80000000);
    // }

    // #[test]
    // fn elf_explicit_htif() {
    //     let data = fs::read("tests/elf-htif/elf-htif").unwrap();
    //     let elf = RiscvElf::try_new(data).unwrap();
    //     let ptr = elf.extract_htif_base().unwrap();
    //     assert_eq!(ptr, 0x80000100);
    // }
}
