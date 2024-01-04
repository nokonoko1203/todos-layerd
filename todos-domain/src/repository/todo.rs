use anyhow;
use async_trait::async_trait;
use thiserror::Error;

use crate::model::todo::{CreateTodo, Todo, UpdateTodo};

#[derive(Debug, Error)]
pub enum RepositoryError {
    #[error("NotFound, id is {0}")]
    NotFound(i32),
}

#[async_trait]
pub trait TodoRepository: Clone + std::marker::Send + std::marker::Sync + 'static {
    async fn create(&self, payload: CreateTodo) -> Todo;
    async fn find(&self, id: i32) -> Option<Todo>;
    async fn all(&self) -> Vec<Todo>;
    async fn update(&self, id: i32, payload: UpdateTodo) -> anyhow::Result<Todo>;
    async fn delete(&self, id: i32) -> anyhow::Result<()>;
}
