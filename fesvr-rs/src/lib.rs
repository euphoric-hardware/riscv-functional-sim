use object::{
    elf::{FileHeader64, SHT_PROGBITS},
    read::elf::{FileHeader, SectionHeader, SectionTable},
    Endianness,
};
use std::{error::Error, fs, path::Path, time::Duration};

pub enum Syscall {
    Write,
    Exit,
}

impl TryFrom<u8> for Syscall {
    type Error = (); // invalid syscall
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Write),
            2 => Ok(Self::Exit),
            _ => Err(()),
        }
    }
}

type Errno = i32;
type SysResult = Result<usize, Errno>;

impl Syscall {
    fn execute(&self) -> SysResult {
        match self {
            Self::Exit => {
                println!("exit");
                Ok(1)
            }
            Self::Write => {
                println!("write");
                Ok(1)
            }
        }
    }
}

trait Htif {
    async fn read(&self, ptr: usize, buf: &mut [u8]);
    async fn write(&self, ptr: usize, buf: &[u8]);
}

// not sure whether to keep this--will discuss later
// wrapper for object's elf, which is quite annoying
pub struct RiscvElf {
    data: Vec<u8>,
    inner: FileHeader64<Endianness>, // owned fileheader
}

impl RiscvElf {
    pub fn try_new(data: Vec<u8>) -> object::Result<Self> {
        Ok(Self {
            inner: FileHeader64::<object::Endianness>::parse(&*data)?.to_owned(),
            data,
        })
    }

    fn endianness(&self) -> Endianness {
        self.inner.endian().expect("valid endianness")
    }

    fn sections(&self) -> object::Result<ElfSectionTable64> {
        self.inner.sections(self.endianness(), &self.data)
    }

    pub fn extract_htif_base(&self) -> Result<usize, Box<dyn Error>> {
        const HTIF_SECTION_NAME: &str = ".htif";
        const HTIF_BASE_ADDR: usize = 0x80000000;

        let e = self.endianness(); // maybe make a macro for this lol
        let sections = self.sections()?;

        let htif_section = sections.iter().find(|s| {
            String::from_utf8_lossy(sections.section_name(e, s).expect("fix later"))
                == HTIF_SECTION_NAME
        });

        Ok(htif_section.map_or(HTIF_BASE_ADDR, |hs| hs.sh_addr(e) as usize))
    }
}

type ElfSectionTable64<'a> = SectionTable<'a, FileHeader64<Endianness>>;

struct Frontend<H> {
    htif: H,
    elf: RiscvElf,
    to_host: usize, // pointers
    from_host: usize,
}

impl<H: Htif> Frontend<H> {
    const POLL_DELAY_MS: u64 = 500;

    fn try_new(htif: H, elf_path: impl AsRef<Path>) -> Result<Self, Box<dyn Error>> {
        let elf_data = fs::read(elf_path)?; // add error ctxt later
        let elf = RiscvElf::try_new(elf_data)?;
        let htif_base = elf.extract_htif_base()?;

        Ok(Self {
            htif,
            elf,
            to_host: htif_base,
            from_host: htif_base + size_of::<u64>(),
        })
    }

    // write appropriate sections of elf into memory
    async fn write_elf(&self) -> Result<(), Box<dyn Error>> {
        let e = self.elf.endianness();

        for section in self.elf.sections()?.iter() {
            if section.sh_type(e) == SHT_PROGBITS && section.sh_addr(e) > 0 {
                let data = section.data(e, &*self.elf.data)?;

                // const CHUNK_SIZE: usize = 1024; do .chunks() for progress bar later
                self.htif.write(section.sh_addr(e) as usize, &data).await;
            }
        }

        Ok(())
    }

    // todo(far): abstract this out
    async fn poll(&self) {
        let delay = Duration::from_millis(Self::POLL_DELAY_MS);
        loop {
            // dummy
            let mut buf = [0; 1];
            self.htif.read(self.to_host, &mut buf).await;

            if let Ok(syscall) = Syscall::try_from(buf[0]) {
                let res = syscall.execute();
                let to_send = res.unwrap_or_else(|e| e as usize).to_ne_bytes(); // not sure if this is intended behavior

                self.htif.write(self.from_host, &to_send).await;
            } else {
                println!("invalid syscall");
            }

            tokio::time::sleep(delay).await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
