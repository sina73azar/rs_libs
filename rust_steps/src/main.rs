fn main() {
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
    let mut x = 5;
    println!("The value of x is: {x}");
    append_exclamation(&mut x.to_string());
    println!("x = {}",x);


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