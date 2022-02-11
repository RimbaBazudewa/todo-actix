use crate::db;
use crate::errors::AppError;
use crate::models::{AppState, CreateTodoList, ResultResponse, Status};
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use slog::{crit, error, o, Logger};

pub async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
    pool.get().await.map_err(|err| {
        let sublog = log.new(o!("cause" => err.to_string()));
        crit!(sublog, "Error createing client");
        AppError::from(err)
    })
}
pub fn log_error(log: Logger) -> impl Fn(AppError) -> AppError {
    move |err| {
        let sublog = log.new(o!("cause" => err.cause.clone()));
        error!(sublog, "{}", err.message());
        err
    }
}

pub async fn status() -> impl Responder {
    web::HttpResponse::Ok().json(Status {
        status: "OK".to_string(),
    })
}

pub async fn get_todos(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "get_todos"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    let result = db::get_todos(&client).await;
    result
        .map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(log))
}
pub async fn get_items(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "get_items"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    let result = db::get_item(&client, path.0).await;
    result
        .map(|td| HttpResponse::Ok().json(td))
        .map_err(log_error(log))
}
pub async fn create_todo(
    state: web::Data<AppState>,
    json: web::Json<CreateTodoList>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "create_todo"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    let result = db::create_todo(&client, json.title.clone()).await;
    result
        .map(|todos| HttpResponse::Ok().json(todos))
        .map_err(log_error(log))
}
pub async fn check_item(
    state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler" => "check item"));
    let client = get_client(state.pool.clone(), log.clone()).await?;

    let result = db::check_item(&client, path.0, path.1).await;
    result
        .map(|update| HttpResponse::Ok().json(ResultResponse { success: update }))
        .map_err(log_error(log))
}
