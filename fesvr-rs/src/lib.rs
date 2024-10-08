use object::{
    elf::{FileHeader64, SHT_PROGBITS},
    read::elf::{FileHeader, SectionHeader, SectionTable},
    Endianness,
};
use std::{error::Error, fs, mem, os::fd::FromRawFd, path::Path, time::Duration};
use tokio::{
    fs::File,
    io::{AsyncWrite, AsyncWriteExt},
};

#[repr(u64)]
pub enum SyscallId {
    Write,
    Exit,
}

impl TryFrom<u64> for SyscallId {
    type Error = (); // invalid syscall
    fn try_from(value: u64) -> Result<Self, Self::Error> {
        match value {
            64 => Ok(Self::Write),
            93 => Ok(Self::Exit),
            _ => Err(()),
        }
    }
}

type Errno = i32;
type SysResult = Result<u64, Errno>;

#[repr(packed)]
struct Syscall {
    syscall_id: SyscallId,
    arg0: u64,
    arg1: u64,
    arg2: u64, // max(args(syscall) for syscalls) = 3 (write)
}

impl Syscall {
    // target system (riscv) little endian?
    fn from_le_bytes(bytes: &[u8]) -> Option<Self> {
        if bytes.len() < 32 {
            return None;
        }

        Some(Syscall {
            syscall_id: SyscallId::try_from(u64::from_le_bytes(bytes[0..8].try_into().ok()?))
                .ok()?,
            arg0: u64::from_le_bytes(bytes[8..16].try_into().ok()?),
            arg1: u64::from_le_bytes(bytes[16..24].try_into().ok()?),
            arg2: u64::from_le_bytes(bytes[24..32].try_into().ok()?),
        })
    }
}

pub trait Htif {
    async fn read(&self, ptr: u64, buf: &mut [u8]) -> Result<usize, Box<dyn Error>>;
    async fn write(&self, ptr: u64, buf: &[u8]) -> Result<usize, Box<dyn Error>>;
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

    pub fn extract_htif_base(&self) -> Result<u64, Box<dyn Error>> {
        const HTIF_SECTION_NAME: &str = ".htif";
        const HTIF_BASE_ADDR: u64 = 0x80000000;

        let e = self.endianness(); // maybe make a macro for this lol
        let sections = self.sections()?;

        let htif_section = sections.iter().find(|s| {
            String::from_utf8_lossy(sections.section_name(e, s).expect("fix later"))
                == HTIF_SECTION_NAME
        });

        Ok(htif_section.map_or(HTIF_BASE_ADDR, |hs| hs.sh_addr(e) as u64))
    }
}

type ElfSectionTable64<'a> = SectionTable<'a, FileHeader64<Endianness>>;

struct Frontend<H> {
    htif: H,
    elf: RiscvElf,
    to_host: u64, // pointers
    from_host: u64,
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
            from_host: htif_base + size_of::<u64>() as u64,
        })
    }

    // write appropriate sections of elf into memory
    async fn write_elf(&self) -> Result<(), Box<dyn Error>> {
        let e = self.elf.endianness();

        for section in self.elf.sections()?.iter() {
            if section.sh_type(e) == SHT_PROGBITS && section.sh_addr(e) > 0 {
                let data = section.data(e, &*self.elf.data)?;

                // const CHUNK_SIZE: u64 = 1024; do .chunks() for progress bar later
                self.htif.write(section.sh_addr(e) as u64, &data).await?;
            }
        }

        Ok(())
    }

    // todo(far): abstract this out
    async fn poll(&self) -> Result<(), Box<dyn Error>> {
        let delay = Duration::from_millis(Self::POLL_DELAY_MS);
        loop {
            let mut buf = [0; size_of::<Syscall>()];
            self.htif.read(self.to_host, &mut buf).await?;

            if let Some(syscall) = Syscall::from_le_bytes(&buf) {
                self.execute_syscall(syscall).await?;

                // "signal chip that syscall processed" (taken from pyuartsi, verbatim)
                self.htif.write(self.to_host, &[0]).await?;
                self.htif.write(self.from_host, &[1]).await?;
            } else {
                println!("invalid syscall");
            }

            tokio::time::sleep(delay).await;
        }
    }

    // execute syscall on host
    async fn execute_syscall(&self, syscall: Syscall) -> Result<(), Box<dyn Error>> {
        match syscall.syscall_id {
            SyscallId::Exit => {
                println!("exiting...");
                std::process::exit(0);
            }
            SyscallId::Write => {
                let (fd, ptr, len) = (syscall.arg0, syscall.arg1, syscall.arg2);

                let mut buf = vec![0; len as usize];
                self.htif.read(ptr, &mut buf).await?;

                let mut f = unsafe { File::from_raw_fd(fd.try_into().expect("valid fd")) };
                f.write_all(&buf).await?;
            }
        }
        Ok(())
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
