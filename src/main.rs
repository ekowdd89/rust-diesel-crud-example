use core::str;

use actix_cors::Cors;
use actix_web::{
    middleware::{self, Logger},
    App, HttpServer,
};
use actix_web_boilerplate::{config::cfg_postgres::pg_connection, db, handlers, services};
use dotenv::dotenv;
use env_logger::Env;

#[derive(Clone)]
pub struct AppState {
    pub db: std::sync::Arc<std::sync::Mutex<diesel::PgConnection>>,
    pub posts: db::models::posts::Post,
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    if std::env::var_os("APP_LOG").is_none() {
        std::env::set_var("APP_LOG", "actix_web=info");
    }
    dotenv().expect("Failed to read .env file");
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Compress::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials(),
            )
            .wrap(Logger::default())
            .configure(handlers::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
