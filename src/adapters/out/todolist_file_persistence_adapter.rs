use std::error;
use std::fs;
use crate::application::ports::out::todolist_persistence_ports::{SaveTodoListOutPort, ReadTodoListOutPort};
use super::todolist_file_persistence_adapter_errors::TodolistFilePersAdapterError;


/*
pub fn notify(item: &(impl Summary + Display)) {  }
pub fn notify<T: Summary + Display>(item: &T) { }
pub fn notify<T>(item: &T)
where
    T:  Summary + Display { }
 */

/*
// Supertrait combinato
trait TraitAB: TraitA + TraitB {}

// Implementazione blanket: chiunque implementi A+B ottiene AB gratis
impl<T: TraitA + TraitB> TraitAB for T {}
 */

//******************   SUPERTRAIT
pub trait TodolistFilePersAdapterI: SaveTodoListOutPort + ReadTodoListOutPort {}
impl<I: SaveTodoListOutPort + ReadTodoListOutPort> TodolistFilePersAdapterI for I {}

pub struct TodolistFilePersAdapter {
    filename: String,
}
impl TodolistFilePersAdapter {
    pub fn new(filename: &str) -> Result<Box<dyn TodolistFilePersAdapterI>, Box<dyn error::Error>>
    {
        if filename.trim().is_empty() {
            Err(Box::new(TodolistFilePersAdapterError::FilenameNotValid("filename should not be empty".to_string())))
        } else {
            Ok(Box::new(Self {
                filename: filename.to_string()
            }))
        }
    }
}

impl SaveTodoListOutPort for  TodolistFilePersAdapter {
    fn insert_data(&self, data: String) -> Result<(),Box<dyn std::error::Error>> {
        //TODO
        fs::write(&self.filename, data)?;
        Ok(())
    }
}

impl ReadTodoListOutPort for TodolistFilePersAdapter {
    fn read_data(&self) -> Result<Option<String>, Box<dyn error::Error>> {
        //TODO
        Ok(Some(fs::read_to_string(&self.filename)?))
    }
}


