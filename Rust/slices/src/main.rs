fn main() {
    println!("Hello, world!");

    let sentence = String::from("Hello my name is nicolai");
    let index = first_word_old(&sentence);
    println!("First word of \"{}\" ends at index {}", sentence, index);

    let hello = first_word(&sentence);
    println!("First word of \"{}\" is \"{}\"",sentence, hello);
}


fn first_word_old(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return i;
        }
    }

    s.len()
}


fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate(){
        if item == b' ' {
            return &s[..i];
        }
    }

    &s[..]
}
