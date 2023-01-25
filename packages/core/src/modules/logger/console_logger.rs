use std::collections::HashMap;

use crate::implements;
use colored::{ColoredString, Colorize};
use once_cell::sync::Lazy;

use crate::cell::RefMut;

use super::{interface::ILogger, level::LogLevel};

pub(crate) const DEFAULT_LOGGER_CONTEXT: &str = "ConsoleLogger";

static LEVEL_COLOR_TABLE: Lazy<HashMap<LogLevel, ColorFn>> = Lazy::new(|| {
    let mut m: HashMap<LogLevel, ColorFn> = HashMap::new();
    m.insert(LogLevel::EMERG, |text| text.bright_red());
    m.insert(LogLevel::ALERT, |text| text.bright_magenta());
    m.insert(LogLevel::CRIT, |text| text.magenta());
    m.insert(LogLevel::ERROR, |text| text.red());
    m.insert(LogLevel::WARN, |text| text.yellow());
    m.insert(LogLevel::NOTICE, |text| text.green());
    m.insert(LogLevel::INFO, |text| text.cyan());
    m.insert(LogLevel::DEBUG, |text| text.white());
    m
});

static INSTANCE: Lazy<std::sync::Mutex<chrono::DateTime<chrono::Local>>> =
    Lazy::new(|| std::sync::Mutex::new(chrono::offset::Local::now()));

type ColorFn = fn(text: String) -> ColoredString;

fn println<'a>(app_name: &String, level: LogLevel, msg: &'a str, ctx: &'a str, color_fn: &ColorFn) {
    let now = chrono::offset::Local::now();
    let old = INSTANCE.lock().unwrap().clone();
    let duration = now.signed_duration_since(old);

    *INSTANCE.lock().unwrap() = now;

    println!(
        "{:}    {:}    {:} {:} {:} {:}",
        format!("[{:}]", app_name).green(),
        now.format("%m/%d/%Y, %H:%M:%S %p"),
        format!("{:<8}", (color_fn)(format!("[{}]", level.to_string()))),
        format!("[{:}]", ctx).yellow(),
        (color_fn)(msg.to_string()),
        format!("+{:}ms", duration.num_milliseconds()).yellow(),
    );
}

pub struct ConsoleLogger {
    app_name: String,
    context: RefMut<String>,
}

impl ConsoleLogger {
    pub fn new(app_name: String) -> ConsoleLogger {
        ConsoleLogger {
            app_name,
            context: RefMut::new(String::from(DEFAULT_LOGGER_CONTEXT)),
        }
    }
}

#[implements]
#[blackbox_di(crate)]
impl ILogger for ConsoleLogger {
    fn log<'a>(&self, level: LogLevel, msg: &'a str) {
        println(
            &self.app_name,
            level,
            msg,
            self.context.as_ref().as_str(),
            LEVEL_COLOR_TABLE.get(&level).unwrap(),
        );
    }
    fn log_with_ctx<'a>(&self, level: LogLevel, msg: &'a str, ctx: &'a str) {
        println(
            &self.app_name,
            level,
            msg,
            ctx,
            LEVEL_COLOR_TABLE.get(&level).unwrap(),
        );
    }
    fn set_context<'a>(&self, ctx: &'a str) {
        *self.context.as_mut() = ctx.to_owned();
    }
    fn get_context(&self) -> String {
        self.context.as_ref().clone()
    }
}
