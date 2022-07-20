fn main() {

    let s1 = String::from("Hello, world!");
    let (len, s2) = calculate_len_move(s1);
    println!("string = {} | is {} chars long", s2, len);




    let s3 = String::from("Hello, ref");
    let len = calculate_len_ref(&s3);
    println!("string = {} | is {} chars long", s3, len);

    let mut s4 = String::from("Hello");
    change(&mut s4);

    println!("{}", s4);


}


fn calculate_len_move(some_string: String) -> (usize, String) {
    let len = some_string.len();

    (len, some_string)
}


fn calculate_len_ref(some_string: &String) -> usize {
    some_string.len()
}


fn change(s: &mut String) {
    s.push_str(", mutable reference!");
}
