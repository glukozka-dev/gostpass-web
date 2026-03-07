use axum::{ http::StatusCode, Json, extract::State};
use sqlx::SqlitePool;
use crate::database;
use crate::models::*;
use crate::crypto::*;

pub async fn create_user( State(pool): State<SqlitePool>, Json(payload): Json<CreateUser> ) ->  (StatusCode, Json<Response>) {

    if let Err(e) = auth_check(payload.auth_token) {
        let error_response = Response {
            res_type: "Wrong token".to_string(),
            res_details: e.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(error_response));
    }


    let [pubkey, privkey] = match create_user_keys() {
        Ok(keys) => keys,
        Err(e) => {
            let error_response = Response {
                res_type: "Failed to create keys".to_string(),
                res_details: e.to_string(),
            };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response));
        }
    };

    let user = User {
        login: payload.login,
        password: payload.password,
        team_id: 0,
        pubkey,
        privkey,
        team_master_key: "None".to_string(),
    };
    match database::create_user(user, &pool).await {
        Ok(()) => {
            let response = Response {
                res_type: "Ok".to_string(),
                res_details: "None".to_string(),
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            let error_response = Response {
                res_type: "Failed to create user in database".to_string(),
                res_details: e.to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

pub async fn delete_user(){
    unimplemented!()
}

pub async fn create_team(){
    unimplemented!()
}

pub async fn delete_team(){
    unimplemented!()
}

pub async fn clear_db(){
    unimplemented!()
}

pub async fn create_personal_secret(){
    unimplemented!()
}

pub async fn create_team_secret(){
    unimplemented!()
}

pub async fn add_to_team(){
    unimplemented!()
}

pub async fn delete_from_team(){
    unimplemented!()
}

pub async fn get_personal_secrets(){
    unimplemented!()
}

pub async fn get_team_secrets(){
    unimplemented!()
}