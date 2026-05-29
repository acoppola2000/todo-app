use std::rc::Rc;
use crate::application::domain::todo_list_domain::todolist::TodoList;
use crate::application::ports::xin::domain::todo_list_domain::todolist_usecases::{AddTodoUseCase,CompleteTodoUseCase};
use crate::application::ports::out::todolist_persistence_ports::{SaveTodoListOutPort, ReadTodoListOutPort};

pub struct TodolistAppService {
    save_todolist_outport: Rc<Box<dyn SaveTodoListOutPort>>,
    read_todolist_outport: Rc<Box<dyn ReadTodoListOutPort>>,
}

impl TodolistAppService {
    pub fn new(save_port: Rc<Box<dyn SaveTodoListOutPort>>, read_port: Rc<Box<dyn ReadTodoListOutPort>>) -> Self {
        Self {
            save_todolist_outport: save_port,
            read_todolist_outport: read_port,
        }        
    }
}

impl AddTodoUseCase for TodolistAppService {
    fn add(&self, description: String) -> Result<(),Box<dyn std::error::Error>> {
        let data = self.read_todolist_outport.read_data().unwrap().unwrap(); //TODO
        let mut todolist = TodoList::restore_from_pdata(&data)?;
        todolist.add_task(description)?;
        let data = todolist.get_pdata_for()?;
        self.save_todolist_outport.insert_data(data)?;
        Ok(())
    }
}
