mod logger;
use once_cell::sync::OnceCell;
use logger::Logger;

static LOGGER: OnceCell<Logger> = OnceCell::new();

pub fn get_logger() -> &'static Logger{
    LOGGER.get_or_init(|| Logger::new())
}

fn main() { 
    let logger = get_logger();
    logger.error("Ошибка 123");
}
