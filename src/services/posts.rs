use crate::db::models::posts::{NewPost, Post, PostService, UpdatePost};
use actix_web::{
    delete, get, post, put,
    web::{self, Json},
    HttpResponse, Responder,
};

#[get("/posts")]
pub async fn get_posts() -> impl Responder {
    let posts = Post::all().unwrap();
    HttpResponse::Ok().json(posts)
}

#[get("/posts/{id}")]
pub async fn get_post(id: web::Path<i64>) -> impl Responder {
    let post = Post::show_post(id.into_inner()).unwrap();
    HttpResponse::Ok().json(post)
}

#[post("/posts")]
pub async fn create_post(payload: Json<NewPost>) -> impl Responder {
    let post = Post::create_post(payload.into_inner()).unwrap();
    HttpResponse::Ok().json(post)
}

#[put("/posts/{id}")]
pub async fn update_post(id: web::Path<i64>, payload: Json<UpdatePost>) -> impl Responder {
    let post = Post::update_post(id.into_inner(), payload.into_inner()).unwrap();
    HttpResponse::Ok().json(post)
}

#[delete("/posts/{id}")]
pub async fn delete_post(id: web::Path<i64>) -> impl Responder {
    let post = Post::delete_post(id.into_inner()).unwrap();
    HttpResponse::Ok().json(post)
}
