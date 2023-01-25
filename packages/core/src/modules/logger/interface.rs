use crate::interface;

use super::level::LogLevel;

#[interface]
#[blackbox_di(crate)]
pub trait ILogger {
    fn set_context<'a>(&self, ctx: &'a str);
    fn get_context(&self) -> String;
    fn log<'a>(&self, level: LogLevel, msg: &'a str);
    fn log_with_ctx<'a>(&self, level: LogLevel, msg: &'a str, ctx: &'a str);
    fn emerg<'a>(&self, msg: &'a str) {
        self.log(LogLevel::EMERG, msg);
    }
    fn alert<'a>(&self, msg: &'a str) {
        self.log(LogLevel::ALERT, msg);
    }
    fn crit<'a>(&self, msg: &'a str) {
        self.log(LogLevel::CRIT, msg);
    }
    fn error<'a>(&self, msg: &'a str) {
        self.log(LogLevel::ERROR, msg);
    }
    fn warn<'a>(&self, msg: &'a str) {
        self.log(LogLevel::WARN, msg);
    }
    fn notice<'a>(&self, msg: &'a str) {
        self.log(LogLevel::NOTICE, msg);
    }
    fn info<'a>(&self, msg: &'a str) {
        self.log(LogLevel::INFO, msg);
    }
    fn debug<'a>(&self, msg: &'a str) {
        self.log(LogLevel::DEBUG, msg);
    }
    fn emerg_with_ctx<'a>(&self, msg: &'a str, ctx: &'a str) {
        self.log_with_ctx(LogLevel::EMERG, msg, ctx);
    }
    fn alert_with_ctx<'a>(&self, msg: &'a str, ctx: &'a str) {
        self.log_with_ctx(LogLevel::ALERT, msg, ctx);
    }
    fn crit_with_ctx<'a>(&self, msg: &'a str, ctx: &'a str) {
        self.log_with_ctx(LogLevel::CRIT, msg, ctx);
    }
    fn error_with_ctx<'a>(&self, msg: &'a str, ctx: &'a str) {
        self.log_with_ctx(LogLevel::ERROR, msg, ctx);
    }
    fn warn_with_ctx<'a>(&self, msg: &'a str, ctx: &'a str) {
        self.log_with_ctx(LogLevel::WARN, msg, ctx);
    }
    fn notice_with_ctx<'a>(&self, msg: &'a str, ctx: &'a str) {
        self.log_with_ctx(LogLevel::NOTICE, msg, ctx);
    }
    fn info_with_ctx<'a>(&self, msg: &'a str, ctx: &'a str) {
        self.log_with_ctx(LogLevel::INFO, msg, ctx);
    }
    fn debug_with_ctx<'a>(&self, msg: &'a str, ctx: &'a str) {
        self.log_with_ctx(LogLevel::DEBUG, msg, ctx);
    }
}
