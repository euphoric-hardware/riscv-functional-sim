use object::{
    elf::FileHeader64,
    read::elf::{FileHeader, SectionHeader, SectionTable},
    Endianness, Object, ObjectSection, ObjectSymbol, SectionFlags, SectionKind,
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

    pub fn extract_htif_from_symbols(&self) -> object::Result<(Option<u64>, Option<u64>)> {
        let obj = object::File::parse(&*self.data)?;
        let mut tohost_addr = None;
        let mut fromhost_addr = None;

        for symbol in obj.symbols() {
            let name = symbol.name()?;
            match name {
                "tohost" => {
                    tohost_addr = Some(symbol.address());
                }
                "fromhost" => {
                    fromhost_addr = Some(symbol.address());
                }
                _ => {}
            }
        }

        Ok((tohost_addr, fromhost_addr))
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

    pub fn extract_htif_addresses(&self) -> (u64, Option<u64>) {
        let (t, h) = match self.extract_htif_from_symbols() {
            // Extract from symbols
            Ok(htif_addrs) => htif_addrs,
            // Fall back to extract from sections
            Err(_) => (
                self.section_base_address(".tohost"),
                self.section_base_address(".fromhost"),
            ),
        };
        (t.expect("tohost not found in elf"), h)
    }

    pub fn extract_reset_vector(&self) -> (u64) {
        let bytes: [u8; 8] = [
            self.data[0x18],
            self.data[0x19],
            self.data[0x1a],
            self.data[0x1b],
            self.data[0x1c],
            self.data[0x1d],
            self.data[0x1e],
            self.data[0x1f],
        ];
        return u64::from_le_bytes(bytes);
    }

    pub fn extract_min_address(&self) -> u64 {
        let obj = object::File::parse(&*self.data).expect("data error");
        let mut start_addr: u64 = 0x80000000;
        for symbol in obj.symbols() {
            let name = symbol.name().expect("no symbol name");
            match name {
                "_start" => {
                    start_addr = symbol.address();
                }

                _ => {}
            }
        }
        start_addr
    }

    pub fn extract_max_address(&self) -> u64 {
        const PF_X: u32 = 0x1;
        let elf = goblin::elf::Elf::parse(&self.data).expect("Failed to parse ELF");

        let mut max_addr = 0;
        for ph in elf.program_headers.iter() {
            if ph.p_type == goblin::elf::program_header::PT_LOAD && (ph.p_flags & PF_X) != 0 {
                let end = ph.p_vaddr + ph.p_memsz;
                if end > max_addr {
                    max_addr = end;
                }
            }
        }
        
        max_addr
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
    //     let (tohost, fromhost) = elf.extract_htif_addresses();
    //     assert_eq!(tohost, 0x80000100);
    // }

    #[test]
    fn elf_hello() {
        let data = fs::read("tests/elf-hello/hello.riscv").unwrap();
        let elf = RiscvElf::try_new(data).unwrap();
        let (tohost, fromhost) = elf.extract_htif_addresses();
        assert_eq!(tohost, 0x80001e00);
        assert_eq!(fromhost.unwrap(), 0x80001e08);
    }
}
