use anyhow;
use std::sync::Arc;

use todos_domain::{
    model::todo::{CreateTodo, Todo, UpdateTodo},
    repository::todo::TodoRepository,
};

pub struct TodoUseCase<T: TodoRepository> {
    todo_repository: Arc<T>,
}

impl<T: TodoRepository> TodoUseCase<T> {
    pub fn new(todo_repository: Arc<T>) -> Self {
        Self { todo_repository }
    }

    pub async fn all(&self) -> Vec<Todo> {
        self.todo_repository.all().await
    }

    pub async fn find(&self, id: i32) -> Option<Todo> {
        self.todo_repository.find(id).await
    }

    pub async fn create(&self, payload: CreateTodo) -> Todo {
        self.todo_repository.create(payload).await
    }

    pub async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo> {
        self.todo_repository.update(id, payload).await
    }

    pub async fn delete(&self, id: i32) -> anyhow::Result<()> {
        self.todo_repository.delete(id).await
    }
}
