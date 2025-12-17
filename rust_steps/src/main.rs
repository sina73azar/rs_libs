fn main() {

    println!("<----------------------part 1---------------------->");
    let result = add(3, 4);
    println!("3 + 4 = {result}");
    println!("3 - 4 = {}",minus(3, 4));
    let msg = String::from("hello");
    print_length(&msg);
    println!("msg = {}",msg);
    print_length("world wonder");
    let mut msg1 = String::from("hello");
    print_length(&msg1);
    append_exclamation(&mut msg1);
    println!("msg = {}",msg1);

    println!("<----------------------part 2---------------------->");
    let x = 5;
    //println!("The value of x is: {x}");
    let mut y = x.to_string();
    append_exclamation(&mut y);
    println!("y = {}",y);

    println!("<----------------------part 3---------------------->");
    let input = read_line("enter something");
    println!("input = {}",input);
}


fn add(a: i32, b: i32) -> i32 {
    a + b // expression return, no semicolon
}

fn minus(a: i32, b: i32) -> i32 {
    a - b // expression return, no semicolon
}


fn print_length(s: &str) {
    println!("length = {}", s.len());
}


fn append_exclamation(s: &mut String) {
    s.push('!');
}

use std::io;

fn read_line(s: &str) -> String {
    println!(s);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input.trim().to_string()
}