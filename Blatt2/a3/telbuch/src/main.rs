extern crate microkv;

use std::io;
use microkv::Store;

fn main() {
    let mut kv = Store::new("telbuch.db").unwrap();
    loop {
        let mut input = String::new();
        print!("telbuch> ");
        io::stdout().flush().unwrap(); // Flush stdout to display the prompt before reading input
        io::stdin().read_line(&mut input).unwrap();

        let parts: Vec<&str> = input.trim().split_whitespace().collect();

        match parts.get(0) {
            Some(&"!") => {
                if parts.len() < 3 {
                    println!("Falsches Format. Verwendung: ! name number");
                } else {
                    let name = parts[1].to_string();
                    let number = parts[2].to_string();

                    match kv.get::<Vec<String>>(&name) {
                        Ok(Some(mut numbers)) => {
                            numbers.push(number);
                            kv.set(&name, numbers).unwrap();
                        }
                        Ok(None) => {
                            kv.set(&name, vec![number]).unwrap();
                        }
                        Err(_) => {
                            println!("Ein Fehler ist aufgetreten.");
                        }
                    }
                }
            },
            Some(&"?") => {
                if parts.len() < 2 {
                    println!("Falsches Format. Verwendung: ? name");
                } else {
                    let name = parts[1];

                    match kv.get::<Vec<String>>(name) {
                        Ok(Some(numbers)) => {
                            for num in numbers {
                                println!("{}", num);
                            }
                        }
                        Ok(None) => {
                            println!("Keine Telefonnummern für {} gefunden.", name);
                        }
                        Err(_) => {
                            println!("Ein Fehler ist aufgetreten.");
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
