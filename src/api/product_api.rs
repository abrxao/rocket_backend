use crate::{
    models::product_model::{ProductModel, ProductModelJsonRequest},
    repository::mongo_repo::MongoRepo,
};
use mongodb::{
    bson::{oid::ObjectId, DateTime},
    results::InsertOneResult,
};
use rocket::{http::Status, serde::json::Json, State};

#[post("/product", data = "<new_product>")]
pub fn create_product(
    db: &State<MongoRepo>,
    new_product: Json<ProductModelJsonRequest>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = ProductModel {
        id: None,
        name: new_product.name.to_owned(),
        price: new_product.price.to_owned(),
        weight: new_product.weight.to_owned(),
        categories: new_product.categories.to_owned(),
        description: new_product.description.to_owned(),
        quantity: new_product.quantity.to_owned(),
        image: new_product.image.to_owned(),
        created_at: DateTime::now(),
        updated_at: None,
        deleted_at: None,
        status: true,
    };
    let product_detail = db.create_product(data);
    match product_detail {
        Ok(product) => Ok(Json(product)),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[get("/products")]
pub fn get_all_products(db: &State<MongoRepo>) -> Result<Json<Vec<ProductModel>>, Status> {
    let users = db.get_all_products();
    match users {
        Ok(users_vec) => Ok(Json(users_vec)),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[put("/product/<path>", data = "<new_product_data>")]
pub fn update_product(
    db: &State<MongoRepo>,
    path: String,
    new_product_data: Json<ProductModelJsonRequest>,
) -> Result<Json<ProductModel>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let data = ProductModel {
        id: Some(ObjectId::parse_str(&id).unwrap()),
        name: new_product_data.name.to_owned(),
        price: new_product_data.price.to_owned(),
        weight: new_product_data.weight.to_owned(),
        categories: new_product_data.categories.to_owned(),
        description: new_product_data.description.to_owned(),
        quantity: new_product_data.quantity.to_owned(),
        image: new_product_data.image.to_owned(),
        created_at: DateTime::parse_rfc3339_str(new_product_data.created_at.to_owned().unwrap())
            .unwrap(),
        updated_at: Some(DateTime::now()),
        deleted_at: None,
        status: new_product_data.status.to_owned(),
    };
    let update_result = db.update_product(&id, data);
    match update_result {
        Ok(update) => {
            if update.matched_count == 1 {
                let updated_product_info = db.get_product(&id);
                return match updated_product_info {
                    Ok(product) => Ok(Json(product)),
                    Err(_) => Err(Status::InternalServerError),
                };
            } else {
                return Err(Status::NotFound);
            }
        }
        Err(_) => Err(Status::InternalServerError),
    }
}
#[get("/product/<path>")]
pub fn get_product(db: &State<MongoRepo>, path: String) -> Result<Json<ProductModel>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let user_detail = db.get_product(&id);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::InternalServerError),
    }
}
#[delete("/product/<path>")]
pub fn delete_product(db: &State<MongoRepo>, path: String) -> Result<Json<ProductModel>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let delete_result = db.delete_product(&id);
    match delete_result {
        Ok(delete) => Ok(Json(delete.unwrap())),
        Err(_) => Err(Status::InternalServerError),
    }
}
