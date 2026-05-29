use serde::{Deserialize, Serialize};

#[derive(Serialize,Deserialize,Debug)]
pub struct Task {
    id: usize,
    description: String,
    completed: bool,
}

impl Task {
    pub fn new(id: usize, description: String) -> Self {
        Task {
            id,
            description,
            completed: false,
        }    
    }
    pub fn id(&self) -> usize {
        self.id
    }
    pub fn complete(&mut self) {
        self.completed = true;
    }
}