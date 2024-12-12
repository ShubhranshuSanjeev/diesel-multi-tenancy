use diesel::{prelude::{Insertable, Queryable}, query_builder::AsChangeset, Selectable};
use serde::{Deserialize, Serialize};

use crate::db::schema::items;

#[derive(Queryable, Insertable, Serialize, Deserialize, Selectable, AsChangeset)]
#[diesel(check_for_backend(diesel::pg::Pg))]
#[diesel(primary_key(id))]
#[diesel(table_name = items)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

#[derive(Insertable, Serialize, Deserialize, Selectable, AsChangeset)]
#[diesel(table_name = items)]
pub struct NewItem {
    pub name: String,
    pub description: Option<String>,
}
