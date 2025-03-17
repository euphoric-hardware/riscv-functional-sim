use crate::{elf::RiscvElf, syscall::SyscallId, Error, Result, Syscall};
use log::info;
use object::{elf::SHT_PROGBITS, read::elf::SectionHeader as _};
use std::{
    cmp::min, fs::{self, File}, io::{self, Write}, os::fd::FromRawFd as _, path::Path
};

pub trait Htif {
    /// Chunk up read transactions based on the address alignment scheme that the target expects
    /// - verbatim from memif.cc in spike
    fn read(&mut self, ptr: u64, buf: &mut [u8]) -> Result<()> {

        let mut len = buf.len();
        let mut addr = ptr;
        let mut buf_ = buf;

        // chunk start
        if (len > 0) && (addr & (self.align() - 1) != 0) {
            let this_len = min(len, (self.align() - (addr & (self.align() - 1))) as usize);
            let mut chunk = vec![0u8; self.align() as usize];
            self.read_chunk(addr & !(self.align() - 1), &mut chunk)?;
            for i in 0..this_len {
                buf_[i] = chunk[(addr & (self.align() - 1)) as usize + i];
            }

            addr += this_len as u64;
            len -= this_len;
            buf_ = &mut buf_[this_len..];
        }

        // chunk end
        if len as u64 & (self.align() - 1) != 0 {
            let this_len = len as u64 & (self.align() - 1);
            let start = len as u64 - this_len;
            let mut chunk = vec![0u8; self.align() as usize];
            self.read_chunk(addr + start, &mut chunk)?;
            for i in 0..this_len {
                buf_[(start + i) as usize] = chunk[i as usize];
            }
            len -= this_len as usize;
        }

        // aligned parts
        for pos in (0..len).step_by(self.max_chunk_bytes() as usize) {
            let start = addr + pos as u64;
            let cur_len = min(self.max_chunk_bytes() as usize, len - pos) as usize;
            self.read_chunk(start, &mut buf_[pos..pos + cur_len])?;
        }

        return Ok(())
    }

    /// Chunk up write transactions based on the address alignment scheme that the target expects
    /// - verbatim from memif.cc in spike
    fn write(&mut self, ptr: u64, buf: &[u8]) -> Result<()> {
        let mut buf_ = buf;
        let mut len = buf.len();
        let mut addr = ptr;

        // chunk start
        if (len > 0) && (addr & (self.align() - 1) != 0) {
            let this_len = min(len, (self.align() - (addr & (self.align() - 1))) as usize);
            let mut chunk = vec![0u8; self.align() as usize];
            self.read_chunk(addr & !(self.align() - 1), &mut chunk)?;
            for i in 0..this_len {
                chunk[(addr & (self.align() - 1)) as usize + i] = buf_[i];
            }
            self.write_chunk(addr & !(self.align() - 1), &chunk)?;

            buf_ = &buf[this_len..];
            addr += this_len as u64;
            len -= this_len;
        }

        // chunk end
        if len as u64 & (self.align() - 1) != 0 {
            let this_len = len as u64 & (self.align() - 1);
            let start = len as u64 - this_len;
            let mut chunk = vec![0u8; self.align() as usize];
            self.read_chunk(addr + start, &mut chunk)?;
            for i in 0..this_len {
                chunk[i as usize] = buf_[(start + i) as usize];
            }
            self.write_chunk(addr + start, &chunk)?;
            len -= this_len as usize;
        }


        // aligned
        for pos in (0..len).step_by(self.max_chunk_bytes() as usize) {
            let start = addr + pos as u64;
            let cur_len = min(self.max_chunk_bytes() as usize, len - pos) as usize;
            self.write_chunk(start, &buf_[pos..pos + cur_len])?;
        }

        return Ok(());
    }

    /// Target expects that the read/write addresses align with this value
    fn align(&self) -> u64;

    /// Maximum bytes that a transaction can handle
    fn max_chunk_bytes(&self) -> u64;

    fn read_chunk(&mut self, ptr: u64, buf: &mut [u8]) -> Result<()>;
    fn write_chunk(&mut self, ptr: u64, buf: &[u8]) -> Result<()>;
}


#[derive(Debug, Clone, PartialEq)]
pub enum FrontendReturnCode {
    Continue,
    Exit,
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
    const MSIP_BASE: u64 = 0x2000000;
    const CHUNK_SIZE_BYTES: u64 = 1024;

