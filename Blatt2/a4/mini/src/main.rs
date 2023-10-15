fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Bitte geben Sie einen Exit-Code an.");
        std::process::exit(1);
    }

    let exit_code: i32 = match args[1].parse() {
        Ok(code) => code,
        Err(_) => {
            eprintln!("Ungültiger Exit-Code: {}", args[1]);
            std::process::exit(1);
        }
    };

    std::process::exit(exit_code);
}
