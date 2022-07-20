fn main() {

    let s1 = String::from("hello");

    takes_ownership(s1);

    let s2 = gives_ownership();

    println!("{}",s2);

    let s3 = takes_and_gives_back(s2);

    println!("{}", s3);


    let i = 32;
    println!("{}",i);

    makes_copy(i);
    println!("{}",i);


}


fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn gives_ownership() -> String {
    let some_string = String::from("test string");

    some_string
}

fn takes_and_gives_back(some_string: String) -> String {
    println!("{}", some_string);
    some_string
}


fn makes_copy(some_int: i32) {
    println!("{}", some_int)
}
