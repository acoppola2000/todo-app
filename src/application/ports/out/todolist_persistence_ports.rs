// Specifying the parent module when calling the function
// makes it clear that the function isn’t locally defined
// while still minimizing repetition of the full path

// when bringing in structs, enums, and other items with use,
// it’s idiomatic to specify the full path.

use std::error;
pub trait SaveTodoListOutPort {
    fn insert_data(&self, data: String) -> Result<(),Box<dyn std::error::Error>>;
    //TODO
     //AccountDATA newData
}


pub trait ReadTodoListOutPort {
    fn read_data(&self) -> Result<Option<String>, Box<dyn error::Error>>;
    //TODO
    //AccountDATA newData
}