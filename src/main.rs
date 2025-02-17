use std::env ;

fn main() {
    let args: Vec<String> = env::args().collect() ; // env::args() - gets command-line arguments
    
    if args.len() < 2 {
        eprintln!("Usage: cli_todo <command> [arguments]")  ;
        return ;
    }

    match args[1].as_str() {
        "add" => {
            if args.len() < 3 {
                eprintln!("Error: Missing task description. \nUsage: cli_todo add \"task description \"") ;
                return ;
            }
            let task = args[2..].join(" ") ;
            println!("Adding task: {}", task) ;
            // TODO: Save task to file
        }
        "list" => {
            println!("Listing all tasks...")  ;
            // TODO: Load and display tasks
        }
    
        "remove" => {
            if args.len() < 3 {
                eprintln!("Error: Missing task number. \nUsage: cli_todo remove <task_id>") ;
                return ;
            }
    
            let task_id = args[2].parse::<usize>() ;
            match task_id {
                Ok(id) => println!("Removing task: {}", id)  ,
                Err(_) => eprintln!("Error: Invalid task number. \nUsage: cli_todo remove <task_id>") ,
            }
    
            // TODO: Implement task removal
        }
    
        _ => {
            eprintln!("Error: Unknown command '{}'. \nUsage: cli_todo <command> [arguments]", args[1]) ;
        }
    }
    
}