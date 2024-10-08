use std::time::Duration;

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
    const POLL_DELAY_MS: u64;

    fn read(&self, ptr: usize, buf: &mut [u8]);
    fn write(&self, ptr: usize, buf: &[u8]);

    // get from_host, to_host addresses
    fn from_host(&self) -> usize;
    fn to_host(&self) -> usize;

    async fn poll(&self) {
        let delay = Duration::from_millis(Self::POLL_DELAY_MS);
        loop {
            // dummy
            let mut buf = [0; 1];
            self.read(self.to_host(), &mut buf);

            if let Ok(syscall) = Syscall::try_from(buf[0]) {
                let res = syscall.execute();
                let to_send = res.unwrap_or_else(|e| e as usize).to_ne_bytes(); // not sure if this is intended behavior

                self.write(self.from_host(), &to_send);
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

    // tests
}
