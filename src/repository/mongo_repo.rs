extern crate dotenv;
use dotenv::dotenv;

use crate::models::{
    product_model::ProductModel,
    user_model::{LoginUser, User},
};
use mongodb::{
    bson::{doc, extjson::de::Error, oid::ObjectId},
    results::{DeleteResult, InsertOneResult, UpdateResult},
    sync::{Client, Database},
};

pub struct MongoRepo {
    db: Database,
}
//Função de inicialização do banco de dados
impl MongoRepo {
    pub fn init() -> Self {
        dotenv().ok();
        let uri = dotenv::var("MONGO_URL").unwrap();
        let client = Client::with_uri_str(uri).unwrap();
        let db_connection = client.database("rustDB");
        MongoRepo { db: db_connection }
    }
}

//Funções de utilização do banco de dados modificações de usuario
impl MongoRepo {
    pub fn register_user(&self, new_user: User) -> Result<InsertOneResult, Error> {
        let filter = doc! {"username": &new_user.username};
        let is_existing_user = self
            .db
            .collection::<User>("users")
            .find_one(filter, None)
            .ok()
            .expect("Error checking if user exists");
        match is_existing_user {
            Some(_) => Err(Error::DeserializationError { message: "User already exists".to_string()}),
            None => {
                let user = self
                    .db
                    .collection::<User>("users")
                    .insert_one(new_user, None)
                    .ok()
                    .expect("Error creating user");
                Ok(user)
            }
        }
    }

    pub fn login(&self, login_user: &LoginUser) -> Result<User, Error> {
        let filter = doc! {"username": &login_user.username};
        let is_existing_user = self
            .db
            .collection::<User>("users")
            .find_one(filter, None)
            .ok()
            .expect("Error checking if user exists");
        match is_existing_user {
            Some(user) => Ok(user),
            None => Err(Error::DeserializationError {
                message: "Usuario não existe".to_string(),
            }),
        }
    }

    /*     pub fn session_token(&self, user_id: &String) -> Result<InsertOneResult, Error> {} */
}
//Funções de utilização do banco de dados para gerenciamento de sessions
impl MongoRepo {}
//Funções de utilização do banco de dados modificações dos produtos
impl MongoRepo {
    pub fn create_product(&self, new_product: ProductModel) -> Result<InsertOneResult, Error> {
        let new_doc = ProductModel {
            id: None,
            name: new_product.name,
            weight: new_product.weight,
            price: new_product.price,
            categories: new_product.categories,
            description: new_product.description,
            quantity: new_product.quantity,
            image: new_product.image,
            created_at: new_product.created_at,
            updated_at: new_product.updated_at,
            deleted_at: new_product.deleted_at,
            status: new_product.status,
        };
        let product = self
            .db
            .collection("products")
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating product");
        Ok(product)
    }
    pub fn get_product(&self, id: &String) -> Result<ProductModel, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let user_detail = self
            .db
            .collection::<ProductModel>("products")
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(user_detail.unwrap())
    }
    pub fn update_product(
        &self,
        id: &String,
        new_product_data: ProductModel,
    ) -> Result<UpdateResult, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let new_doc = doc! {
            "$set":
                {
                   "name": new_product_data.name,
                   "price": new_product_data.price,
                   "categories": new_product_data.categories,
                   "description": new_product_data.description,
                   "weight": new_product_data.weight,
                   "quantity": new_product_data.quantity,
                   "image": new_product_data.image,
                   "created_at": new_product_data.created_at,
                   "updated_at": new_product_data.updated_at,
                   "status": new_product_data.status
                },
        };
        let updated_doc = self
            .db
            .collection::<ProductModel>("products")
            .update_one(filter, new_doc, None)
            .ok()
            .expect("Error updating user");
        Ok(updated_doc)
    }
    pub fn get_all_products(&self) -> Result<Vec<ProductModel>, Error> {
        let users = self
            .db
            .collection("products")
            .find(None, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(users.map(|user| user.unwrap()).collect())
    }
    pub fn delete_product(&self, id: &String) -> Result<Option<ProductModel>, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let deleted_doc = self
            .db
            .collection("products")
            .find_one_and_delete(filter, None)
            .ok()
            .expect("Error deleting product");
        Ok(deleted_doc)
    }
    pub fn delete_many_products(&self, ids: &Vec<String>) -> Result<DeleteResult, Error> {
        let to_delete_ids: Vec<ObjectId> = ids
            .iter()
            .map(|id| ObjectId::parse_str(id).unwrap())
            .collect();
        let filter = doc! {"_id":{"$in": to_delete_ids}};
        let deleted_doc = self
            .db
            .collection::<ProductModel>("products")
            .delete_many(filter, None)
            .ok()
            .expect("Error deleting product");
        Ok(deleted_doc)
    }
}
