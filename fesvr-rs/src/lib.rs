use object::{
    elf::{self, SHT_PROGBITS, SHT_SYMTAB},
    read::elf::{FileHeader, SectionHeader, Sym},
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

fn get_symbol_value(
    symbols: &object::read::elf::SymbolTable<'_, elf::FileHeader64<object::Endianness>>,
    symbol_name: &str,
    endian: Endianness,
) -> u64 {
    symbols
        .iter()
        .find(|s| {
            s.name(endian, symbols.strings())
                .map(String::from_utf8_lossy)
                .map_or(false, |s| s == "fromhost")
        })
        .expect(&format!("elf should have {}", symbol_name)) // clean up later
        .st_value(endian)
}

trait Htif {
    const POLL_DELAY_MS: u64;

    // get from_host, to_host addresses
    fn from_host(&self) -> usize;
    fn to_host(&self) -> usize;

    fn set_from_host(&self, ptr: usize);
    fn set_to_host(&self, ptr: usize);

    async fn read(&self, ptr: usize, buf: &mut [u8]);
    async fn write(&self, ptr: usize, buf: &[u8]);

    async fn poll(&self) {
        let delay = Duration::from_millis(Self::POLL_DELAY_MS);
        loop {
            // dummy
            let mut buf = [0; 1];
            self.read(self.to_host(), &mut buf).await;

            if let Ok(syscall) = Syscall::try_from(buf[0]) {
                let res = syscall.execute();
                let to_send = res.unwrap_or_else(|e| e as usize).to_ne_bytes(); // not sure if this is intended behavior

                self.write(self.from_host(), &to_send).await;
            } else {
                println!("invalid syscall");
            }

            tokio::time::sleep(delay).await;
        }
    }

    async fn load_elf(&self, path: impl AsRef<Path>) -> Result<(), Box<dyn Error>> {
        let data = fs::read(path)?;
        let elf = elf::FileHeader64::<object::Endianness>::parse(&*data)?;
        let endian = elf.endian()?;
        let sections = elf.sections(endian, &*data)?;

        for section in sections.iter() {
            if section.sh_type(endian) == SHT_PROGBITS && section.sh_addr(endian) > 0 {
                let data = section.data(endian, &*data)?;

                // const CHUNK_SIZE: usize = 1024; do .chunks() for progress bar later
                self.write(section.sh_addr(endian) as usize, &data).await;
            }
        }

        let symbols = sections.symbols(endian, &*data, SHT_SYMTAB)?;

        let from_host_addr = get_symbol_value(&symbols, "fromhost", endian) as usize;
        let to_host_addr = get_symbol_value(&symbols, "tohost", endian) as usize;

        self.set_from_host(from_host_addr);
        self.set_from_host(to_host_addr);

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // tests
}
