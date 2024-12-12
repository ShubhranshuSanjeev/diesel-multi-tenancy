mod db;
mod diesel_ext;

use actix_web::web::{Data, Json, Path};
use actix_web::{web, App, HttpServer};
use db::models::{Item, NewItem};
use db::schema::items;
use diesel::QueryDsl;
use diesel::r2d2::{self, ConnectionManager};
use diesel::{PgConnection, RunQueryDsl};
use diesel::ExpressionMethods;
use diesel::SelectableHelper;
use dotenv::dotenv;
use std::env;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

async fn get_items(pool: Data<DbPool>) -> actix_web::Result<Json<Vec<Item>>> {
    let mut conn = pool.get().unwrap();
    let items = items::table
        .schema_name(&String::from("test"))
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
        .returning(Item::as_returning())
        .schema_name(&String::from("test"))
        .get_result(&mut conn)
        .map_err(|e| e.to_string())
        .unwrap();
    Ok(Json(inserted_item))
}

async fn update_item(pool: Data<DbPool>, id: Path<i32>, item: Json<NewItem>) -> actix_web::Result<Json<Item>> {
    let mut conn = pool.get().unwrap();
    let item = item.into_inner();

    let updated_item: Item = diesel::update(items::dsl::items.find(id.into_inner()))
        .set((
            items::description.eq(item.description),
        ))
        .returning(Item::as_returning())
        .schema_name(&String::from("test"))
        .get_result(&mut conn)
        .map_err(|e| e.to_string())
        .unwrap();
    Ok(Json(updated_item))
}

async fn upsert_item(pool: Data<DbPool>, item: Json<Item>) -> actix_web::Result<Json<Item>> {
    let mut conn = pool.get().unwrap();
    let item = item.into_inner();

    let updated_item: Item = diesel::insert_into(items::table)
        .values(&item)
        .on_conflict(items::id)
        .do_update()
        .set(&item)
        .returning(Item::as_returning())
        .schema_name(&String::from("test"))
        .get_result(&mut conn)
        .map_err(|e| e.to_string())
        .unwrap();
    Ok(Json(updated_item))
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
        App::new().app_data(web::Data::new(pool.clone()))
            .service(
                web::resource("/items/{id}")
                    .route(web::patch().to(update_item))
            )
            .service(
                web::resource("/items")
                    .route(web::put().to(upsert_item))
                    .route(web::post().to(create_item))
                    .route(web::get().to(get_items)),
            )
    })
    .bind(("127.0.0.1", 8089))?
    .run()
    .await
}
