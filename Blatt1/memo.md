# Rustup

## Werkzeuge rustc, cargo, rustup

### Rustup cli

`rustup show`
`rustup check`
`rustup update`
`rustup self uninstall`

`rustc --version` gibt Version aus.

### “Hallo Welt”

// Datei "programm.rs"
fn main() {
    println!("Hallo Welt!");
}
`rustc programm.rs` erzeugt ausführbare Datei programm. Laufen lassen mit `./programm`.
`rustc --test programm.rs` kompiliert die Testfälle im Programm.

### Cargo

`cargo new` programm erzeugt ein Package.
programm
+-- Cargo.toml
+-- src
+-- main.rs
`cd programm`
`cargo --version`
`cargo build`
`cargo build --release`
`cargo run`
`cargo check`
`cargo fmt`
`cargo clean`

### Clippy(lint)

rustup component add clippy
`cargo clippy`

### others

`chmod +x word_counter.rs`
