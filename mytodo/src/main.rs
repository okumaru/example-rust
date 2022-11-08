use std::io;
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

fn main() {
    println!("\n### Lets manage your todolist! ### \n");

    let mut todolist = ToDoList::new().expect("Initialisation of db failed");
    todolist.help();

    loop {
        println!("---");
        println!("\nWhat will you do today?");

        let mut input = String::new();

        // Request terminal to input some words.
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input");

        // Remove new line after user do enter on terminal and set to be string.
        let input: String = input
            .trim()
            .to_string();

        // Do next if user do enter on terminal.
        if input.is_empty() { continue; }

        // Split written words into action and item name.
        let (action, item_name): (ToDoActions, String) = todolist.read_input(input);

        match action {
            ToDoActions::Add => todolist.add(&item_name),
            ToDoActions::Process => todolist.process(&item_name),
            ToDoActions::Pause => todolist.pause(&item_name),
            ToDoActions::Done => todolist.done(&item_name),
            ToDoActions::Remove => todolist.remove(&item_name),
            ToDoActions::Status => todolist.status(),
            ToDoActions::Help => todolist.help(),
            ToDoActions::Exit => {
                println!("You closing application!");
                println!("Your todolist has been saved, you can open it again another time.");
                break;
            },
            _ => println!("Does not found action!,\nplease use `help` to show list of actions command."),
        }
    }
}

#[derive(Debug)]
struct ToDoList {
    list: Vec<ToDo>,
}

impl ToDoList {
    fn new() -> Result<ToDoList, std::io::Error> {
        Ok(ToDoList { 
            list: vec![]
        })
    }

    fn help(&self) {
        for action in ToDoActions::iter() {
            match action {
                ToDoActions::Add => println!("{: <15} {}", String::from("add: <foo>"), String::from("add the <foo> to the list of todolist")),
                ToDoActions::Process => println!("{: <15} {}", String::from("process: <foo>"), String::from("update todolist <foo> to 'progress'")),
                ToDoActions::Pause => println!("{: <15} {}", String::from("pause: <foo>"), String::from("update todolist <foo> to 'paused'")),
                ToDoActions::Done => println!("{: <15} {}", String::from("done: <foo>"), String::from("update todolist <foo> to 'finish'")),
                ToDoActions::Remove => println!("{: <15} {}", String::from("remove: <foo>"), String::from("delete todolist <foo> from todolist")),
                ToDoActions::Status => println!("{: <15} {}", String::from("status"), String::from("show your todolist")),
                ToDoActions::Help => println!("{: <15} {}", String::from("help"), String::from("show command of application")),
                ToDoActions::Exit => println!("{: <15} {}", String::from("exit"), String::from("the way you exit the application")),
                _ => println!("{: <15}", String::from(""))
            }
        }
    }

    fn read_input(&self, input: String) -> (ToDoActions, String) {
        let action_str: String = input
            .split(":")
            .next()
            .unwrap()
            .to_lowercase();

        // Get length of first word.
        let action_len = if input.len() > action_str.len() { &action_str.len() + 2 } else { 0 };

        let action = match action_str.as_str() {
            "add" => ToDoActions::Add,
            "process" => ToDoActions::Process,
            "pause" => ToDoActions::Pause,
            "done" => ToDoActions::Done,
            "remove" => ToDoActions::Remove,
            "status" => ToDoActions::Status,
            "help" => ToDoActions::Help,
            "exit" => ToDoActions::Exit,
            _ => ToDoActions::Unknown,
        };

        // Get words next first word.
        let item_name = if input.len() > action_str.len() + 1 { 
                (&input[action_len..input.len()]).to_string()
            } else { 
                "".to_string()
            };

        (action, item_name)
    }

    fn add(&mut self, item_name: &str) {
        println!("Add '{}' to todolist", item_name);
    }

    fn process(&mut self, item_name: &str) {
        println!("Process '{}' from todolist", item_name);
    }

    fn pause(&mut self, item_name: &str) {
        println!("Pause '{}' from todolist", item_name);
    }

    fn done(&mut self, item_name: &str) {
        println!("Set todolist '{}' to be 'Done'", item_name);
    }

    fn remove(&mut self, item_name: &str) {
        println!("Remove todolist '{}' from todolist", item_name);
    }

    fn status(&self) {
        println!("You choose to 'Status' action");
    }
}

#[derive(Debug, EnumIter)]
enum ToDoActions {
    Add,
    Process,
    Pause,
    Done,
    Remove,
    Status,
    Help,
    Exit,
    Unknown
}

#[derive(Debug)]
struct ToDoAction {
    key: ToDoActions,
    example: String,
    desc: String,
}

#[derive(Debug)]
struct ToDo {
    name: String,
    status: String,
}

impl ToDo {
    fn process(&mut self) {
        self.status = "progress".to_string()
    }

    fn pause(&mut self) {
        self.status = "paused".to_string()
    }

    fn done(&mut self) {
        self.status = "finish".to_string()
    }
}