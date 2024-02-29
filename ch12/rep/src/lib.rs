pub mod logger {
    pub enum LogLevel {
        Info,
        Warning,
        Error,
    }

    pub fn log(level: LogLevel, message: &str) {
        match level {
            LogLevel::Info => println!("[INFO]: {}", message),
            LogLevel::Warning => println!("[WARNING]: {}", message),
            LogLevel::Error => println!("[ERROR]: {}", message),
        }
    }
}
