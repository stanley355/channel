use super::req::{CreatePostPayload, ViewHomePostsPayload};
use super::{model::Post, req::ViewChannelPostParam};
use crate::channel::model::Channel;
use crate::db::PgPool;
use actix_web::web::Json;
use actix_web::{get, post, web, HttpResponse};

#[post("/")]
async fn create_post(pool: web::Data<PgPool>, body: web::Json<CreatePostPayload>) -> HttpResponse {
    let create_post = Post::create(pool.clone(), Json(body.clone()));

    match create_post {
        Ok(post) => {
            let update_count = Channel::update_posts_count(pool.clone(), body.channels_id);
            match update_count {
                Ok(_) => println!("Post count updated"),
                Err(err) => println!("Fail to update count: {:?}", err),
            };
            HttpResponse::Ok().json(post)
        }
        Err(err) => HttpResponse::InternalServerError().body(format!("Error:{:?}", err)),
    }
}

#[get("/")]
async fn view_posts(
    pool: web::Data<PgPool>,
    param: web::Query<ViewChannelPostParam>,
) -> HttpResponse {
    let channel_res = Channel::check_channel_by_slug(pool.clone(), param.slug.clone());

    match channel_res {
        Ok(channel) => {
            let post_list = Post::view_posts(pool, channel.id);

            match post_list {
                Ok(post) => HttpResponse::Ok().json(post),
                Err(err) => HttpResponse::BadRequest().body(format!("Error: {:?}", err)),
            }
        }
        Err(err) => HttpResponse::BadRequest().body(format!("Error: {:?}", err)),
    }
}

#[get("/home/")]
async fn view_home_posts(
    pool: web::Data<PgPool>,
    body: web::Json<ViewHomePostsPayload>,
) -> HttpResponse {
    let posts_result = match body.subscriptions.len() {
        0 => Post::view_free_posts(pool),
        _ => Post::view_home_posts(pool, body.subscriptions.clone()),
    };

    match posts_result {
        Ok(posts) => HttpResponse::Ok().json(posts),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(create_post)
        .service(view_posts)
        .service(view_home_posts);
}
