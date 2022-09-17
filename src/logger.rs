use crossterm::style::Stylize;

pub enum LogLevel {
    Debug,
    Info,
    Warn,
    Error
}

impl Default for LogLevel {
    fn default() -> Self {
        Self::Info
    }
}

impl Copy for LogLevel {}

impl Clone for LogLevel {
    fn clone(&self) -> Self {
        return *self;
    }
}

pub struct Logger {
    log_level: LogLevel,
    debug_enabled: bool
}

impl Logger {
    pub const fn new(log_level: LogLevel, debug_enabled: bool) -> Self { Self { log_level, debug_enabled } }

    pub fn log(&self, message: String) {
        match self.log_level {
            LogLevel::Debug => self.debug(message),
            LogLevel::Info => self.info(message),
            LogLevel::Warn => self.warn(message),
            LogLevel::Error => self.error(message)
        }
    }

    pub fn debug(&self, message: String) {
        if self.debug_enabled {
            println!("[{}] {}", format!("DEBUG").dark_blue(), message);
        }
    }

    pub fn info(&self, message: String) {
        println!("[{}] {}", format!("GREEN").dark_green(),  message);
    }

    pub fn warn(&self, message: String) {
        println!("[{}] {}", format!("WARN").dark_yellow(),  message);
    }
    
    pub fn error(&self, message: String) {
        println!("[{}] {}", format!("ERROR").dark_red(),  message);
    }
}