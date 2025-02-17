use std::env;
use std::fs::{OpenOptions, File};
use std::io::{Write, BufRead, BufReader};  

const FILE_NAME: &str = "tasks.txt"; // File to store tasks

fn main() {
    let args: Vec<String> = env::args().collect(); // Collecting command-line arguments

    if args.len() < 2 { // If there are not enough arguments
        eprintln!("Usage: cli_todo <command> [arguments]");
        return; 
    }

    match args[1].as_str() { // Matching the command (second argument)
        "add" => {
            if args.len() < 3 { // If task description is missing
                eprintln!("Error: Missing task description.\nUsage: cli_todo add \"task description\"");
                return; 
            }
            let task = args[2..].join(" "); // Joining all task description parts
            add_task(&task); // Call function to add task
        }

        "list" => {
            list_tasks(); // Call function to list all tasks
        }

        "remove" => {
            if args.len() < 3 { // If task ID is missing
                eprintln!("Error: Missing task number.\nUsage: cli_todo remove <task_id>");
                return; // Exit early
            }

            let task_id = args[2].parse::<usize>(); // Try to parse task ID as usize
            match task_id {
                Ok(id) => remove_task(id), // Call function to remove task if ID is valid
                Err(_) => eprintln!("Error: Invalid task number.\nUsage: cli_todo remove <task_id>"),
            }
        }

        _ => { // Catch all for unknown commands
            eprintln!("Error: Unknown command '{}'.\nUsage: cli_todo <command> [arguments]", args[1]);
        }
    }
}

// ✅ Function to add a task to the file
fn add_task(task: &str) {
    let mut file = OpenOptions::new()
        .append(true) 
        .create(true) 
        .open(FILE_NAME) 
        .expect("Failed to open file");

    writeln!(file, "{}", task).expect("Failed to write to file"); 
    println!("Task added: {}", task); 
}

// ✅ Function to list tasks from the file
fn list_tasks() {
    let file = File::open(FILE_NAME);

    match file {
        Ok(file) => {
            let reader = BufReader::new(file); 
            let mut count = 1; 

            for line in reader.lines() { 
                match line {
                    Ok(task) => {
                        println!("{}: {}", count, task); 
                    }
                    Err(_) => {
                        eprintln!("Failed to read line");
                    }
                }
                count += 1; 
            }

            if count == 1 { // If no tasks found (count stayed at 1)
                println!("No tasks found");
            }
        }
        Err(_) => {
            eprintln!("No tasks found."); 
        }
    }
}

// ✅ Function to remove a task by ID
fn remove_task(task_id: usize) {
    let file = File::open(FILE_NAME); 

    match file {
        Ok(file) => {
            let reader = BufReader::new(file);
            let tasks: Vec<String> = reader.lines().filter_map(Result::ok).collect(); // Collect tasks into a vector

            if task_id == 0 || task_id > tasks.len() { // If task ID is out of range
                eprintln!("Error: Task ID out of range.");
                return; 
            }

            let new_tasks: Vec<String> = tasks.into_iter()
                .enumerate()
                .filter(|(index, _)| *index + 1 != task_id) // Exclude the task at task_id
                .map(|(_, task)| task) // Keep the remaining tasks
                .collect();

            let mut file = File::create(FILE_NAME).expect("Failed to open file"); // Re-open file to overwrite with remaining tasks
            for task in new_tasks { // Write the updated list of tasks to the file
                writeln!(file, "{}", task).expect("Failed to write task");
            }

            println!("Task {} removed.", task_id); 
        }
        Err(_) => eprintln!("No tasks found."), 
    }
}
