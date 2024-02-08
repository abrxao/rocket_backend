mod api;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::product_api::{
    create_product, delete_product, get_all_products, get_product, update_product,
}; //import the handler here
use api::user_api::{create_user, delete_user, get_all_users, get_user, update_user}; //import the handler here
use repository::mongo_repo::MongoRepo;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000/"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods: vec![Method::Get, Method::Post, Method::Put, Method::Delete]
            .into_iter()
            .map(From::from)
            .collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept", "Content-Type"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors();
    let db = MongoRepo::init();

    rocket::build()
        .manage(db)
        .mount("/", routes![create_user])
        .mount("/", routes![get_user])
        .mount("/", routes![update_user])
        .mount("/", routes![get_all_users])
        .mount("/", routes![delete_user])
        .mount("/", routes![delete_product])
        .mount("/", routes![create_product])
        .mount("/", routes![get_product])
        .mount("/", routes![update_product])
        .mount("/", routes![get_all_products])
        .attach(cors.unwrap())
}
