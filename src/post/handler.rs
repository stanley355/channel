use super::req::CreatePostPayload;
use super::{model::Post, req::ViewChannelPostParam};
use crate::db::PgPool;
use actix_web::{get, post, web, HttpResponse};

#[post("/")]
async fn create_post(pool: web::Data<PgPool>, body: web::Json<CreatePostPayload>) -> HttpResponse {
    let create_post = Post::create(pool, body);

    match create_post {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error:{:?}", err)),
    }
}

#[get("/")]
async fn view_posts(
    pool: web::Data<PgPool>,
    param: web::Query<ViewChannelPostParam>,
) -> HttpResponse {
    let post_exist = Post::view_posts(pool, param.channel_id);

    match post_exist {
        Ok(post) => HttpResponse::Ok().json(post),
        Err(err) => HttpResponse::BadRequest().body(format!("Error: {:?}", err)),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(create_post).service(view_posts);
}