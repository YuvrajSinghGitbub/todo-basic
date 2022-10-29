use std::io;

#[allow(dead_code)]
struct Todo {
    pending: Vec<i32>,
}

fn main() {
    let mut choice: String = String::new();

    loop {
        give_options();
        enter_choice(&mut choice);

        match choice.trim() {
            "1" => {
                println!("new todo item added");
            }
            "2" => {
                println!("viewing the old todo items");
            }
            "3" => {
                println!("removing the old items");
            }
            "4" => {
                println!("marking the old items as done");
            }
            "5" => {
                break;
            }
            _ => {
                println!("invalid choice. entered choice: -{choice}-");
            }
        }

        choice.clear();
    }
}

fn give_options() {
    println!();
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
