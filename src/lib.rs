pub mod todo {
    use serde::{Deserialize, Serialize};
    use std::fs::{write, File};
    use std::{io::Read, path::Path};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Todo {
        pending: Vec<String>,
        doing: Vec<String>,
        done: Vec<String>,
    }

    impl Todo {
        /** new() looks for the 'todo.json' file.
         * If found it parses it using 'serde' and deserializes it as a json string, then serializes as a Todo struct and returs.
         * If no file is found it returns a Todo struct with empty files. */
        pub fn new() -> Self {
            let todo_path: String = String::from("todo.json");
            let path: &Path = Path::new(&todo_path);

            match path.exists() {
                // if the pathd does not exist ->
                false => Todo {
                    pending: Vec::new(),
                    doing: Vec::new(),
                    done: Vec::new(),
                },

                // if the path does exist ->
                true => {
                    // load the file as a json string ->
                    let mut buffer: String = String::new();
                    let mut todo_file: File = File::open(path).unwrap();
                    todo_file.read_to_string(&mut buffer).unwrap();
                    println!("\nReading from todo file...");

                    println!("Contents of todo file: \n{}", buffer);

                    // parsing the file content ->
                    let parsed_todo: Self = serde_json::from_str(&buffer).unwrap();
                    parsed_todo
                }
            }
        }

        /// Takes in the task as a string and pushes it to the pending queue of Todo struct.
        pub fn create_task(&mut self, task: String) {
            self.pending.push(task);
            println!("--> Task added successfully.")
        }

        /** Gives a view of the task present in pending, doing, and done queues to the Todo struct.*/
        pub fn view_tasks(&self) {
            println!("------ Pending tasks ------");
            for (task_no, task) in self.pending.iter().enumerate() {
                print!("`{}: {}`, ", task_no, task);
            }

            println!("\n------ Tasks currently doing ------");
            for (task_no, task) in self.doing.iter().enumerate() {
                print!("`{}: {}`, ", task_no, task);
            }

            println!("\n------ Tasks done ------");
            for (task_no, task) in self.done.iter().enumerate() {
                print!("`{}: {}`, ", task_no, task);
            }
        }

        /// Marks a task in the pending queue as doing by:
        /// 1. removing the task from the pending queue, then
        /// 2. pushing it to the doing queue.
        pub fn do_task(&mut self, task_id: usize) {
            // if task_id > self.pending.len() {
            //     println!("--- Task id out of bounds. Please enter correctly ---");
            // } else {
            //     let removed: String = self.pending.remove(task_id);
            //     self.doing.push(removed);
            // }

            // in rust's match pattern -->
            match task_id > self.pending.len() {
                true => {
                    let removed: String = self.pending.remove(task_id);
                    self.doing.push(removed);
                }
                false => println!("--- Task id out of bounds. Please enter correctly ---"),
            }
        }

        /// Marks a task in the doing queue as done by:
        /// 1. removing the task from doing queue, then
        /// 2. pushing it to the done queue
        pub fn mark_task_done(&mut self, task_id: usize) {
            // if task_id > self.doing.len() {
            //     println!("--- Task id out of bounds. Please enter correctly ---");
            // } else {
            //     let removed: String = self.doing.remove(task_id);
            //     self.done.push(removed);
            //     println!("--- Task marked as doing ---");
            // }

            // in rust's match pattern -->
            match task_id > self.pending.len() {
                true => {
                    let removed: String = self.pending.remove(task_id);
                    self.doing.push(removed);
                }
                false => println!("--- Task id out of bounds. Please enter correctly ---"),
            }
        }

        /// Removes a task fro the pending queue and/or from the doing queue.
        pub fn remove_task(&mut self, task_id: usize, from: i8) {
            match from {
                1 => {
                    if task_id < self.pending.len() {
                        println!("--- Removing from pending ---");
                        self.pending.remove(task_id);
                        println!("--- Task removed ---");
                    } else {
                        println!("--- Invalid task id ---");
                    }
                }
                2 => {
                    if task_id < self.doing.len() {
                        println!("--- Removing from doing vec ---");
                        self.pending.remove(task_id);
                        println!("--- Task removed ---");
                    } else {
                        println!("--- Invalid task id ---");
                    }
                }
                _ => {
                    println!("--- Please enter from where you want to remove task correctly ---");
                }
            }
        }

        /** Saves the contents of the todo list as a json string in the 'todo.json' file.
         * First it looks if the 'todo.json' file is present, if not it creates one, then writes the todo content to the file. */
        pub fn save(&self) {
            let todo_path: String = String::from("todo.json");
            let path: &Path = Path::new(&todo_path);

            // file does not exist, create one ->
            if !path.exists() {
                println!("todo.json does not exists, creating one...");
                File::create(path).unwrap();
            }

            // serialize the data to json string
            let serialized_todo: String = serde_json::to_string(self).unwrap();
            write(path, serialized_todo).unwrap();

            println!("Writing to file...");
            println!("Done");
        }
    }
}

pub mod helper {
    use std::io;

    /** Promts the uses with options to create, view, mark task as doing, mark task as done, removing task, and quiting the program.
     * Also takes in the choice from the standard input. */
    pub fn give_choices(choice: &mut String) {
        println!("Enter your choice:");
        println!("1. Create new task:");
        println!("2. View tasks:");
        println!("3. Mark task as doing:");
        println!("4. Mark task as done:");
        println!("5. Remove task:");
        println!("6. Quit:");

        match io::stdin().read_line(choice) {
            Ok(choice) => println!("--> Choice entered: {}", choice),
            Err(why) => {
                println!("--> Error while parsing the entered choice.");
                println!("--> Error: {}", why)
            }
        };
    }
}
