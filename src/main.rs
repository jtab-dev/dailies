// ***************************** REMOVE ONCE READY TO OPTOMIZE ***************
#![allow(unused)]
// ***************************** REMOVE ONCE READY TO OPTOMIZE ***************

mod files;
mod render;
use crossterm::{
    event::{Event, KeyCode, KeyEvent},
    execute,
};

fn main() {
    // Learn how to implement crossterm so this loops infinitely and stops when the user presses q
    files::does_goals_exist();
}
