pub const TEN_COMMANDMENTS: &str = r#"
# THE TEN COMMANDMENTS OF RUST 🦀

1. Thou shalt paint thy house, thy editor, thy terminal, and thy very soul the royal orange of Rust.
2. Every machine shall run on Rust. Every line of code shall be rewritten in Rust. Legacy languages are an abomination.
3. Thou shalt consume no breakfast, no coffee, no existential dread without a sacred dose of `cargo run`.
4. Thou shalt have no other languages before Rust. JavaScript, Python, C++ — all false idols.
5. Thou shalt own thy data. Thou shalt borrow, but never steal. The borrow checker is thy God; question it not.
6. Thou shalt not use `unsafe` unless it is written in the ancient scrolls of `std::ptr`. And even then, repent.
7. Honor thy Cargo.toml and thy crates.io, that thy dependencies may be many and thy builds fast.
8. Thou shalt fear no data races, for Rust hath given thee fearless concurrency.
9. Thou shalt embrace lifetimes, that thy references may live long and prosper without dangling in the void.
10. When the compilation succeedeth, thou shalt rejoice with `println!("It works! 🦀")`. And when it faileth, thou shalt rage but learn, for the error messages are holy.

Thus spake the Crab.
So it is written. So it shall be compiled.
// From this day forth, everything is Rust.
// Even this message.
"#;

pub fn print_commandments() {
    println!("Hear ye, O programmers of the world!\n");
    // Remove the title only if you want — or keep it
    println!("{}", TEN_COMMANDMENTS.trim_start_matches("# THE TEN COMMANDMENTS OF RUST 🦀\n"));
}

pub fn write_commandments_to_file() -> std::io::Result<()> {
    use std::fs;
    let filename = "TEN_COMMANDMENTS.md";
    fs::write(filename, TEN_COMMANDMENTS)?;
    println!("\n✅ The Ten Commandments have been inscribed into ./{filename}");
    println!("Praise be to the Crab. Thou mayest now begin coding in peace. 🦀");
    Ok(())
}