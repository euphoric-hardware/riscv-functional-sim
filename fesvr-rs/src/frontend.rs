use crate::{elf::RiscvElf, syscall::SyscallId, Error, Result, Syscall};
use log::info;
use object::{elf::SHT_PROGBITS, read::elf::SectionHeader as _};
use std::{
    fs::{self, File},
    io::Write,
    os::fd::FromRawFd as _,
    path::Path,
    time::Duration,
};

pub trait Htif {
    fn read(&self, ptr: u64, buf: &mut [u8]) -> Result<u64>;
    fn write(&self, ptr: u64, buf: &[u8]) -> Result<u64>;
}

pub struct Frontend<H> {
    htif: H,
    elf: RiscvElf,
    to_host: u64, // pointers
    from_host: u64,
}

impl<H: Htif> Frontend<H> {
    pub fn try_new(htif: H, elf_path: impl AsRef<Path>) -> Result<Self> {
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
    pub async fn write_elf(&self) -> Result<()> {
        let e = self.elf.endianness();

        for section in self.elf.sections()?.iter() {
            if section.sh_type(e) == SHT_PROGBITS && section.sh_addr(e) > 0 {
                let data = section.data(e, &*self.elf.data)?;

                // const CHUNK_SIZE: u64 = 1024; do .chunks() for progress bar later
                self.htif.write(section.sh_addr(e) as u64, &data)?;
            }
        }

        Ok(())
    }

    // todo(far): abstract this out
    pub async fn poll(&self, delay: Duration) -> Result<()> {
        loop {
            let mut buf = [0; size_of::<Syscall>()];
            self.htif.read(self.to_host, &mut buf)?;
            let syscall = Syscall::from_le_bytes(&buf);

            if let Some(syscall) = syscall {
                self.execute_syscall(syscall)?;

                // "signal chip that syscall processed" (taken from pyuartsi, verbatim)
                self.htif.write(self.to_host, &[0])?;
                self.htif.write(self.from_host, &[1])?;
            } else {
                info!("target attempted invalid syscall: {:?}", syscall);
            }

            std::thread::sleep(delay);
        }
    }

    // execute syscall on host
    fn execute_syscall(&self, syscall: Syscall) -> Result<()> {
        match syscall.syscall_id {
            SyscallId::Exit => {
                info!("target requested exit, exiting...");
                std::process::exit(0);
            }
            SyscallId::Write => {
                let (fd, ptr, len) = (syscall.arg0, syscall.arg1, syscall.arg2);

                let mut buf = vec![0; len as usize];
                self.htif.read(ptr, &mut buf)?;

                let fd = fd.try_into().map_err(|_| Error::InvalidSyscallArg {
                    arg_no: 0,
                    value: syscall.arg0,
                })?;
                let mut f = unsafe { File::from_raw_fd(fd) };

                f.write_all(&buf)
                    .map_err(|io_error| Error::SyscallFailed { io_error, syscall })
            }
        }
    }
}
