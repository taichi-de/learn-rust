use std::io::{self, BufRead};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    for i in 0..args.len() {
        match args[i].as_str() {
            "-h" => {
                println!("usage: program [opts]");
                println!("   -h   --  print usage");
                println!("   -d n --  delay for n seconds");
                println!("   -e   --  echo stdin to stdout, until '.'");
                println!("   -r n  -- return with exit code n");
            }
            "-d" => {
                if i + 1 < args.len() {
                    let delay: u64 = args[i + 1].parse().unwrap_or(0);
                    std::thread::sleep(std::time::Duration::from_secs(delay));
                }
            }
            "-e" => {
                let stdin = io::stdin();
                for line in stdin.lock().lines() {
                    let line = line.unwrap();
                    if line == "." {
                        break;
                    }
                    println!("{}", line);
                }
            }
            "-r" => {
                if i + 1 < args.len() {
                    let code: i32 = args[i + 1].parse().unwrap_or(0);
                    std::process::exit(code);
                }
            }
            _ => {}
        }
    }
}
