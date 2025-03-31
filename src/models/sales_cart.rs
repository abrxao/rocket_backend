use mongodb::bson::oid::ObjectId;
use mongodb::bson::DateTime;
use product_model::ProductModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct SalesCart {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub saler_id: ObjectId,
    pub products: Vec<ProductModel>,
    //pub status: ,
}

enum SalesCartStatus {
    Closed,
    Open,
}

pub struct SelectedProducts {}
