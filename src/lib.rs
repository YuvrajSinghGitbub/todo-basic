
pub mod todo {
    pub struct Todo {
        pending: Vec<String>,
        doing: Vec<String>,
        done: Vec<String>,
    }

    impl Todo {
        // TODO: new associated function  
        // pub fn new() -> Self {
            /* check for a todo json file in the current folder
             * if found load all the data to create the struct
             * if not just return a struct with empty queues*/
        // }
        
        pub fn create_task(&mut self, task: String) {
           self.pending.push(task); 
           println!("--> Task added successfully.")
        }

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

        pub fn do_task(&mut self, task_id: usize){
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
        pub fn save(&self) {
            /* TODO: save the contents of the tasks to a json file
             * check to see if the todo file exists in the current folder
             * if exists then write all the contents of the queues as json
             * if not then make the folder then do the above step*/
        }
    }
}

pub mod helper {
    use std::io;

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
           Err(why)   => {
               println!("--> Error while parsing the entered choice.");
               println!("--> Error: {}", why)
           },
        };
    }

    // TODO: Implement ability to read from a json file
    // TODO: Implement ability to write to a json file
}
