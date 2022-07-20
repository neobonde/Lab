use std::io;


fn is_vowel(letter: &char) -> bool {
    match letter {
        'a' | 'e' | 'i' | 'o' | 'u' | 'y' | 'æ' | 'ø' | 'å' | 'ä'  => true,
        'A' | 'E' | 'I' | 'O' | 'U' | 'Y' | 'Æ' | 'Ø' | 'Å' | 'Ä'  => true,
        _ => false
    }
}

fn to_pig_latin(word: &String) -> String {

    let first_letter = word.chars().take(1).last().unwrap();
    let mut pig_latin_word = String::new();

    for (index, letter) in word.chars().enumerate() {
        if index > 0 || is_vowel(&first_letter) {
            pig_latin_word.push(letter)
        }
    }

    if is_vowel(&first_letter){
        pig_latin_word.push_str("hay");
    }else {
        pig_latin_word.push(first_letter);
        pig_latin_word.push_str("ay");
    }

    pig_latin_word

}

fn main() {

    let mut sentence = String::new();

    println!("Please write a sentence");
    io::stdin()
    .read_line(&mut sentence)
    .expect("Failed to read line");

    let words :Vec<String> = sentence.split_whitespace().map(str::to_string).collect();
    let mut pig_latin_words = Vec::new();


    for word in words {
        pig_latin_words.push(to_pig_latin(&word));
    }

    let pig_latin_sentence = pig_latin_words.join(" ");


    println!("{pig_latin_sentence:?}");
}
