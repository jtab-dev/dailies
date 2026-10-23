// --- FILE MANAGER ---
//
// Meant to manage the user's files and store information locally
// > The user's list of goals
// > The Gacha Pool
// > The user's unlocked/locked characters
// > The Idle-game's score

// ***************************** REMOVE ONCE READY TO OPTOMIZE ***************
#![allow(unused)]
// ***************************** REMOVE ONCE READY TO OPTOMIZE ***************

// --- Modules ---
use crossterm::{
    event::{Event, KeyCode, KeyEvent},
    execute,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs::{File, OpenOptions};
use std::io::{self, BufReader};
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
        edit_goals().expect("Could not edit goals");
    } else {
        // If goals.json doesn't exist, allow the user to make goals
        make_goals().expect("Could not make goals");
    }
}

fn make_goals() -> io::Result<()> {
    // Create a .json storing the user's tasks
    let dir = File::create("goals.json")?;

    // Prompt user to write at least 5 goals
    println!("Write at least 5 goals");
    let mut list: Vec<Task> = Vec::<Task>::new();

    for i in 0..5 {
        println!("Goals left: {}", i);
        // Get user input
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to get Input");

        // Append user input to list as Task
        list.push(Task {
            desc: input,
            is_done: false,
        });
    }

    // Prompt user: Ask if done creating list

    serde_json::to_writer_pretty(dir, &list).expect("Failed to write to JSON file");
    Ok(())
}
fn edit_goals() -> Result<Vec<Task>, Box<dyn Error>> {
    let file = File::open("goals.json")?;
    let list: Vec<Task> = serde_json::from_reader(BufReader::new(file))?;

    for task in &list {
        println!("{}", task.desc);
    }

    Ok(list)
}
