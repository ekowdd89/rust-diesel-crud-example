use crate::db::models::posts::{NewPost, Post, PostService};
use actix_web::{get, post, web::Json, HttpResponse, Responder};

#[get("/posts")]
pub async fn get_posts() -> impl Responder {
    let posts = Post::all().unwrap();
    HttpResponse::Ok().json(posts)
}

#[post("/posts")]
pub async fn create_post(payload: Json<NewPost>) -> impl Responder {
    let post = Post::create_post(payload.into_inner()).unwrap();
    HttpResponse::Ok().json(post)
}
