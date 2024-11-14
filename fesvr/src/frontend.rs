use crate::{elf::RiscvElf, syscall::SyscallId, Error, Result, Syscall};
use log::info;
use object::{elf::SHT_PROGBITS, read::elf::SectionHeader as _};
use std::{
    fs::{self, File},
    io::Write,
    os::fd::FromRawFd as _,
    path::Path,
};

pub trait Htif {
    fn read(&mut self, ptr: u64, buf: &mut [u8]) -> Result<()>;
    fn write(&mut self, ptr: u64, buf: &[u8]) -> Result<()>;
}

pub struct Frontend {
    elf: RiscvElf,
    to_host: u64, // pointers
    from_host: Option<u64>,
}

impl std::fmt::Debug for Frontend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Frontend")
            .field("to_host", &self.to_host)
            .field("from_host", &self.from_host)
            .finish()
    }
}

impl Frontend {
    pub fn try_new(elf_path: impl AsRef<Path>) -> Result<Self> {
        let elf_data = fs::read(elf_path)?; // add error ctxt later
        let elf = RiscvElf::try_new(elf_data)?;
        let (to_host, from_host) = elf.extract_htif_addresses();

        Ok(Self {
            elf,
            to_host,
            from_host,
        })
    }

    // write appropriate sections of elf into memory
    pub fn write_elf<H: Htif>(&self, htif: &mut H) -> Result<()> {
        let e = self.elf.endianness();

        for section in self.elf.sections()?.iter() {
            if section.sh_type(e) == SHT_PROGBITS && section.sh_addr(e) > 0 {
                let data = section.data(e, &*self.elf.data)?;

                // const CHUNK_SIZE: u64 = 1024; do .chunks() for progress bar later
                htif.write(section.sh_addr(e) as u64, &data)?;
            }
        }

        Ok(())
    }

    pub fn process<H: Htif>(&mut self, htif: &mut H) -> Result<()> {
        let mut buf = [0; size_of::<u64>()];
        htif.read(self.to_host, &mut buf)?;
        let tohost = u64::from_le_bytes(buf);
        // todo: implement all of https://github.com/riscv-software-src/riscv-isa-sim/issues/364#issuecomment-607657754
        if tohost & 1 == 1 {
            std::process::exit(0);
        } else {
            Ok(())
        }

        // if let Some(syscall) = syscall {
        //     self.execute_syscall(syscall, htif)?;

        //     // "signal chip that syscall processed" (taken from pyuartsi, verbatim)
        //     htif.write(self.to_host, &[0])?;
        //     if let Some(from_host) = self.from_host {
        //         htif.write(from_host, &[1])?;
        //     }
        //     Ok(())
        // } else {
        //     // nothing there
        //     Ok(())
        // }
    }

    // execute syscall on host
    fn execute_syscall<H: Htif>(&mut self, syscall: Syscall, htif: &mut H) -> Result<()> {
        match syscall.syscall_id {
            SyscallId::Exit => {
                info!("target requested exit, exiting...");
                std::process::exit(0);
            }
            SyscallId::Write => {
                let (fd, ptr, len) = (syscall.arg0, syscall.arg1, syscall.arg2);

                let mut buf = vec![0; len as usize];
                htif.read(ptr, &mut buf)?;

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
