use std::{fs::File, io::Write, time::SystemTime};
use once_cell::sync::OnceCell;

// logger path
const LOGGER_PATH: &str = "./logger/logger_file.txt";

struct Logger {

}

static LOGGER: OnceCell<Logger> = OnceCell::new();

impl Logger{
    fn new() -> Self {
        Logger {}
    }

    fn log(&self, text: &str, level: &str){
        let time = SystemTime::now();
        println!("{:?} | {} | {}", time, level, text);
    }

    fn error(&self, text: &str) {
        self.log(text, "ERROR");
    }

    fn warn(&self, text: &str){
        self.log(text, "WARN");
    }

    fn success(&self, text: &str){
        self.log(text, "SUCCESS");
    }
}

pub fn get_logger() -> &'static Logger{
    LOGGER.get_or_init(|| Logger::new())
}

fn main() { 
    let logger = get_logger();
    logger.error("some error");
}
