use std::{error, fmt};
use std::fmt::Formatter;

#[derive(Debug)]
pub enum TodolistFilePersAdapterError {
    FilenameNotValid(String)
}
impl fmt::Display for TodolistFilePersAdapterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {  //TODO AMC cos'è fmt::Result ? dove scrive ? cosa fa write! ?
        match self {
            TodolistFilePersAdapterError::FilenameNotValid(msg) => write!(f, "Todolist persistence error: {}", msg),
        }
    }
}
impl error::Error for TodolistFilePersAdapterError {}

