mod todo;
mod todo_list;

use std::io;
use std::io::Write;
use todo_list::TodoList;

fn main() {
    let mut todo_list = TodoList::new();
    let filename = "todo_list.json";

    // Try to load the to-do list from file
    if todo_list.load(filename).is_err() {
        println!("No existing to-do list found, starting a new one.");
    }

    loop {
        println!("1. Add a new To-Do");
        println!("2. Mark a To-Do as done");
        println!("3. List all To-Dos");
        println!("4. Delete a To-Do");
        println!("5. Save and Exit");

        print!("Enter your choice: ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");

        match choice.trim() {
            "1" => {
                let mut title = String::new();
                print!("Enter the title of the To-Do: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut title).expect("Failed to read line");
                todo_list.add(title.trim().to_string());
            }
            "2" => {
                let mut id_str = String::new();
                print!("Enter the ID of the To-Do to mark as done: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                if let Ok(id) = id_str.trim().parse::<usize>() {
                    todo_list.mark_done(id);
                } else {
                    println!("Invalid ID");
                }
            }
            "3" => {
                todo_list.list();
            }
            "4" => {
                let mut id_str = String::new();
                print!("Enter the ID of the To-Do to delete: ");
                io::stdout().flush().unwrap();
                io::stdin().read_line(&mut id_str).expect("Failed to read line");
                if let Ok(id) = id_str.trim().parse::<usize>() {
                    todo_list.delete(id);
                } else {
                    println!("Invalid ID");
                }
            }
            "5" => {
                // Save the to-do list to file and exit
                if todo_list.save(filename).is_err() {
                    println!("Failed to save the to-do list.");
                }
                break;
            }
            _ => {
                println!("Invalid choice");
            }
        }
    }
}


