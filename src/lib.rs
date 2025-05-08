use actix_web::{Either, Error, HttpResponse};
use serde::Serialize;

pub mod config;
pub mod db;
pub mod handlers;
pub mod services;
pub type HandlerResponse = Either<HttpResponse, Result<&'static str, Error>>;

#[derive(Serialize, Debug)]
struct Response<T> {
    status: bool,
    code: u16,
    message: String,
    data: T,
}
