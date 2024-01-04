use todos_domain::model::todo::{CreateTodo, UpdateTodo};

use axum::{
    async_trait,
    extract::{Extension, FromRequest, Path, Request},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::de::DeserializeOwned;
use std::sync::Arc;
use validator::Validate;

use crate::module::Modules;

#[derive(Debug)]
pub struct ValidatedJson<T>(T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidatedJson<T>
where
    T: DeserializeOwned + Validate,
    S: Send + Sync,
{
    type Rejection = (StatusCode, String);

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state)
            .await
            .map_err(|rejection| {
                let message = format!("Json parse error: [{}]", rejection);
                (StatusCode::BAD_REQUEST, message)
            })?;
        value.validate().map_err(|rejection| {
            let message = format!("Validation error: [{}]", rejection).replace('\n', ", ");
            (StatusCode::BAD_REQUEST, message)
        })?;
        Ok(ValidatedJson(value))
    }
}

pub async fn create_todo(
    Extension(module): Extension<Arc<Modules>>,
    ValidatedJson(payload): ValidatedJson<CreateTodo>,
) -> impl IntoResponse {
    let todo = module.todo_use_case().create(payload).await;

    (StatusCode::CREATED, Json(todo))
}

pub async fn find_todo(
    Extension(module): Extension<Arc<Modules>>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = module
        .todo_use_case()
        .find(id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;
    Ok((StatusCode::OK, Json(todo)))
}

pub async fn all_todo(Extension(module): Extension<Arc<Modules>>) -> impl IntoResponse {
    let todo = module.todo_use_case().all().await;
    (StatusCode::OK, Json(todo))
}

pub async fn update_todo(
    Extension(module): Extension<Arc<Modules>>,
    Path(id): Path<i32>,
    ValidatedJson(payload): ValidatedJson<UpdateTodo>,
) -> Result<impl IntoResponse, StatusCode> {
    let todo = module
        .todo_use_case()
        .update(id, payload)
        .await
        .or(Err(StatusCode::NOT_FOUND))?;
    Ok((StatusCode::CREATED, Json(todo)))
}

pub async fn delete_todo(
    Extension(module): Extension<Arc<Modules>>,
    Path(id): Path<i32>,
) -> StatusCode {
    module
        .todo_use_case()
        .delete(id)
        .await
        .map(|_| StatusCode::NO_CONTENT)
        .unwrap_or(StatusCode::NOT_FOUND)
}
