mod application;
mod adapters;


use crate::adapters::out::todolist_file_persistence_adapter::TodolistFilePersAdapter;
use crate::application::domain::todo_list_domain::todolist::TodoList;
use crate::application::domain::todo_list_domain::todolist_appservice::TodolistAppService;

//TodolistAppService
use crate::application::domain::todo_list_domain::todolist_errors::TodolistError;

fn main() {
    let persistence =  TodolistFilePersAdapter::new("tasks.json").unwrap();
    let s = persistence.read_data().unwrap().unwrap();
    let lista = TodoList::restore_from_pdata(&s);
    
    println!("{:?}", lista);
}
