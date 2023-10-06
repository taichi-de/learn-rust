use std::io;

fn main() {
    println!("Please input your num.");

    let mut n = String::new();

    io::stdin()
        .read_line(&mut n)
        .expect("Failed to read line");

    let n: u32 = n.trim().parse().expect("Failed");

    println!("Collatz-Folge für {}: ", n);
    collatz(n);
}

fn collatz(mut n: u32) {
    while n != 1 {
        println!("{}", n);
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = 3 * n + 1;
        }
    }
    println!("{}", n);
}
