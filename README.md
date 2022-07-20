## RustLog 

- A cool log library built using rust-lang

## Installation:
- Cargo.toml
```toml
rustlog = { git = "https://github.com/krishpranav/rustlog" }
log = "0.4"
```

## Usage:
- trace log:
```rust
#[macro_use]
extern crate log;
extern crate rustlog;

fn main() {
    trace!("Trace details");
}
```

- debug log:
```rust
#[macro_use]
extern crate log;
extern crate rustlog;

fn main() {
    debug!("Debug details.");
}
```