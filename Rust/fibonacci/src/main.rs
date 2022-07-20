use std::io;

fn main() {
    println!("Welcome to the Nth fibonacci calculater!");

    loop {
        println!("Enter N:");

        let mut n = String::new();

        io::stdin().read_line(&mut n).expect("Could not read line!");

        let n : i128 = n.trim().parse().expect("Expected a integer number!");

        println!("the {} fibonacci number is: {}",n, fibonacci(n));
    }
}

fn fibonacci(n :i128) -> i128 {
    if n <= 1 {
        return n;
    }

    fibonacci(n-1) + fibonacci(n-2)
}
