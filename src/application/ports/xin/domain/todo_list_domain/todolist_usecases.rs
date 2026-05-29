pub trait AddTodoUseCase {
    fn add(&self, description: String) -> Result<(),Box<dyn std::error::Error>>;
}

pub trait CompleteTodoUseCase {
    fn complete(&self, id: usize) -> Result<(),Box<dyn std::error::Error>>;
}