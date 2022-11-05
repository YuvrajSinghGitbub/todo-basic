use std::{io, num::ParseIntError};

#[allow(dead_code)]
/// Struct defining the pending tasks as a vector.
pub struct Todo {
    /// pending vec holds all task that are newly added.
    pending: Vec<String>,

    /// doing vec holds task that marked as doin
    doing: Vec<String>,

    // done vec holds last 5 task that were done/removed from doing and/or pending
    done: Vec<String>,
}

#[allow(dead_code)]
#[allow(unused_variables)]
impl Todo {
    /// Takes in a new task as a string and pushes it to the pending vector.
    pub fn add_new(&mut self, task: String) {
        self.pending.push(task);
        println!("new item added");
    }

    /// Iterates over the pending vec and displays the old pending tasks.
    pub fn view_tasks(&self) {
        println!("Pending old tasks:");
        println!("[");
        for (task_num, task) in self.pending.iter().enumerate() {
            println!("  {}: {}", task_num, task);
        }
        println!("]");

        println!("Tasks currenty doing:");
        println!("[");
        for (task_num, task) in self.doing.iter().enumerate() {
            println!("  {}: {}", task_num, task);
        }
        println!("]");

        println!("Tasks done:");
        println!("[");
        for (task_num, task) in self.done.iter().enumerate() {
            println!("  {}: {}", task_num, task);
        }
        println!("]");
    }

    /// Removes any task based on its id from any task vecs
    /// from = 1: removes from the pending vec.
    /// from = 2: removes from the doing vec.
    pub fn remove_task(&mut self, from: String, task_id: String) -> Result<(), ParseIntError> {
        let from = from.parse::<i32>()?;
        let task_id = task_id.parse::<usize>()?;

        match from {
            1 => {
                println!("Removing from pending");
                if task_id < self.pending.len() {
                    self.pending.remove(task_id);
                    println!("Task removed.");
                } else {
                    println!("Invalid task id");
                }
            }
            2 => {
                println!("Removing from doing vec");
                if task_id < self.doing.len() {
                    self.pending.remove(task_id);
                    println!("Task removed.");
                } else {
                    println!("Invalid task id");
                }
            }
            _ => {
                println!("Please enter from where you want to remove task correctly");
            }
        }
        Ok(())
    }

    /// Adds a pending task from pending vec to doing vec based on task_id
    pub fn mark_doing(&mut self, task_id: String) -> Result<(), ParseIntError> {
        let task_id = task_id.parse::<usize>()?;
        match task_id > self.pending.len() {
            true => {
                let removed = self.pending.remove(task_id);
                self.doing.push(removed);
            }
            false => println!("Invalid task id"),
        }
        Ok(())
    }

    /// Removes the task from the doing vector and adds it to the done vec.
    pub fn mark_done(&mut self, task_id: String) -> Result<(), ParseIntError> {
        let task_id = task_id.parse::<usize>()?;

        match task_id > self.doing.len() {
            true => {
                let removed = self.pending.remove(task_id);
                self.done.push(removed);
            }
            false => println!("Invalid task id"),
        }
        Ok(())
    }
}

fn main() {
    let mut choice: String = String::new();
    let mut todo1: Todo = Todo {
        pending: Vec::new(),
        doing: Vec::new(),
        done: Vec::new(),
    };

    loop {
        give_options();
        enter_choice(&mut choice);

        match choice.trim() {
            "1" => {
                let mut task: String = String::new();

                // get the task ->
                println!("--> Enter your task:");
                match io::stdin().read_line(&mut task) {
                    Err(why) => println!("Error occured while reading task number. Error: {why}"),
                    Ok(_) => {
                        todo1.add_new(task.trim().to_owned());
                    }
                };
            }
            "2" => {
                todo1.view_tasks();
            }
            "3" => {
                let mut from: String = String::new();
                let mut task_id: String = String::new();

                println!("--> Enter from where you want to remove task:");
                println!("--> 1: from pending:\n--> 2: from doing");
                io::stdin().read_line(&mut from).unwrap();

                println!("--> Tasks:");
                todo1.view_tasks();
                println!("--> Enter task id:");
                io::stdin().read_line(&mut task_id).unwrap();

                match todo1.remove_task(from.trim().to_owned(), task_id.trim().to_owned()) {
                    Ok(()) => println!("Task removes successfully"),
                    Err(why) => println!("Error occured while parsing: {}", why),
                };
            }
            "4" => {
                let mut task_id: String = String::new();

                println!("--> Enter the pending task id you want to mark as doing:");
                io::stdin().read_line(&mut task_id).unwrap();
                match todo1.mark_doing(task_id.trim().to_owned()) {
                    Ok(()) => println!("Task moved to doing successfully"),
                    Err(why) => println!("Error occured while parsing: {}", why),
                }
            }
            "5" => {
                let mut task_id: String = String::new();

                println!("--> Enter the doing task id you want to mark as done:");
                io::stdin().read_line(&mut task_id).unwrap();
                match todo1.mark_doing(task_id.trim().to_owned()) {
                    Ok(()) => println!("Task moved to done successfully"),
                    Err(why) => println!("Error occured while parsing: {}", why),
                }
            }
            "6" => {
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
    println!("3. Remove old todo item:");
    println!("4. Mark old pending item as doing:");
    println!("5. Mark todo item as done:");
    println!("6. Exit\n->");
    print!("->");
}

fn enter_choice(choice: &mut String) {
    match io::stdin().read_line(choice) {
        Err(why) => panic!("couldn't register the choice. {why}"),
        Ok(_) => println!("choice is: {choice}"),
    };
}
