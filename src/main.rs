use console::style;
use std::io;

use std::env;

#[derive(Clone)]
struct Task {
    id : u32,
    description : String,
    completed : bool,
}

struct TodoList {
    task: Vec<Task>,
}

impl TodoList {
    fn new() -> TodoList {
        TodoList { task: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let end_task = self.task.pop();
        let id : u32;
        if let None = end_task {
            id = 1;
        } else {
            id = end_task.clone().unwrap().id + 1;
            self.task.push(end_task.expect("REASON"));
        }
        let task = Task {
            id,
            description,
            completed: false,
        };
        self.task.push(task);
    }

    fn complete_task(&mut self, task_id: u32) {
        let mut found_task : bool = false;
        for task in &mut self.task {
            if task.id == task_id {
                task.completed = true;
                found_task = true;
            }
        }
        if !found_task {
            println!("Task not found\n");
        }
    }

    fn delete_task(&mut self, task_id: u32) {
        let mut found_task : bool = false;
        if let Some(index) = self.task.iter().position(|task| task.id == task_id) {
            self.task.remove(index);
            found_task = true;
        }
        if !found_task {
            println!("Task not found\n");
        }
    }

    fn print_list(&mut self) {
        println!("Your List:");
        for task in &mut self.task {
            if task.completed == false {
                println!(
                    "{}    {}",
                    style(task.id).white(),
                    style(&task.description).bright()
                );
            } else {
                println!(
                    "{}    {}",
                    style(task.id).green(),
                    style(&task.description).green()
                );
            }
        }
    }

    fn do_something(&mut self, command: Vec<String>) {
        let _add = String::from("add");
        let mut mark_completed: bool = false;
        let mut add_to_list: bool = false;
        let mut delete_from_list: bool = false;

        for argument in command {
            if mark_completed {
                self.complete_task(argument.parse::<u32>().unwrap());
            } else if add_to_list {
                self.add_task(argument.clone());
            } else if delete_from_list {
                self.delete_task(argument.parse::<u32>().unwrap());
            }

            if "add" == argument {
                mark_completed = false;
                add_to_list = true;
                delete_from_list = false;
            } else if "completed" == argument {
                mark_completed = true;
                add_to_list = false;
                delete_from_list = false;
            } else if "delete" == argument {
                mark_completed = false;
                add_to_list = false;
                delete_from_list = true;
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
        println!("\nPlease input one of the following comands:
 - 'add' followed by the items description(s)
 - 'completed' followed by the list number(s)
 - 'delete' followed by the list number(s)
 - 'exit' to exit");
        let result = get_input();
        println!("");
        if result == ["exit"] {
            println!("Goodbye :)");
            break;
        } else if result.get(0) == Some(&"add".to_string())
            || result.get(0) == Some(&"completed".to_string())
            || result.get(0) == Some(&"delete".to_string())
        {
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
