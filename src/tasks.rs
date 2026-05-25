use serde::{Deserialize, Serialize};
//use


#[derive(Serialize,Deserialize,Debug)]
struct Task {
    id: usize,
    description: String,
    completed: bool,
}

#[derive(Serialize,Deserialize,Debug)]
pub struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

use std::path::Path;
use std::fs;
use std::ptr::addr_of_mut;
use crate::errors::TodoError;

impl TodoList {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn load_from_file(filename: &str) -> Result<Self,Box<dyn std::error::Error>> {
        if Path::new(filename).exists() {
            let contents = fs::read_to_string(filename)?;
            let todo_list = serde_json::from_str(&contents)?;
            Ok(todo_list)
        } else {
            Ok(TodoList::new())
        }
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(),Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(filename, json)?;
        Ok(())
    }


    pub fn add_task(&mut self, description: String) -> Result<(), crate::errors::TodoError> {
        if description.trim().is_empty() {
            return Err(TodoError::InvalidInput("Task description cannot be empty".to_string()));
        }
        let task = Task {
            id: self.next_id,
            description,
            completed: false,
        };
        self.tasks.push(task);
        self.next_id += 1;
        Ok(())
    }

    pub fn complete(&mut self, id: usize) -> Result<(),TodoError> {
        match self.tasks.iter_mut().find(|task| task.id == id) { //Option<&mut Task>
            Some(mut_ref_task) => {
                mut_ref_task.completed = true;
                Ok(())
            },
            None => Err(TodoError::TaskNotFound(id)),
        }
    }
}
