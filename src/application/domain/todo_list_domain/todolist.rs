use super::todo::Task;
use serde::{Deserialize, Serialize};
use crate::application::domain::todo_list_domain::todolist_errors::TodolistError;

#[derive(Serialize,Deserialize,Debug)]
pub struct TodoList {
    tasks: Vec<Task>,
    next_id: usize,
}

impl TodoList {
    fn new() -> Self {
        Self {
            tasks: Vec::new(),
            next_id: 1,
        }
    }

    pub fn add_task(&mut self, description: String) -> Result<(), TodolistError> {
        if description.trim().is_empty() {
            return Err(TodolistError::InvalidInput("Task description cannot be empty".to_string()));
        }
        let task = Task::new(self.next_id, description);
        self.tasks.push(task);
        self.next_id += 1;
        Ok(())
    }

    pub fn complete(&mut self, id: usize) -> Result<(), TodolistError> {
        match self.tasks.iter_mut().find(|task| task.id() == id) { //Option<&mut Task>
            Some(mut_ref_task) => {
                mut_ref_task.complete();
                Ok(())
            },
            None => Err(TodolistError::TaskNotFound(id)),
        }
    }

    //********************  PERSISTENCE

    pub fn restore_from_pdata(data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(serde_json::from_str(&data)?)
    }

    pub fn get_pdata_for(&self) -> Result<String, Box<dyn std::error::Error>> {
        Ok(serde_json::to_string_pretty(self)?)
    }

}
