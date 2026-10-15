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
pub fn does_goals_exist() -> bool {
    // Check if goals.json file exists
    //
    // If goals.json does exist, edit_goals
    // If does not exist, make_goals
    false
}

fn make_goals() -> io::Result<()> {
    // Create a .json storing the user's tasks
    let dir = File::create("goals.json")?;

    println!("Write at least 5 goals");
    let mut list: Vec<Task> = vec![
        Task {
            desc: String::from(""),
            is_done: false,
        },
        Task {
            desc: String::from(""),
            is_done: false,
        },
        Task {
            desc: String::from(""),
            is_done: false,
        },
        Task {
            desc: String::from(""),
            is_done: false,
        },
        Task {
            desc: String::from(""),
            is_done: false,
        },
    ];

    for task in &mut list {
        let mut input = String::new();
        io::stdin().read_line(&mut input);

        task.desc = input;
    }

    serde_json::to_writer_pretty(dir, &list);
    Ok(())
}

pub fn edit_goals() {}
