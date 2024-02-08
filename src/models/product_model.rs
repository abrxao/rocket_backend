use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProductModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub price: f64,
    pub weight: f64, //em kilogramas
    pub categories: Vec<String>,
    pub description: String,
    pub quantity: i32,
    pub image: String,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub deleted_at: Option<DateTime>,
    pub status: bool,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct ProductModelJsonRequest {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub price: f64,
    pub weight: f64, //em kilogramas
    pub categories: Vec<String>,
    pub description: String,
    pub quantity: i32,
    pub image: String,
    pub created_at: Option<String>,
    pub updated_at: Option<String>,
    pub deleted_at: Option<String>,
    pub status: bool,
}
