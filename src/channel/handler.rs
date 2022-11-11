use super::model::Channel;
use super::req::*;
use super::res::{ChannelErrorRes, ChannelTokenRes};
use crate::db::PgPool;
use actix_web::{get, post, put, web, HttpResponse};

#[post("/")]
async fn create_channel(
    pool: web::Data<PgPool>,
    body: web::Json<CreateChannelPayload>,
) -> HttpResponse {
    let channel_exist = Channel::check_channel(pool.clone(), &body.channel_name);

    match channel_exist {
        Ok(channel) => {
            let uuid = uuid::Uuid::parse_str(&body.owner_id);

            if uuid.unwrap() == channel.owner_id {
                let token = Channel::hash_channel_data(channel);
                HttpResponse::Ok().json(ChannelTokenRes::new(token))
            } else {
                let error_msg = format!("Channel with {:?} name exists!", &body.channel_name);
                let error_res = ChannelErrorRes::new(error_msg);
                HttpResponse::BadRequest().json(error_res)
            }
        }
        Err(_) => {
            let channel_result = Channel::create(pool, body);

            match channel_result {
                Ok(channel) => {
                    let token = Channel::hash_channel_data(channel);
                    HttpResponse::Ok().json(ChannelTokenRes::new(token))
                }
                Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
            }
        }
    }
}

#[get("/")]
async fn check_channel(
    pool: web::Data<PgPool>,
    param: web::Query<CheckChannelParam>,
) -> HttpResponse {
    if let Some(owner_id) = param.owner_id.clone() {
        let channel_exist = Channel::check_channel_by_owner(pool, owner_id);

        match channel_exist {
            Ok(channel) => {
                let token = Channel::hash_channel_data(channel);
                HttpResponse::Ok().json(ChannelTokenRes::new(token))
            }
            Err(_) => HttpResponse::BadRequest().body("Error: Channel doesn't exist!"),
        }
    } else if let Some(slug) = param.slug.clone() {
        let channel_exist = Channel::check_channel_by_slug(pool, slug);

        match channel_exist {
            Ok(channel) => HttpResponse::Ok().json(channel),
            Err(_) => HttpResponse::BadRequest().body("Error: Channel doesn't exist!"),
        }
    } else {
        HttpResponse::BadRequest().body("Missing Parameter")
    }
}

#[put("/")]
async fn update_channel_subscribers(
    pool: web::Data<PgPool>,
    param: web::Query<UpdateChannelParam>,
) -> HttpResponse {
    let channel_update = Channel::update_subscribers_count(pool, param.channel_id);

    match channel_update {
        Ok(update) => HttpResponse::Ok().json(update),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
    }
}

#[get("/search/")]
async fn search_similar_channel(
    pool: web::Data<PgPool>,
    query: web::Query<SearchSimilarChannelQuery>,
) -> HttpResponse {
    let similar_channel = Channel::search_similar_channel(pool, query);

    match similar_channel {
        Ok(channels) => HttpResponse::Ok().json(channels),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
    }
}
pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(create_channel)
        .service(check_channel)
        .service(update_channel_subscribers)
        .service(search_similar_channel);
}
