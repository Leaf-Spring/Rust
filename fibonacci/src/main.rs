use std::io;

fn main() {
    loop {
        println!("Get the Fibonnaci number: ");

        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Failed to read line.");

        let n: u32 = match n.trim().parse() {
            Ok(m) => m,
            Err(_) => break,
        };

        println!("[{n}]: {}", fibonacci(n));
    }
}

fn fibonacci(n: u32) -> u32 {
    if 0 == n {
        return 0;
    } else if 1 == n {
        return 1;
    }

    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for _ in 2..=n {
        let c = a;
        a += b;
        b = c;
    }

    a
}
