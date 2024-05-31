use std::io;
use console::style;

use std::env;

struct Task {
    id : u32,
    description : String,
    due_date : Option<String>,
    completed : bool,
}

struct TodoList {
    task : Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { task : Vec::new() }
    }

    fn add_task(&mut self, description : String, due_date : Option<String>) {
        let id = self.task.len() as u32 + 1;
        let task = Task {
            id,
            description,
            due_date,
            completed : false,
        };
        self.task.push(task);
    }

    fn complete_task(&mut self, task_id : u32) -> Result<(), String> {
        for task in &mut self.task {
            if task.id == task_id {
                task.completed = true;
                return Ok(());
            }
        }
        Err("Task not found".to_string())
    }

    fn delete_task(&mut self, task_id : u32) -> Result<(), String> {
        if let Some(index) = self.task.iter().position(|task| task.id == task_id) {
            self.task.remove(index);
                return Ok(());
        }
        Err("Task not found".to_string())
    }

    fn print_list(&mut self) {
        println!("\nYour List:");
        for task in &mut self.task {
            if task.completed == false {
                println!("{}    {}", style(task.id).white(), style(&task.description).bright());
            } else {
                println!("{}    {}", style(task.id).green(), style(&task.description).green());
            }
        }
    }

    fn do_something(&mut self, command : Vec<String>) {
        let _add = String::from("add");
        let mut addToList : bool = false;
        let mut deleteFromList : bool = false;
        let mut markCompleted : bool = false;
    
        for argument in command {            
            if markCompleted == true {
                self.complete_task(argument.parse::<u32>().unwrap());
            } else if deleteFromList == true {
                self.delete_task(argument.parse::<u32>().unwrap());
            } else if addToList == true {
                self.add_task(argument.clone(), None);
            }

            if "add" == argument {
                addToList = true;
                markCompleted = false;
                deleteFromList = false;
            } else if "completed" == argument {
                addToList = false;
                markCompleted = true;
                deleteFromList = false;
            } else if "delete" == argument {
                addToList = false;
                markCompleted = false;
                deleteFromList = true;
            }
        }
    
        self.print_list();
    }
}

fn main() {

    let mut todo_list = TodoList::new();
    let args: Vec<String> = env::args().skip(1).collect();

    todo_list.do_something(args);

    loop {
        println!("\nPlease input one of the following comands:\n - 'add' followed by the items\n - 'completed' followed by the list number\n - 'delete' followed by the list number\n - 'exit' to exit");
        let result = get_input();
        if result == ["exit"] {
            println!("Goodbye :)");
            break;
        } else if result.get(0) == Some(&"add".to_string()) || result.get(0) == Some(&"completed".to_string()) || result.get(0) == Some(&"delete".to_string()) {
            todo_list.do_something(result);
        } else {
            println!("That is not a valid input");
        }
    }

}

fn get_input() -> Vec<String> {
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let args: Vec<String> = input.split_whitespace().map(|s| s.to_string()).collect();
    return args;
}