use super::model::Channel;
use super::req::*;
use super::res::{ChannelErrorRes, ChannelTokenRes};
use crate::db::PgPool;
use crate::post::model::Post;
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
    if let Some(id) = param.id.clone() {
        let channel_exist = Channel::check_channel_by_id(pool, id);

        match channel_exist {
            Ok(channel) => {
                let token = Channel::hash_channel_data(channel);
                HttpResponse::Ok().json(ChannelTokenRes::new(token))
            }
            Err(_) => HttpResponse::BadRequest().body("Error: Channel doesn't exist!"),
        }
    } else if let Some(owner_id) = param.owner_id.clone() {
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

#[get("/subscriptions/")]
async fn find_subscribed_channels(
    pool: web::Data<PgPool>,
    param: web::Json<FindSubscribedChannelsPayload>,
) -> HttpResponse {
    let channels_res = Channel::find_subscribed_channels(pool, param.id_list.clone());

    match channels_res {
        Ok(channels) => HttpResponse::Ok().json(channels),
        Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
    }
}

#[put("/")]
async fn update_channel_data(
    pool: web::Data<PgPool>,
    payload: web::Json<UpdateChannelPayload>,
) -> HttpResponse {
    match payload.channel_name.clone() {
        Some(channel_name) => {
            let existing_channel = Channel::check_channel(pool.clone(), &channel_name);

            match existing_channel {
                Ok(channel) => {
                    let res = ChannelErrorRes::new(format!(
                        "Channel dengan nama {} sudah terdaftar !",
                        channel.channel_name
                    ));
                    HttpResponse::BadRequest().json(res)
                }
                Err(_) => {
                    let channel_update = Channel::update(pool.clone(), payload);

                    match channel_update {
                        Ok(channel) => {
                            let _post_channel_update = Post::update_channel(
                                pool,
                                channel.id.clone(),
                                channel.slug.clone(),
                            );

                            HttpResponse::Ok().json(channel)
                        }
                        Err(err) => {
                            HttpResponse::InternalServerError().body(format!("Error : {:?}", err))
                        }
                    }
                }
            }
        }
        None => {
            let channel_update = Channel::update(pool, payload);

            match channel_update {
                Ok(update) => HttpResponse::Ok().json(update),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error : {:?}", err)),
            }
        }
    }
}

#[put("/subscribers/")]
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
        .service(find_subscribed_channels)
        .service(update_channel_data)
        .service(update_channel_subscribers)
        .service(search_similar_channel);
}
