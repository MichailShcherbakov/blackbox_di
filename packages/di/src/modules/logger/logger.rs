use di_codegen::{implements, injectable};

use crate::{cell::RefMut, reference::Ref};

use super::{
    buffer::LoggerBuffer, console_logger::DEFAULT_LOGGER_CONTEXT, interface::ILogger,
    level::LogLevel,
};

#[injectable]
#[scope(Transient)]
#[blackbox_di(crate)]
pub struct Logger {
    #[inject]
    buffer: Ref<LoggerBuffer>,

    context: RefMut<String>,
}

#[implements]
#[blackbox_di(crate)]
impl Logger {
    #[factory]
    pub fn new(buffer: Ref<LoggerBuffer>) -> Logger {
        let context = RefMut::new(String::from(DEFAULT_LOGGER_CONTEXT));

        Logger { buffer, context }
    }

    pub fn register_logger(&self, instance: Ref<dyn ILogger>) {
        self.buffer.register_logger(instance);
    }

    pub fn attach_buffer(&self) {
        self.buffer.attach_buffer();
    }

    pub fn detach_buffer(&self) {
        self.buffer.detach_buffer();
    }

    pub fn flush(&self) {
        self.detach_buffer();

        for log_record in self.buffer.get_logs() {
            self.log(log_record.level, log_record.msg.as_str())
        }

        self.buffer.clear_logs();
    }
}

#[implements]
#[blackbox_di(crate)]
impl ILogger for Logger {
    fn log<'a>(&self, level: LogLevel, msg: &'a str) {
        let ctx = self.context.as_ref();

        if self.buffer.is_attached() {
            return self.buffer.write_log(level, msg, ctx.as_str());
        };

        let logger_instance = self.buffer.get_instance();

        logger_instance.log_with_ctx(level, msg, ctx.as_str());
    }
    fn log_with_ctx<'a>(&self, level: LogLevel, msg: &'a str, ctx: &'a str) {
        if self.buffer.is_attached() {
            return self.buffer.write_log(level, msg, ctx);
        };

        let logger_instance = self.buffer.get_instance();

        logger_instance.log_with_ctx(level, msg, ctx);
    }
    fn set_context<'a>(&self, ctx: &'a str) {
        *self.context.as_mut() = ctx.to_owned();
    }
    fn get_context(&self) -> String {
        self.context.as_ref().clone()
    }
}
