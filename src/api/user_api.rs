use crate::lib::users_utils::{encrypt_password, verify_password};
use crate::models::user_model::{LoginUser, User};
use crate::repository::mongo_repo::MongoRepo;
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State};

#[post("/register_user", data = "<new_user>")]
pub fn register_user(
    db: &State<MongoRepo>,
    new_user: Json<LoginUser>,
) -> Result<Json<InsertOneResult>, Status> {
    let password = new_user.password.to_owned();
    if password.len() < 8
        || password.chars().all(char::is_alphabetic)
        || password.chars().all(char::is_numeric)
    {
        return Err(Status::BadRequest);
    }

    let password_hash = encrypt_password(&password);

    let data = User {
        id: None,
        username: new_user.username.to_owned(),
        password_hash: password_hash.to_owned(),
        role: Some("user".to_string()),
    };
    let user_detail = db.register_user(data);
    match user_detail {
        Ok(user) => Ok(Json(user)),
        Err(_) => Err(Status::Conflict),
    }
}

#[post("/login", data = "<login>")]
pub fn login(db: &State<MongoRepo>, login: Json<LoginUser>) -> Result<Json<User>, Status> {
    let login = login.into_inner();
    let user_res = db.login(&login);
    let user = match user_res {
        Ok(user) => user,
        Err(_) => return Err(Status::InternalServerError),
    };

    let is_password_ok = verify_password(&login.password, &user.password_hash);

    if is_password_ok {
        Ok(Json(user))
    } else {
        Err(Status::InternalServerError)
    }
}
