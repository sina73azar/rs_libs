use std::io::{self, Write};

#[derive(Debug, Clone)]
enum Gender {
    Male,
    Female,
    Other,
}

struct User {
    name: String,
    age: u32,
    gender: Gender,
}

fn read_input(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}

fn parse_gender(input: &str) -> Option<Gender> {
    match input.to_lowercase().as_str() {
        "male" => Some(Gender::Male),
        "female" => Some(Gender::Female),
        "other" => Some(Gender::Other),
        _ => None,
    }
}

fn add_user(users: &mut Vec<User>) {
    let name = read_input("Name: ");
    let age = read_input("Age: ").parse().unwrap_or(0);
    let gender_input = read_input("Gender (male/female/other): ");
    let gender = match parse_gender(&gender_input) {
        Some(gender) => gender,
        None => {
            println!("Invalid gender");
            return;
        }
    };
    users.push(User{name,age,gender});
    println!("✅ User added");
}

fn list_users(users: &[User]) {
    if users.is_empty() {
        println!("No users found");
        return;
    }

    for (i, user) in users.iter().enumerate() {
        println!(
            "{}: {} | Age: {} | Gender: {:?}",
            i, user.name, user.age, user.gender
        );
    }
}

fn remove_user(users: &mut Vec<User>) {
    let index: usize = read_input("Index to remove: ").parse().unwrap_or(usize::MAX);

    if index < users.len() {
        users.remove(index);
        println!("✅ User removed");
    } else {
        println!("❌ Invalid index");
    }
}

fn update_user(users: &mut Vec<User>) {
    let index: usize = read_input("Index to update: ").parse().unwrap_or(usize::MAX);

    if index >= users.len() {
        println!("❌ Invalid index");
        return;
    }

    let name = read_input("New name: ");
    let age: u32 = read_input("New age: ").parse().unwrap_or(users[index].age);

    let gender_input = read_input("New gender (male/female/other): ");
    let gender = parse_gender(&gender_input).unwrap_or(users[index].gender.clone());

    users[index] = User { name, age, gender };
    println!("✅ User updated");
}

fn main() {
    println!("Welcome to CRUD Fake 🚀");

    let mut users: Vec<User> = Vec::new();

    loop {
        println!("\nCommands: add | list | update | remove | exit");
        let command = read_input("> ");

        match command.as_str() {
            "add" => add_user(&mut users),
            "list" => list_users(&users),
            "update" => update_user(&mut users),
            "remove" => remove_user(&mut users),
            "exit" => {
                println!("Goodbye 👋");
                break;
            }
            _ => println!("Unknown command"),
        }
    }
}
