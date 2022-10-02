use super::model::Post;
use super::req::CreatePostPayload;
use crate::db::PgPool;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn create_post(pool: web::Data<PgPool>, body: web::Json<CreatePostPayload>) -> HttpResponse {
    let create_post = Post::create(pool, body);

    match create_post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error:{:?}", err)),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(create_post);
}
