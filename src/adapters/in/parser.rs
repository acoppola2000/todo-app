use clap::{Parser, Subcommand};

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
