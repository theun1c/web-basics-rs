use std::fs::File;

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
fn error_log(){

}

// greem highlight
fn succesful_log(){

}

// yellow highlight
fn warn_log(){ 

}

fn main() { 
    create_logger_file();
}