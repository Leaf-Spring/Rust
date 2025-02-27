fn main() {
    println!(
        "{}, {}, {}, {}, {}",
        fibonacci(1),
        fibonacci(2),
        fibonacci(3),
        fibonacci(10),
        fibonacci(11)
    );
}

fn fibonacci(n: u32) -> u32 {
    if 1 == n {
        return 0;
    }

    if 2 == n {
        return 1;
    }

    let mut a: u32 = 1;
    let mut b: u32 = 0;
    for _i in 3..=n {
        let c = a;
        a += b;
        b = c;
    }

    a
}
