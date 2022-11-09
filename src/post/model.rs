use super::req::CreatePostPayload;
use crate::db::PgPool;
use crate::schema::{channels, posts};

use actix_web::web;
use diesel::{
    BoolExpressionMethods, ExpressionMethods, JoinOnDsl, NullableExpressionMethods, QueryDsl,
    QueryResult, RunQueryDsl,
};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Post {
    id: i32,
    channels_id: i32,
    channels_slug: String,
    created_at: chrono::NaiveDateTime,
    img_url: String,
    description: String,
    likes: i32,
    post_type: String,
    is_free: bool,
}

// #[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
// pub struct Poster {
//     id: i32,
//     channels_id: i32,
//     channels_slug: String,
//     channel_name: String,
// }

impl Post {
    pub fn create(
        pool: web::Data<PgPool>,
        body: web::Json<CreatePostPayload>,
    ) -> QueryResult<Post> {
        let conn = &pool.get().unwrap();

        let post_type = body.post_type.to_string();
        let data = (
            (posts::channels_id.eq(&body.channels_id)),
            (posts::channels_slug.eq(&body.channels_slug)),
            (posts::img_url.eq(&body.img_url)),
            (posts::description.eq(&body.description)),
            (posts::post_type.eq(&post_type)),
            (posts::is_free.eq(&body.is_free)),
        );

        diesel::insert_into(posts::table)
            .values(data)
            .get_result::<Post>(conn)
    }

    pub fn view_posts(pool: web::Data<PgPool>, slug: String) -> QueryResult<Vec<Post>> {
        let conn = &pool.get().unwrap();
        posts::table
            .filter(posts::channels_slug.eq(slug))
            .order(posts::created_at.desc())
            .get_results(conn)
    }

    pub fn view_free_posts(pool: web::Data<PgPool>) -> QueryResult<Vec<Post>> {
        let conn = &pool.get().unwrap();

        posts::table
            .filter(posts::is_free.eq(true))
            .order(posts::created_at.desc())
            .limit(20)
            .get_results(conn)
    }

    pub fn view_home_posts(
        pool: web::Data<PgPool>,
        subscriptions: Vec<i32>,
    ) -> QueryResult<Vec<Post>> {
        let conn = &pool.get().unwrap();

        // let selection = (
        //     posts::id,
        //     posts::channels_id,
        //     posts::channels_slug,
        //     channels::channel_name,
        // );

        // posts::table
        //     .filter(
        //         posts::channels_id
        //             .eq_any(subscriptions)
        //             .or(posts::is_free.eq(true)),
        //     )
        //     .order(posts::created_at.desc())
        //     .left_join(channels::table.on(posts::channels_id.eq(channels::id)))
        //     .select(selection.nullable())
        //     .get_results(conn)


        posts::table
            .filter(
                posts::channels_id
                    .eq_any(subscriptions)
                    .or(posts::is_free.eq(true)),
            )
            .order(posts::created_at.desc())
            .get_results(conn)
    }
}
