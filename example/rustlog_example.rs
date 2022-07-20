#[macro_use]
extern crate log;
extern crate rustlog;

fn main() {
    rustlog::init_from_env("LOG_LEVEL");

    trace!("Trace.");
    debug!("Debug.");
    info!("Hello, World!");
    warn!("Warning");
    error!("Error.");
}
