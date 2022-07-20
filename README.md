## RustLog 

- A cool log library built using rust-lang

## Installation:
- Cargo.toml
```toml
rustlog = { git = "https://github.com/krishpranav/rustlog" }
log = "0.4.17"
```

## Usage:
- trace log:
```rust
#[macro_use]
extern crate log;
extern crate rustlog;

fn main() {
    rustlog::init_from_env("LOG_LEVEL");
    trace!("Trace details");
}
```

- debug log:
```rust
#[macro_use]
extern crate log;
extern crate rustlog;

fn main() {
    rustlog::init_from_env("LOG_LEVEL");
    debug!("Debug details.");
}
```

- info log:
```rust
#[macro_use]
extern crate log;
extern crate rustlog;

fn main() {
    rustlog::init_from_env("LOG_LEVEL");
    info!("information.");
}
```

- warn log:
```rust
#[macro_use]
extern crate log;
extern crate rustlog;

fn main() {
    rustlog::init_from_env("LOG_LEVEL");
    warn!("warning.");
}
```

- error log:
```rust
#[macro_use]
extern crate log;
extern crate rustlog;

fn main() {
    rustlog::init_from_env("LOG_LEVEL");
    error!("errorr!!!!!.");
}
```

## License:
- rustlog is licensed under [GPL-2.0 License]()