use std::io;

fn main() {
    loop {
        println!("Insert the position of the fibonacci series (x to exit):");
        let mut value = String::new();
        io::stdin().read_line(&mut value)
            .expect("Failed to read line");

        match value.trim().parse() {
            Ok(num) => println!("Fibonacci({}) = {}", value.trim(), fib(num)),
            Err(_) => break,
        };
    }
}

fn fib(n: u128) -> u128 {
    if n == 1 {
        return 1;
    } else if n == 2 {
        return 1;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}
