use std::fs::File;
use std::path::Path;
use env_logger::Builder;
use log::LevelFilter;
use std::io::{Write, Stdout};

pub fn init_logger(enable_logging: bool, filename: &str) {
    if enable_logging {
        let file = File::create(filename).expect("Failed to create log file");

        Builder::new()
            .filter_level(LevelFilter::Info)
            .format(|buf, record| {
                // Write just the message part without level, tags, or newlines
                write!(buf, "{}", record.args()).unwrap(); // No newline
                Ok(())
            })
            .target(env_logger::Target::Pipe(Box::new(file))) // Logs to file
            .init();
    }
}