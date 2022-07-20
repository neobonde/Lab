use std::io;

fn main() {
    println!("Welcome to the temperature converter!");
    println!("Enter a temperature and see it converted between °C and °F:");

    let mut temperature_c = String::new();

    io::stdin().read_line(&mut temperature_c).expect("Could not read line!");

    let temp :f64 = temperature_c.trim().parse().expect("Please enter a number!");

    println!("{} °C is {} °F",temp, c_to_f(temp));
    println!("{} °F is {} °C",temp, f_to_c(temp));
    println!("{} °C is {} °K",temp, c_to_k(temp));
    println!("{} °F is {} °K",temp, f_to_k(temp));
    println!("{} °K is {} °C",temp, k_to_c(temp));
    println!("{} °K is {} °F",temp, k_to_f(temp));


}

fn c_to_f(c: f64) -> f64 {
    c * 1.8 + 32.0
}

fn f_to_c(f: f64) -> f64 {
    (f-32.0)/1.8
}

fn c_to_k(c: f64) -> f64 {
    c - 273.15
}

fn k_to_c(k: f64) -> f64 {
    k + 273.15
}

fn f_to_k(f: f64) -> f64 {
    c_to_k(f_to_c(f))
}

fn k_to_f(k: f64) -> f64 {
    c_to_f(k_to_c(k))
}
