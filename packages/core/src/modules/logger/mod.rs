mod buffer;
mod console_logger;
mod interface;
mod level;
mod logger;

use crate::module;

use self::buffer::LoggerBuffer;

pub use self::interface::ILogger;
pub use self::level::LogLevel;
pub use self::logger::Logger;

#[module]
#[blackbox_di(crate)]
pub struct LoggerModule {
    #[provider]
    pub buffer: LoggerBuffer,

    #[provider]
    #[export]
    pub logger: Logger,
}
