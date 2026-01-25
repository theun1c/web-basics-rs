use std::{fs::File, io::Write};
use chrono::Local;

// logger path
const LOGGER_PATH: &str = "./logger/logger_file.txt";

pub struct Logger {}

impl Logger{
    pub fn new() -> Self {
        Logger {}
    }

    fn log(&self, text: &str, level: &str){
        let time = Local::now();
        println!("{} | {} | {}", time.format("%Y.%m.%d %H:%M:%S"), level, text);
    }

    pub fn error(&self, text: &str) {
        self.log(text, "ERROR");
    }

    pub fn warn(&self, text: &str){
        self.log(text, "WARN");
    }

    pub fn success(&self, text: &str){
        self.log(text, "SUCCESS");
    }
}


