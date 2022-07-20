use std::io;
use std::collections::HashMap;
use rand::Rng;

const NUMBER_RANGE: i32 = 100;

fn generate_integer_list(len :u32) -> Vec<i32>{

    let mut list = Vec::new();
    let mut rng = rand::thread_rng();

    for _ in 0..len{
        list.push(rng.gen_range(-NUMBER_RANGE..=NUMBER_RANGE));
    }

    println!("list: {list:?}");
    list
}

fn sort_list(list: &mut [i32])
{
    list.sort_unstable();
    // println!("sorted list: {list:?}");
}

fn mean(list: &Vec<i32>) -> f32 {

    list.iter().sum::<i32>() as f32 / list.len() as f32
}

fn median(list: &Vec<i32>) -> f32 {
    let len = list.len();

    if len % 2 == 0 {
        (list[len/2] + list[len/2-1]) as f32 /2.0
    } else {
        list[(len)/2] as f32
    }
}

fn mode(list: &Vec<i32>) -> Vec<&i32> {

    let mut map = HashMap::new();
    let mut mode_list = Vec::new();
    let mut highest_count = 0;

    for number in list{
        let count = map.entry(number).or_insert(0);
        *count += 1;
        if *count > highest_count {
            highest_count = *count;
        }
    }

    if highest_count == 1{
        return vec![]
    }

    for (&number, &count) in &map{
        if count == highest_count {
            mode_list.push(number);
        }
    }

    mode_list

}


fn main() {

    let mut list_len = String::new();

    println!("Please input a number:");
    io::stdin()
        .read_line(&mut list_len)
        .expect("Failed to read line");

    let list_len: u32 = list_len.trim().parse().expect("Please type a number!");

    let mut list = generate_integer_list(list_len);
    sort_list(&mut list[..]);
    let mean = mean(&list);
    let median = median(&list);
    let mode = mode(&list);

    println!("----------------------- ");

    println!("mean: {mean}");
    println!("median: {median}");
    println!("mode: {mode:?}");

}
