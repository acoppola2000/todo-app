use std::fmt;
use std::fmt::Formatter;

#[derive(Debug)]
pub enum TodoError {
    FileError(String),
    ParseError(String),
    TaskNotFound(usize),
    InvalidInput(String)
}

impl fmt::Display for TodoError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {  //TODO AMC cos'è fmt::Result ? dove scrive ? cosa fa write! ?
        match self {
            TodoError::FileError(msg) => write!(f,"File error: {}",msg),
            TodoError::ParseError(msg) => write!(f,"Parse error: {}",msg),
            TodoError::TaskNotFound(id) => write!(f,"Task {} not found",id),
            TodoError::InvalidInput(msg) => write!(f,"Invalid input: {}",msg),
        }
    }
}

//TODO AMC che significa
impl std::error::Error for TodoError {} 