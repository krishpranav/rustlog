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

pub fn minimal<T: AsRef<str>>(level: Option<T>) {
    inner(level, true)
}

pub fn minimal_from_env<T: AsRef<str>>(envar: T) {
    minimal(env::var(envar.as_ref()).ok())
}


struct Brackets<T>(T);
impl<T: fmt::Display> fmt::Display for Brackets<T> {
    fn fmt(&self, f: &mut, fmt::Formatter) -> fmt::Result {
        write!(f, "[{}]", self.0)
    }
}