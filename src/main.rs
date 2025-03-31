mod api;
mod lib;
mod models;
mod repository;

#[macro_use]
extern crate rocket;

use api::{
    product_api::{
        create_product, delete_many_products, delete_product, get_all_products, get_product,
        update_product,
    },
    user_api::{login, register_user},
}; //import the handler here
use repository::mongo_repo::MongoRepo;
use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[launch]
fn rocket() -> _ {
    let allowed_origins = AllowedOrigins::some_exact(&["http://localhost:3000/"]);
    let allowed_methods = vec![Method::Get, Method::Post, Method::Put, Method::Delete]
        .into_iter()
        .map(From::from)
        .collect();
    let allowed_headers =
        AllowedHeaders::some(&["Authorization", "Accept", "Content-Type", "token"]);

    let cors = rocket_cors::CorsOptions {
        allowed_origins,
        allowed_methods,
        allowed_headers,
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors();

    let db = MongoRepo::init();

    rocket::build()
        .manage(db)
        .mount(
            "/",
            routes![
                delete_product,
                create_product,
                get_product,
                get_all_products,
                update_product,
                delete_many_products
            ],
        )
        .mount("/", routes![register_user, login])
        .attach(cors.unwrap())
}
