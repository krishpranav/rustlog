#![deny(missing_docs)]

extern crate chrono;
extern crate env_logger;
extern crate libc;
extern crate log;

use chrono::Local;
use env_logger::fmt::{Color, Style, StyledValue};
use env_logger::Builder;
use log::Level::{Debug, Error, Info, Trace, Warn};
use log::{Level, LevelFilter};
use std::io::Write;
use std::{env, fmt};

pub fn init<T: AsRef<str>>(level: Option<T>) {
    inner(level, false)
}

pub fn init_from_env<T: AsRef<str>>(envar: T) {
    init(env::var(envar.as_ref()).ok())
}

pub fn minimal<T: AsRef<str>>(level: Option<T>) {
    inner(level, true)
}

pub fn minimal_from_env<T: AsRef<str>>(envar: T) {
    minimal(env::var(envar.as_ref()).ok())
}

fn inner<T: AsRef<str>>(level: Option<T>, minimal: bool) {
    let mut builder = Builder::new();

    builder.filter(None, LevelFilter::Info);

    if minimal {
        builder.format(|buf, record| {
            let mut level_style = buf.style();
            let level = format_level(&mut level_style, record.level());

            writeln!(buf, "{} {}", level, record.args())
        });
    } else {
        builder.format(|buf, record| {
            let mut level_style = buf.style();
            let level = format_level(&mut level_style, record.level());

            let mut detail_style = buf.style();
            detail_style.set_color(Color::Black);
            detail_style.set_intense(true);

            writeln!(
                buf,
                "{} {} {} {}",
                detail_style.value(Local::now().format("%H:%M:%S")),
                level,
                detail_style.value(Brackets(record.target())),
                record.args()
            )
        });
    }

    if let Some(level) = level {
        builder.parse(level.as_ref());
    }

    builder.init();
}

fn format_level(style: &mut Style, level: Level) -> StyledValue<&'static str> {
    let (color, string) = match level {
        Error => (Color::Red, "[ERROR]"),
        Warn => (Color::Yellow, "[WARN] "),
        Info => (Color::Cyan, "[INFO] "),
        Debug => (Color::Green, "[DEBUG]"),
        Trace => (Color::Magenta, "[TRACE]"),
    };
    style.set_color(color);
    style.value(string)
}

struct Brackets<T>(T);
impl<T: fmt::Display> fmt::Display for Brackets<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0)
    }
}
