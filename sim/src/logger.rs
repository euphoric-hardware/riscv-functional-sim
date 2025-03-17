use slog::{Drain, Logger, OwnedKVList, Record};
use slog_async;
use lazy_static::lazy_static;
use std::fs::OpenOptions;
use std::sync::Mutex;
use std::io::{BufWriter, Write};

lazy_static! {
    static ref LOGGER: Mutex<Option<Logger>> = Mutex::new(None);
}

// Custom drain that only logs raw message text without adding newlines
struct RawMessageDrain<W: Write + Send + 'static>(Mutex<W>);

impl<W: Write + Send + 'static> Drain for RawMessageDrain<W> {
    type Ok = ();
    type Err = std::io::Error;

    fn log(&self, record: &Record, _values: &OwnedKVList) -> Result<(), Self::Err> {
        let mut writer = self.0.lock().unwrap();
        write!(writer, "{}", record.msg())?; // No automatic newline
        writer.flush() // Ensure immediate write
    }
}

// Initialize the logger with a larger async buffer
pub fn init_logger(log_file: &str) {
    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(log_file)
        .expect("Failed to open log file");

    let file = BufWriter::new(file);
    let drain = RawMessageDrain(Mutex::new(file)).fuse();
    
    // Increase buffer size to 10,000 messages
    let drain = slog_async::Async::new(drain).chan_size(10_000).build().fuse();

    let logger = Logger::root(drain, slog::o!());

    let mut global_logger = LOGGER.lock().unwrap();
    *global_logger = Some(logger);
}

// Function to get the global logger
pub fn get_logger() -> Logger {
    let global_logger = LOGGER.lock().unwrap();
    global_logger.clone().expect("Logger not initialized")
}