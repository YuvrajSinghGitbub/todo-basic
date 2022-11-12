use std::io;
use todo_basic::{helper::give_choices, todo::Todo};

fn main() {
    let mut choice: String = String::new();
    let mut todo1: Todo = Todo::new();

    loop {
        give_choices(&mut choice);

        match choice.trim() {
            // Create a new task ->
            "1" => {
                println!("--> Enter your task:");

                let mut task: String = String::new();

                match io::stdin().read_line(&mut task) {
                    Err(why) => println!("Error occured while reading task number. Error: {why}"),
                    Ok(_) => {
                        todo1.create_task(task.trim().to_owned());
                    }
                };
            }
            "2" => {
                todo1.view_tasks();
            }
            "3" => {
                let mut task_id: String = String::new();

                println!("--> Task id to mark as doing:");
                io::stdin().read_line(&mut task_id);

                todo1.do_task(task_id);
            }
            "4" => {
                let mut task_id: String = String::new();

                println!("--> Task id to mark as done:");
                io::stdin().read_line(&mut task_id);

                todo1.mark_task_done(task_id);
            }
            "5" => {
                let mut from: String = String::new();
                let mut task_id: String = String::new();

                println!("--> Enter from where you want to remove task:");
                println!("--> 1: from pending:\n--> 2: from doing");
                io::stdin().read_line(&mut from).unwrap();

                todo1.view_tasks();

                println!("--> Enter task id:");
                io::stdin().read_line(&mut task_id).unwrap();

                todo1.remove_task(task_id, from);
            }
            "6" => {
                todo1.save();
                break;
            }
            _ => {
                println!("invalid choice. entered choice: -{choice}-");
            }
        }

        choice.clear();
    }
}
