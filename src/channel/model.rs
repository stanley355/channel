use super::req::{CheckChannelParam, CreateChannelReq};
use crate::db::PgPool;
use crate::schema::channels;
use actix_web::web;
use diesel::{BoolExpressionMethods, ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Channel {
    pub id: uuid::Uuid,
    pub owner_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub channel_name: String,
    pub slug: String,
    pub subscribers: i32,
    pub posts_number: i32,
}

impl Channel {
    pub fn check_channel(pool: web::Data<PgPool>, name: &String) -> QueryResult<Channel> {
        let conn = &pool.get().unwrap();
        channels::table
            .filter(channels::channel_name.eq(name))
            .get_result(conn)
    }

    pub fn check_channel_by_owner(
        pool: web::Data<PgPool>,
        param: web::Query<CheckChannelParam>,
    ) -> QueryResult<Channel> {
        let conn = &pool.get().unwrap();
        println!("{:?}", param);
        channels::table
            .filter(
                channels::owner_id
                    .eq(&param.owner_id)
                    .and(channels::channel_name.eq(&param.channel_name)),
            )
            .get_result(conn)
    }

    pub fn create(
        pool: web::Data<PgPool>,
        body: web::Json<CreateChannelReq>,
    ) -> QueryResult<Channel> {
        let conn = &pool.get().unwrap();

        let slug = &body.channel_name.trim().replace(" ", "-");
        let data = (
            (channels::owner_id.eq(&body.owner_id)),
            (channels::channel_name.eq(&body.channel_name)),
            (channels::slug.eq(slug)),
        );

        diesel::insert_into(channels::table)
            .values(data)
            .get_result::<Channel>(conn)
    }
}
