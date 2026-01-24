use std::fs::File;

fn main() -> Result<(), std::io::Error>{ 
    let mut file = match File::create("Hello.txt") {
        Ok(f) => f,
        Err(e) => return Err(e), // ru: чтобы вернуть Result Err 
        // нужно использовать -> Result<(), std::io::Error в main
    };

    Ok(())
}