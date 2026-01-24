use std::{fs::File, io::Write, time::SystemTime};

// logger path
const LOG_PATH: &str = "./logger/logger_file.txt";

// creating a file in the logger folder
fn create_logger_file(){
    match File::create(LOG_PATH){
        Ok(..) => println!("succsesful create file!"),
        Err(..) => println!("error while create a file!")
    };
}

// red highlight 
fn error_log(text: &str, mut file: File){
    let time = SystemTime::now();
    let format_str = format!("{:?} | ERROR | {}", time, text);
    file.write_all(format_str.as_bytes());
}

// greem highlight
fn succesful_log(text: &str, mut file: File){
    let time = SystemTime::now();
    let format_str = format!("{:?} | WARN | {}", time, text);
    file.write_all(format_str.as_bytes());
}

// yellow highlight
fn warn_log(text: &str, mut file: File){ 
    let time = SystemTime::now();
    let format_str = format!("{:?} | SUCCSES | {}", time, text);
    file.write_all(format_str.as_bytes());
}

fn main() { 
    create_logger_file();
}