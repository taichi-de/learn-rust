use std::io;
use std::io::Write;
use std::collections::HashMap;

fn main() {
    let mut kv: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut input = String::new();
        print!("telbuch> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.get(0) {
            Some(&"!") => {
                if parts.len() < 3 {
                    println!("Falsches Format. Verwendung: ! name number");
                } else {
                    let name = parts[1].to_string();
                    let number = parts[2].to_string();

                    match kv.get_mut(&name) {
                        Some(numbers) => {
                            numbers.push(number);
                        }
                        None => {
                            kv.insert(name, vec![number]);
                        }
                    }
                }
            },
            Some(&"?") => {
                if parts.len() < 2 {
                    println!("Falsches Format. Verwendung: ? name");
                } else {
                    let name = parts[1];

                    match kv.get(name) {
                        Some(numbers) => {
                            for num in numbers {
                                println!("{}", num);
                            }
                        }
                        None => {
                            println!("Keine Telefonnummern für {} gefunden.", name);
                        }
                    }
                }
            },
            Some(&".") => {
                break;
            },
            _ => {
                println!("Unbekanntes Kommando.");
            }
        }
    }
}