    pub fn try_new(elf_path: impl AsRef<Path>) -> Result<Self> {
        let elf_data = fs::read(elf_path)?; // add error ctxt later
        let elf = RiscvElf::try_new(elf_data)?;
        let (to_host, from_host) = elf.extract_htif_addresses();

        Ok(Self {
            elf,
            to_host,
            from_host
        })
    }

    pub fn reset<H: Htif>(&self, htif: &mut H) -> Result<()> {
        htif.write(Self::MSIP_BASE, &[1])?;
        Ok(())
    }

    // write appropriate sections of elf into memory
    pub fn write_elf<H: Htif>(&self, htif: &mut H) -> Result<()> {
        let e = self.elf.endianness();

        for section in self.elf.sections()?.iter() {
            if section.sh_type(e) == SHT_PROGBITS && section.sh_addr(e) > 0 {
                let data = section.data(e, &*self.elf.data)?;

                let data_chunks = data.chunks(Self::CHUNK_SIZE_BYTES as usize);
                let mut addr = section.sh_addr(e) as u64;
                for chunk in data_chunks {
                    htif.write(addr, &chunk)?;
                    addr += chunk.len() as u64;
                }
            }
        }

        Ok(())
    }

    pub fn process<H: Htif>(&mut self, htif: &mut H) -> Result<FrontendReturnCode> {
        let mut buf = [0; size_of::<u64>()];
        htif.read(self.to_host, &mut buf)?;
        let tohost = u64::from_le_bytes(buf);
        // todo: implement all of https://github.com/riscv-software-src/riscv-isa-sim/issues/364#issuecomment-607657754

        // check if payload bottom bit is 1
        if (tohost & 0x1 == 1) {
            return Ok(FrontendReturnCode::Exit);
        }
        
        match tohost {
            1 => Ok(FrontendReturnCode::Exit),
            0 => Ok(FrontendReturnCode::Continue),
            a => {
                htif.write(self.to_host, &[0; size_of::<u64>()])?;
                self.dispatch_syscall(u64::from_le_bytes(buf), htif)?;
                htif.write(self.from_host.unwrap(), &[1])?;
                Ok(FrontendReturnCode::Continue)
            }
        }
    }

    fn dispatch_syscall<H: Htif>(&mut self, tohost: u64, htif: &mut H) -> Result<()> {
        let addr = tohost;
        let mut magicmem = [0u8; 64];
        htif.read(addr, &mut magicmem)?;

        let sc_opt = Syscall::from_le_bytes(&magicmem);
        match sc_opt {
            Some(sc) => {
                let rc = self.execute_syscall(sc, htif)?;
                magicmem[0..8].copy_from_slice(&rc.to_le_bytes());
                htif.write(addr, &mut magicmem)?;
            }
            _ => {
                return Err(Error::Misc);
            }
        }

        return Ok(());
    }

    // execute syscall on host
    fn execute_syscall<H: Htif>(&mut self, syscall: Syscall, htif: &mut H) -> Result<u64> {
        match syscall.syscall_id {
            SyscallId::Exit => {
                info!("target requested exit, exiting...");
                std::process::exit(0);
            }
            SyscallId::Write => {
                let (fd, ptr, len) = (syscall.arg0, syscall.arg1, syscall.arg2);

                let mut buf = vec![0; len as usize];
                htif.read(ptr, &mut buf)?;

                if fd == 1 {
                    let mut f = io::stdout();
                    self.write_buf(&mut f, &buf)?;
                } else if fd == 2 {
                    let mut f = io::stderr();
                    self.write_buf(&mut f, &buf)?;
                } else {
                    let fd = fd.try_into().map_err(|_| Error::InvalidSyscallArg {
                        arg_no: 0,
                        value: syscall.arg0,
                    })?;
                    let mut f = unsafe { File::from_raw_fd(fd) };
                    self.write_buf(&mut f, &buf)?;
                }
                return Ok(len);
            }
        }
    }

    fn write_buf<Wr: Write>(&self, f: &mut Wr, buf: &[u8]) -> Result<u64> {
        match f.write_all(&buf) {
            Ok(_) => {
                Ok(buf.len() as u64)
            }
            Err(io_error) => {
                Err(Error::IoError(io_error))
            }
        }
    }
}
