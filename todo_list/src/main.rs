use std::io;

//define a struct for task to manage individual items
struct Task{
    description: String,
    completed: bool,
}

fn main() {
    // create an empty list of tasks.
    let mut tasks: Vec<Task> = Vec::new();

    loop {
        //Display Menu
        println!("\nTodo List Command Line Interface");
        println!("1. Add a new task");
        println!("2. List all taks");
        println!("3. Mark a task as complete");
        println!("4. Quit");
        println!("Choose an option: ");

        //Get user input
        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        //`match` to handle different user inputs, similar to switch-case
        match choice.trim(){
            "1" => add_task(&mut tasks),
            "2" => list_tasks(&tasks),
            "3" => mark_complete(&mut tasks),
            "4" => {
                println!("Goodbye !");
                break;
            },
            _ => println!("Invalid option, please try again."),

        }   
    
    }
}

// Functions 
//add_task 
fn add_task(tasks: &mut Vec<Task>){
    println!("Enter a task: ");
    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Failed to read line");
//create a new task with the description and mark it as not completed
    let task = Task {
        description: description.trim().to_string(),
        completed: false,
    };
    tasks.push(task);
    println!("Task added successfully!");
}

//fn to list all tasks 
fn list_tasks(tasks: &Vec<Task>){
    println!("\nTasks: ");
    if tasks.is_empty(){
        println!("No tasks available.");

    }else{
        for (i, task) in tasks.iter().enumerate(){
            println!("{}: {} [{}]", i + 1, task.description, if task.completed{"✓"} else { " " });
        }
    }
}

//fn to mark task as complete
fn mark_complete(tasks: &mut Vec<Task>){
    println!("Enter the number of the task you to mark as complete: ");
    let mut task_number = String::new();
    io::stdin().read_line(&mut task_number).expect("Failed to read line ");

    if let Ok(index) = task_number.trim().parse::<usize>(){

        if index == 0 || index > tasks.len() {
            println!("Invalid task number.");
        }else{
            tasks[index - 1].completed = true;
            println!("Task marked as complete !");
        }

    }else{
            println!("Please enter a valid number. ")
        }
    }
