mod db;
mod diesel_ext;

use actix_web::web::{Data, Json};
use actix_web::{web, App, HttpServer};
use db::models::{Item, NewItem};
use db::schema::items;
use diesel::r2d2::{self, ConnectionManager};
use diesel::{PgConnection, RunQueryDsl};
use diesel_ext::WithSchema;
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn get_items(pool: Data<DbPool>) -> actix_web::Result<Json<Vec<Item>>> {
    let mut conn = pool.get().unwrap();
    let items = items::table
        .with_schema("test")
        .load::<Item>(&mut conn)
        .map_err(|e| e.to_string())
        .unwrap();
    Ok(Json(items))
}

async fn create_item(pool: Data<DbPool>, item: Json<NewItem>) -> actix_web::Result<Json<Item>> {
    let mut conn = pool.get().unwrap();
    let new_item = item.into_inner();

    let inserted_item: Item = diesel::insert_into(items::table)
        .values(&new_item)
        .get_result(&mut conn)
        .map_err(|e| e.to_string())
        .unwrap();
    Ok(Json(inserted_item))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool");

    HttpServer::new(move || {
        App::new().app_data(web::Data::new(pool.clone())).service(
            web::resource("/items")
                .route(web::post().to(create_item))
                .route(web::get().to(get_items)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

