use axum::{ http::StatusCode, Json };
use crate::models::*;
use crate::crypto::*;

pub async fn create_user( Json(payload): Json<CreateUser> ) ->  (StatusCode, Json<Response>) {
    match auth_check(payload.auth_token){
        Ok(()) => {
            match create_user_keys() {
                Ok([pubkey, privkey]) => {
                    let user = User {
                        login: payload.login,
                        password: payload.password,
                        team_id: 0,
                        pubkey: pubkey,
                        privkey: privkey,
                        team_master_key: "None".to_string()
                    };

                    let response = Response {
                        res_type: "Ok".to_string(),
                        res_details: "None".to_string(),
                    };

                    return (StatusCode::CREATED, Json(response))
                }
                Err(e) => {
                    let error_response= Response { res_type: "Failed to create async keys".to_string(), res_details: e.to_string() };
                    return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
                }
            }
        }
        Err(e) => {
            let error_response= Response { res_type: "Wrong token".to_string(), res_details: e.to_string() };
            return (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
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