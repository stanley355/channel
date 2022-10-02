use super::req::CreatePostPayload;
use crate::db::PgPool;
use crate::schema::posts;

use actix_web::web;
use diesel::{ExpressionMethods, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Post {
    id: i32,
    channels_id: i32,
    created_at: chrono::NaiveDateTime,
    img_url: String,
    description: String,
    likes: i32,
    post_type: String,
    is_free: bool,
}

impl Post {
    pub fn create(
        pool: web::Data<PgPool>,
        body: web::Json<CreatePostPayload>,
    ) -> QueryResult<Post> {
        let conn = &pool.get().unwrap();

        let post_type = body.post_type.to_string();
        let data = (
            (posts::channels_id.eq(&body.channels_id)),
            (posts::img_url.eq(&body.img_url)),
            (posts::description.eq(&body.description)),
            (posts::post_type.eq(&post_type)),
            (posts::is_free.eq(&body.is_free)),
        );

        diesel::insert_into(posts::table)
            .values(data)
            .get_result::<Post>(conn)
    }
}
