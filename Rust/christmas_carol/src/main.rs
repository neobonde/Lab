
fn main() {

    for day in 1..=12 {
        println!("On the {} day of christmas", num_to_text(day));
        println!("My true love sent to me");

        for num in (1..=day).rev() {
            if num == 1 {
                if day == 1{
                    println!("A {}",gift(num));
                } else {
                    println!("And a {}",gift(num));
                }
            }
            else {
                println!("{}",gift(num));
            }

        }
        println!();
        println!();

    }
}


fn num_to_text(num: i32) -> String{

    match num {
        1 => String::from("first"),
        2 => String::from("second"),
        3 => String::from("third"),
        4 => String::from("forth"),
        5 => String::from("fifth"),
        6 => String::from("sixth"),
        7 => String::from("seventh"),
        8 => String::from("eighth"),
        9 => String::from("nineth"),
        _ => format!("{}th",num)
    }
}


fn gift(num: i32) -> String {

    match num {
        1 => String::from("partridge in a pear tree"),
        2 => String::from("Two turtle-doves"),
        3 => String::from("Three French hens"),
        4 => String::from("Four calling birds"),
        5 => String::from("Five golden rings (five golden rings)"),
        6 => String::from("Six geese a-laying"),
        7 => String::from("Seven swans a-swimming"),
        8 => String::from("Eight maids a-milking"),
        9 => String::from("Nine ladies dancing"),
        10 => String::from("Ten lords a-leaping"),
        11 => String::from("Eleven pipers piping"),
        12 => String::from("12 drummers drumming"),
        _ => String::new()
    }
}
