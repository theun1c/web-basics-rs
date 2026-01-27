use std::{cell::RefCell, fs::{File, OpenOptions}, io::Write};
use chrono::Local;
use once_cell::sync::OnceCell;
use std::{str, sync::Mutex};

// logger path
const LOGGER_PATH: &str = "logs/logs.txt";

pub struct Logger {
    file: RefCell<File>,
}

impl Logger{
    pub fn new() -> std::io::Result<Self> {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .write(true)
            .open(LOGGER_PATH)?;
        Ok(Logger { file: RefCell::new(file) })
    }

    fn log(&self, text: &str, level: &str){
        let time = Local::now();
        let log_msg = format!("{} | {:<7} | {}\n", time.format("%Y.%m.%d %H:%M:%S"), level, text);
        let mut file = self.file.borrow_mut();
        file.write_all(log_msg.as_bytes());
    }

    pub fn error(&self, text: &str) {
        self.log(text, "ERROR");
    }

    pub fn warn(&mut self, text: &str){
        self.log(text, "WARN");
    }

    pub fn success(&mut self, text: &str){
        self.log(text, "SUCCESS");
    }
}


static LOGGER: OnceCell<Mutex<Logger>> = OnceCell::new();

pub fn get_logger() -> &'static Mutex<Logger>{
    LOGGER.get_or_init(|| {
        let logger = Logger::new().expect("logger creating error");
        Mutex::new(logger)
    })
}