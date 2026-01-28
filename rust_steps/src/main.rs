fn main() {
    /*    println!("<----------------------part 1---------------------->");
        let result = add(3, 4);
        println!("3 + 4 = {result}");
        println!("3 - 4 = {}", minus(3, 4));
        let msg = String::from("hello");
        print_length(&msg);
        println!("msg = {}", msg);
        print_length("world wonder");
        let mut msg1 = String::from("hello");
        print_length(&msg1);
        append_exclamation(&mut msg1);
        println!("msg = {}", msg1);

        println!("<----------------------part 2---------------------->");
        let x = 5;
        //println!("The value of x is: {x}");
        let mut y = x.to_string();
        append_exclamation(&mut y);
        println!("y = {}", y);
    */
    /*    println!("<----------------------part 3---------------------->");
        let _target: i32 = loop {
            let input = read_line("enter countering number");

            match input.parse::<i32>() {
                Ok(n) => break n,
                Err(_) => {
                    println!("Invalid number, try again.");
                }
            }
        };
    */
    /*    println!("<----------------------part 5---------------------->");
    for i in 1..=10 {
        if i == 3 {
            continue; // skip 3
        }
        if i == 5 {
            break; // stop before printing 5
        }
        println!("i is: {}", i);
    }*/


    let first_array: [i16; 3] = [1, 2, 3];
    println!("{:?}", first_array);

   /* let mut scnd_arr: [i32; 5];
    for i in 1..=5 {
        let input = read_line("enter ");
        scnd_arr.fill(input.trim().parse::<i32>().unwrap());
    }*/

    let mut v = vec![1, 2, 3, 4];
    println!("{:?}", v);

    v.push(5);
    println!("{:?}", v);

    let len = v.iter();
    println!("{:?}", len);


    let mut v = vec![1, 2];
    let r =  v; // exclusive access
    // r.push(7);

    // println!("{:?}",r);
    println!("{:?}",r);
    unsafe {
        let mut a = vec![45];
        let ptr = a.clone();
        std::mem::forget(a);
        // println!("{:?}",*ptr);
    }

    let p: *const i32 = &10;
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
use std::mem::forget;

fn read_line(s: &str) -> String {
    let mut test_input = String::new();
    let res = io::stdin().read_line(&mut test_input);
    println!("{:?}", res);

    println!("{}", s);
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("failed to read line");
    input.trim().to_string()
}
