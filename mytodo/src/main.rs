use std::io;

fn main() {
    println!("\n ### Lets manage your todolist! ### \n");
    
    help_action();

    loop {
        println!("\nWhat will you do today?");

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get input");

        let input: String = input.trim().to_string();

        if input.is_empty() {
            continue;
        }

        let action: String = input.split_whitespace().next().unwrap().to_lowercase();
        let action_len = if input.len() > action.len() { action.len() + 1 } else { 0 } ;
        let item_name = (&input[action_len..input.len()]).to_string();

        if action == "add"
        {
            add_action(&item_name);
        }

        if action == "do"
        {
            do_action(&item_name);
        }

        if action == "done"
        {
            done_action(&item_name);
        }

        if action == "remove"
        {
            remove_action(&item_name);
        }

        if action == "list"
        {
            list_action();
        }

        if action == "help"
        {
            help_action();
        }

        if action == "exit"
        {
            println!("You closing application!");
            println!("Your todolist has been saved, you can open it again another time.");
            break;
        }
    }
}

fn help_action() {
    println!("<command> \n");
    println!("Usage: \n");
    println!("add <foo>      add the <foo> to the list of todolist");
    println!("do <foo>       update todolist <foo> to 'on progress'");
    println!("done <foo>     update todolist <foo> to 'finish'");
    println!("remove <foo>   delete todolist <foo> from todolist");
    println!("status         show your todolist.");
    println!("help           show command of application");
    println!("exit           the way you exit the application.");
}

fn add_action(item_name: &str) {
    println!("You choose to 'Add' action");
    println!("{}", item_name);
}

fn do_action(item_name: &str) {
    println!("You choose to 'Do' action");
    println!("{}", item_name);
}

fn done_action(item_name: &str) {
    println!("You choose to 'Done' action");
    println!("{}", item_name);
}

fn remove_action(item_name: &str) {
    println!("You choose to 'Remove' action");
    println!("{}", item_name);
}

fn list_action() {
    println!("You choose to 'Status' action");
}