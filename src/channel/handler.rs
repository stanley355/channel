use super::model::Channel;
use super::req::CreateChannelReq;
use crate::db::PgPool;
use actix_web::{post, web, HttpResponse};

#[post("/")]
async fn create_channel(
    pool: web::Data<PgPool>,
    body: web::Json<CreateChannelReq>,
) -> HttpResponse {
    let channel_exist = Channel::check_channel(pool.clone(), &body.channel_name);

    match channel_exist {
        Ok(_) => HttpResponse::BadRequest().body(format!(
            "Channel with {:?} name exists!",
            &body.channel_name
        )),
        Err(_) => {
            let channel_result = Channel::create(pool, body);

            match channel_result {
                Ok(channel) => HttpResponse::Ok().json(channel),
                Err(err) => HttpResponse::InternalServerError().body(format!("Error: {:?}", err)),
            }
        }
    }
}

pub fn route(config: &mut web::ServiceConfig) {
    config.service(create_channel);
}
