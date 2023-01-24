pub mod logger;

use crate::module;

use self::logger::LoggerModule;

#[module]
#[global]
#[blackbox_di(crate)]
pub struct CoreModule {
    #[import]
    pub logger_module: LoggerModule,
}
