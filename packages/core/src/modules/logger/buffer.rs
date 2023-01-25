use crate::{implements, injectable};

use crate::{
    cell::{Ref, RefMut},
    ILogger,
};

use super::{console_logger::ConsoleLogger, level::LogLevel};

#[derive(Clone)]
pub struct LogBufferRecord {
    pub level: LogLevel,
    pub msg: String,
    pub ctx: String,
}

impl LogBufferRecord {
    pub fn new<'a>(level: LogLevel, msg: &'a str, ctx: &'a str) -> LogBufferRecord {
        LogBufferRecord {
            level,
            msg: msg.to_owned(),
            ctx: ctx.to_owned(),
        }
    }
}

#[injectable]
#[blackbox_di(crate)]
pub struct LoggerBuffer {
    instance: RefMut<Ref<dyn ILogger>>,
    log_buffer: RefMut<Vec<LogBufferRecord>>,
    is_buffer_attached: RefMut<bool>,
}

#[implements]
#[blackbox_di(crate)]
impl LoggerBuffer {
    #[factory]
    pub fn new() -> LoggerBuffer {
        let instance = Ref::new(ConsoleLogger::new("App".to_string()))
            .cast::<dyn ILogger>()
            .unwrap();

        LoggerBuffer {
            log_buffer: RefMut::new(Vec::new()),
            instance: RefMut::new(instance),
            is_buffer_attached: RefMut::new(false),
        }
    }

    pub fn register_logger(&self, instance: Ref<dyn ILogger>) {
        *self.instance.as_mut() = instance;
    }

    pub fn attach_buffer(&self) {
        *self.is_buffer_attached.as_mut() = true;
    }

    pub fn detach_buffer(&self) {
        *self.is_buffer_attached.as_mut() = false;
    }

    pub fn is_attached(&self) -> bool {
        self.is_buffer_attached.as_ref().clone()
    }

    pub fn get_instance(&self) -> Ref<dyn ILogger> {
        self.instance.as_ref().clone()
    }

    pub fn get_logs(&self) -> Vec<LogBufferRecord> {
        self.log_buffer.as_ref().clone()
    }

    pub fn write_log<'a>(&self, level: LogLevel, msg: &'a str, ctx: &'a str) {
        self.log_buffer
            .as_mut()
            .push(LogBufferRecord::new(level.clone(), msg, ctx));
    }

    pub fn clear_logs(&self) {
        self.log_buffer.as_mut().clear();
    }
}
