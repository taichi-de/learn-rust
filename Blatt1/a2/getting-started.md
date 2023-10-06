<!-- und fassen Sie die wesentlichen Ausagen in einer Datei Blatt1/a2/getting-started.md zusammen. -->
# Zusammenfassung

- Rustup: der Rust installer und version managementmittel
    - um zu prüfen, ob man die aktuelle Version hat, kann man mit `rustup update` durchführen

- Cargo: das Rust build Mittel und package manager
    - Cargo kann Projekte builden, run, testen, dokumentieren, publischen

- Commandos:
    - `cargo new xxx`: herstellt ein Projekt
    - `cargo run`
    - Cargo.toml: ist das Manifest

- "crates": packages für Rust

- Dependencies: kann man in Cargo.toml hinzufügen unter [dependencies] oder durch `cargo add ferris-says`

- Danach kann man bilden durch `cargo build`

- Um ein Dependency zu verwenden, kann man in einem File schreiben: `use ferris-says::say;`
