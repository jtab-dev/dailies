// --- FILE MANAGER ---
//
// Meant to manage the user's files and store information locally
// > The user's list of goals
// > The Gacha Pool
// > The user's unlocked/locked characters
// > The Idle-game's score

#![allow(unused)] // ************************************* REMOVE ONCE READY TO OPTOMIZE

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
        // If goals.json exists start editing goals
        edit_goals();
    } else {
        // If goals.json doesn't exist, allow the user to make goals
        make_goals();
    }
}

fn make_goals() -> io::Result<()> {
    // Create a .json storing the user's tasks
    let dir = File::create("goals.json")?;

    // Prompt user to write at least 5 goals
    println!("Write at least 5 goals");
    let mut list: Vec<Task> = Vec::<Task>::new();

    for i in 0..5 {
        // Get user input
        let mut input = String::new();
        io::stdin().read_line(&mut input);

        // Append user input to list as Task
        list.push(Task {
            desc: input,
            is_done: false,
        });
    }

    // Prompt user: Ask if done creating list

    serde_json::to_writer_pretty(dir, &list);
    Ok(())
}

pub fn edit_goals() {}
