#![allow(unused)] // REMOVE ONCE READY TO OPTOMIZE

// --- Modules ---
use serde::{Deserialize, Serialize};
use std::fs::{File, OpenOptions};
use std::io;
use std::path::Path;

// --- Structures ---
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    desc: String,
    is_done: bool,
}

// --- Functions ---
pub fn does_goals_exist() {
    // Check if goals.json file exists
    let file = Path::new("goals.json");
    if file.exists() {
        edit_goals(); // If goals.json exists, allow the user to edit_goals
    } else {
        make_goals(); // If goals.json doesn't exist, allow the user to make_goals
    }
}

fn make_goals() -> io::Result<()> {
    // Create a .json storing the user's tasks
    let dir = File::create("goals.json")?;

    // Prompt user to write at least 5 goals
    println!("Write at least 5 goals");
    let mut list: Vec<Task> = Vec::<Task>::new();

    for i in 0..5 {
        let mut input = String::new();
        io::stdin().read_line(&mut input); // Get user input

        list.push(Task {
            // Append user input as Task
            desc: input,
            is_done: false,
        });
    }

    // Prompt user: Ask if done creating list

    serde_json::to_writer_pretty(dir, &list);
    Ok(())
}

pub fn edit_goals() {}
