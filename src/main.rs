mod tasks;
mod errors;

//use std::env;
use clap::{Parser, Subcommand};
use crate::errors::TodoError;
use crate::tasks::TodoList;

#[derive(Parser)]
#[command(name ="TodoList")]
#[command(about = "A simple todo list manager")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug)]
#[derive(PartialEq)]
#[derive(Subcommand)]
pub enum Command {
    Add {text: String},
    List,
    Complete {id: usize},
    Remove {id: usize},
}

fn run_command(cli: Cli) -> Result<(),TodoError> {
    let mut todo_list = TodoList::load_from_file("tasks.json")
        .map_err(|e| TodoError::FileError(e.to_string()))?;  //TODO AMC map_error ??
    match cli.command {
        Command::Add {text} => {
            todo_list.add_task(text)?;
            println!("task added");
        },
        Command::Complete {id} => {
            todo_list.complete(id)?;
            println!("task completed");
        },
        _ => {
            println!("not yet implemented");
        }
    }
    todo_list.save_to_file("tasks.json")
        .map_err(|e| TodoError::FileError(e.to_string()))?;
    Ok(())
}

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run_command(cli) {
        // eprintln! is Equivalent to the println! macro, except that output goes to io::stderr instead of io::stdout. See println! for example usage.
        // Use eprintln! only for error and progress messages.
        // Use println! instead for the primary output of your program
        eprintln!("Error: {}", e);
        std::process::exit(1)    //exit from main value 1
    }
    
}
