#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash)]
pub enum LogLevel {
    EMERG,
    ALERT,
    CRIT,
    ERROR,
    WARN,
    NOTICE,
    INFO,
    DEBUG,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogLevel::EMERG => write!(f, "EMERG"),
            LogLevel::ALERT => write!(f, "ALERT"),
            LogLevel::CRIT => write!(f, "CRIT"),
            LogLevel::ERROR => write!(f, "ERROR"),
            LogLevel::WARN => write!(f, "WARN"),
            LogLevel::NOTICE => write!(f, "NOTICE"),
            LogLevel::INFO => write!(f, "INFO"),
            LogLevel::DEBUG => write!(f, "DEBUG"),
        }
    }
}
