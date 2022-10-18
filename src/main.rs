use std::io;

#[allow(dead_code)]
struct Todo {
    pending: Vec<i32>,
}

fn main() {
    let mut choice: String = String::new();
    give_options();
    enter_choice(&mut choice);

    // TODO:
    // 1. function for each entered choice
    // 2. how to use the Todo struct
}

fn give_options() {
    println!("Enter your choice");
    println!("1. Enter a new todo item:");
    println!("2. View old todo items:");
    println!("3. Remove old todo items:");
    println!("4. Mark toto items as done:");
    println!("5. Exit\n->");
    print!("->");
}

fn enter_choice(choice: &mut String) {
    match io::stdin().read_line(choice) {
        Err(why) => panic!("couldn't register the choice. {why}"),
        Ok(_) => println!("choice is: {choice}"),
    };
}
