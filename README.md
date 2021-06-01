# Rust-Learning

 Repository for manage process of learning rust-lang

 ---

## CARGO CLI

- `cargo new {name-of-project}`
- `cargo build`
- `cargo check`
- `cargo run`
- `cargo build --release`
- `cargo update`
- `cargo doc --open`
- `cargo new --lib {name-of-lib}`

## TODO

- [ ] Add description of cargo commands

## Cargo.toml

```[.toml]
[profile.release]
panic = 'abort' 
```

Adding to .toml to abort in case of panic(do not clean the stack)