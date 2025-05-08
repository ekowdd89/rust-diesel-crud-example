use crate::services;

pub fn config(conf: &mut actix_web::web::ServiceConfig) {
    let scope = actix_web::web::scope("/v1")
        .service(services::backpdf_queue_service::manual_hello)
        .service(services::posts::get_posts)
        .service(services::posts::create_post);
    conf.service(scope);
}
