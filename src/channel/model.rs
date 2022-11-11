use super::req::{CreateChannelPayload, SearchSimilarChannelQuery};
use crate::db::PgPool;
use crate::schema::channels;

use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl, TextExpressionMethods};
use jsonwebtokens as jwt;
use jwt::{encode, Algorithm, AlgorithmID};
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, AsExpression)]
pub struct Channel {
    pub id: i32,
    pub owner_id: uuid::Uuid,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub channel_name: String,
    pub slug: String,
    pub subscribers: i32,
    pub posts_number: i32,
    pub subscription_price: i32,
    pub profile_img_url: Option<String>,
    pub background_img_url: Option<String>,
}

impl Channel {
    pub fn check_channel(pool: web::Data<PgPool>, name: &String) -> QueryResult<Channel> {
        let conn = &pool.get().unwrap();
        channels::table
            .filter(channels::channel_name.eq(name))
            .get_result::<Channel>(conn)
    }

    pub fn check_channel_by_owner(
        pool: web::Data<PgPool>,
        owner_id: String,
    ) -> QueryResult<Channel> {
        let conn = &pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&owner_id).unwrap();

        channels::table
            .filter(channels::owner_id.eq(uuid))
            .get_result(conn)
    }

    pub fn check_channel_by_slug(pool: web::Data<PgPool>, slug: String) -> QueryResult<Channel> {
        let conn = &pool.get().unwrap();

        channels::table
            .filter(channels::slug.eq(&slug))
            .get_result(conn)
    }

    pub fn create(
        pool: web::Data<PgPool>,
        body: web::Json<CreateChannelPayload>,
    ) -> QueryResult<Channel> {
        let conn = &pool.get().unwrap();
        let uuid = uuid::Uuid::parse_str(&body.owner_id).unwrap();
        let slug = &body.channel_name.trim().replace(" ", "-").to_lowercase();

        let data = (
            (channels::owner_id.eq(uuid)),
            (channels::channel_name.eq(&body.channel_name)),
            (channels::slug.eq(slug)),
            (channels::subscription_price.eq(&body.subscription_price)),
            (channels::profile_img_url.eq(&body.profile_img_url))
        );

        diesel::insert_into(channels::table)
            .values(data)
            .get_result::<Channel>(conn)
    }

    pub fn hash_channel_data(channel: Channel) -> String {
        let alg = Algorithm::new_hmac(AlgorithmID::HS256, "secret").unwrap();
        let header = json!({ "alg": alg.name() });
        let body = json!(channel);
        encode(&header, &body, &alg).unwrap()
    }

    pub fn update_posts_count(pool: web::Data<PgPool>, id: i32) -> QueryResult<usize> {
        let conn = &pool.get().unwrap();

        diesel::update(channels::table)
            .filter(channels::id.eq(id))
            .set(channels::posts_number.eq(channels::posts_number + 1))
            .execute(conn)
    }

    pub fn update_subscribers_count(pool: web::Data<PgPool>, id: i32) -> QueryResult<usize> {
        let conn = &pool.get().unwrap();

        diesel::update(channels::table)
            .filter(channels::id.eq(id))
            .set(channels::subscribers.eq(channels::subscribers + 1))
            .execute(conn)
    }

    pub fn search_similar_channel(pool: web::Data<PgPool>, query: web::Query<SearchSimilarChannelQuery>) -> QueryResult<Vec<Channel>> {
        let conn = &pool.get().unwrap();

        let query = format!("{}%", query.channel_name);

        channels::table
            .filter(channels::channel_name.like(query))
            .get_results(conn)
    }
}
