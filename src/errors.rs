use std::fmt;
use std::fmt::Formatter;

//for custom errors, it's best practice to implement display and std:errors:Error

#[derive(Debug)]
pub enum TodoError {
    FileError(String),
    ParseError(String),
    TaskNotFound(usize),
    InvalidInput(String)
}

// Display - Implementing this trait for a type will automatically implement the ToString trait for the type,
//           allowing the usage of the .to_string() method.
// Prefer implementing the Display trait for a type, rather than ToString.
// Display is similar to Debug, but Display is for user-facing output, and so cannot be derived.
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


// Errors must describe themselves through the Display and Debug traits.
// Error messages are typically concise lowercase sentences without trailing punctuation:
impl std::error::Error for TodoError {
    // Provided methods
    // fn source(&self) -> Option<&(dyn Error + 'static)> { ... }
    // fn description(&self) -> &str { ... }
    // fn cause(&self) -> Option<&dyn Error> { ... }
    // fn provide<'a>(&'a self, request: &mut Request<'a>) { ... }


    // source:
    // If one module must report an error that is caused by an error from a lower-level module,
    // it can allow accessing that error via Error::source().
    // This makes it possible for the high-level module to provide its own errors
    // while also revealing some of the implementation for debugging.
    //
    // In error types that wrap an underlying error, the underlying error should be either
    // returned by the outer error’s Error::source(),
    // or rendered by the outer error’s Display implementation, but not both. //TODO AMC perchè ?
}