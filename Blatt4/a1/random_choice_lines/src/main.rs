use std::fs;
use rand::Rng;

fn choose_line() -> Option<String> {
    let content = fs::read_to_string("random_choice_lines.txt").expect("Error reading file"); // or .ok()?
    let lines: Vec<&str> = content.lines().collect();

    let mut rng = rand::thread_rng(); // https://docs.rs/rand/latest/rand/trait.Rng.html
    let random_index = rng.gen_range(0..lines.len());
    let chosen_line = lines[random_index].to_string();

    Some(chosen_line)
}

fn main(){
    if let Some(line) = choose_line(){
        println!("{}", line);
    }
}