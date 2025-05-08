use diesel::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::cfg_postgres::pg_connection;
use crate::db::schema::posts;
#[derive(Queryable, Selectable, Clone, Debug, Serialize)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
    pub id: i64,
    pub title: String,
    pub content: String,
    pub published: bool,
}

#[derive(Insertable, Debug, Deserialize)]
#[diesel(table_name = posts)]
pub struct NewPost {
    pub title: String,
    pub content: String,
    pub published: bool,
}

#[derive(AsChangeset, Debug, Deserialize)]
#[diesel(table_name = posts)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
    pub published: Option<bool>,
}

pub trait PostService {
    fn all() -> Result<Vec<Post>, diesel::result::Error>;
    fn create_post(new_post: NewPost) -> Result<Post, diesel::result::Error>;
    fn update_post(id: i64, update_post: UpdatePost) -> Result<Post, diesel::result::Error>;
    fn delete_post(id: i64) -> Result<Post, diesel::result::Error>;
    fn show_post(id: i64) -> Result<Post, diesel::result::Error>;
}
impl PostService for Post {
    fn all() -> Result<Vec<Post>, diesel::result::Error> {
        posts::table.load::<Post>(&mut pg_connection())
    }
    fn show_post(id: i64) -> Result<Post, diesel::result::Error> {
        posts::table.find(id).get_result(&mut pg_connection())
    }
    fn create_post(new_post: NewPost) -> Result<Post, diesel::result::Error> {
        diesel::insert_into(posts::table)
            .values(&new_post)
            .get_result(&mut pg_connection())
    }
    fn update_post(id: i64, update_post: UpdatePost) -> Result<Post, diesel::result::Error> {
        diesel::update(posts::table.find(id))
            .set(&update_post)
            .get_result(&mut pg_connection())
    }
    fn delete_post(id: i64) -> Result<Post, diesel::result::Error> {
        diesel::delete(posts::table.find(id)).get_result(&mut pg_connection())
    }
}
