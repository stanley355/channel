use super::model::Channel;
use super::req::{CheckChannelParam, CreateChannelReq};
use crate::db::PgPool;
use actix_web::{get, post, web, HttpResponse};

#[post("/")]
async fn create_channel(
    pool: web::Data<PgPool>,
    body: web::Json<CreateChannelReq>,
) -> HttpResponse {
    let channel_exist = Channel::check_channel(pool.clone(), &body.channel_name);

    match channel_exist {
        Ok(channel) => {
            if body.owner_id == channel.owner_id {
                HttpResponse::Ok().json(channel)
            } else {
                HttpResponse::BadRequest().body(format!(
                    "Channel with {:?} name exists!",
                    &body.channel_name
                ))
            }
        }
        Err(_) => {
            let channel_result = Channel::create(pool, body);

            match channel_result {
                Ok(channel) => HttpResponse::Ok().json(channel),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
            }
        }
    }
}

#[get("")]
async fn check_channel_by_owner(
    pool: web::Data<PgPool>,
    param: web::Query<CheckChannelParam>,
) -> HttpResponse {
    let channel_exist = Channel::check_channel_by_owner(pool, param);

    match channel_exist {
        Ok(channel) => HttpResponse::Ok().json(channel),
        Err(_) => HttpResponse::BadRequest().body("Error: Channel doesn't exist!"),
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(create_channel)
        .service(check_channel_by_owner);
}
