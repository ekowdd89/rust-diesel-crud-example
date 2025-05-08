use crate::{HandlerResponse, Response};
use actix_web::{get, http::StatusCode, Either, HttpResponse};
use serde_json::json;

#[get("/hey")]
pub async fn manual_hello() -> HandlerResponse {
    let code = StatusCode::BAD_REQUEST;
    if code != StatusCode::CREATED {
        let val = Response {
            status: false,
            code: code.as_u16(),
            message: "Bad Request".to_string(),
            data: "",
        };
        return Either::Left(HttpResponse::Ok().status(StatusCode::BAD_REQUEST).json(val));
    }
    Either::Left(HttpResponse::Ok().status(StatusCode::CREATED).json(json!({
        "code":code.as_u16(),
        "message":"successfully !",
    })))
}
#[get("/ping")]
pub async fn health_check() -> HandlerResponse {
    return Either::Left(HttpResponse::Ok().status(StatusCode::OK).body("pong"));
}
