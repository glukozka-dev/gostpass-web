use axum::response::IntoResponse;
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
        acc_type: payload.acc_type,
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

pub async fn delete_user(State(pool): State<SqlitePool>, Json(payload): Json<DeleteUser> ) ->  (StatusCode, Json<Response>){
    if let Err(e) = adm_auth_check(payload.auth_token) {
        let error_response = Response {
            res_type: "Wrong token".to_string(),
            res_details: e.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(error_response));
    }
    match database::delete_user(payload.login, &pool).await{
        Ok(()) => {
            let response = Response {
                res_type: "Ok".to_string(),
                res_details: "None".to_string(),
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error_response = Response {
                res_type: "Failed to delete user in database".to_string(),
                res_details: e.to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

pub async fn create_team(State(pool): State<SqlitePool>, Json(payload): Json<CreateTeam> ) ->  (StatusCode, Json<Response>){
    if let Err(e) = auth_check(payload.auth_token) {
        let error_response = Response {
            res_type: "Wrong token".to_string(),
            res_details: e.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(error_response));
    }
    match database::create_team(payload.name, payload.masterkey, payload.owner_login , &pool).await{
        Ok(()) => {
            let response = Response {
                res_type: "Ok".to_string(),
                res_details: "None".to_string(),
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            let error_response = Response {
                res_type: "Failed to create team in database".to_string(),
                res_details: e.to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

pub async fn delete_team(State(pool): State<SqlitePool>, Json(payload): Json<DeleteTeam> ) ->  (StatusCode, Json<Response>){
    if let Err(e) = auth_check(payload.auth_token) {
        let error_response = Response {
            res_type: "Wrong token".to_string(),
            res_details: e.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(error_response));
    }

    match database::delete_team(payload.name, &pool).await{
        Ok(()) => {
            let response = Response {
                res_type: "Ok".to_string(),
                res_details: "None".to_string(),
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error_response = Response {
                res_type: "Failed to delete team in database".to_string(),
                res_details: e.to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }

}

pub async fn clear_db(State(pool): State<SqlitePool>, Json(payload): Json<CreateUser> ) ->  (StatusCode, Json<Response>){
    if let Err(e) = auth_check(payload.auth_token) {
        let error_response = Response {
            res_type: "Wrong token".to_string(),
            res_details: e.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(error_response));
    }
    unimplemented!()
}

pub async fn create_personal_secret(State(pool): State<SqlitePool>, Json(payload): Json<CreatePersonalSecret> ) ->  (StatusCode, Json<Response>){
    if let Err(e) = auth_check(payload.auth_token) {
        let error_response = Response {
            res_type: "Wrong token".to_string(),
            res_details: e.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(error_response));
    }

    match database::create_personal_secret(payload.login, payload.secret, &pool).await{
        Ok(()) => {
            let response = Response {
                res_type: "Ok".to_string(),
                res_details: "None".to_string(),
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            let error_response = Response {
                res_type: "Failed to create personal secret in database".to_string(),
                res_details: e.to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

pub async fn create_team_secret(State(pool): State<SqlitePool>, Json(payload): Json<CreateTeamSecret> ) ->  (StatusCode, Json<Response>){
    if let Err(e) = auth_check(payload.auth_token) {
        let error_response = Response {
            res_type: "Wrong token".to_string(),
            res_details: e.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(error_response));
    }
    match database::create_team_secret(payload.team_name, payload.secret, &pool).await{
        Ok(()) => {
            let response = Response {
                res_type: "Ok".to_string(),
                res_details: "None".to_string(),
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            let error_response = Response {
                res_type: "Failed to create personal secret in database".to_string(),
                res_details: e.to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

pub async fn get_user_pubkey(State(pool): State<SqlitePool>, Json(payload): Json<GetUserPubkey> ) ->  (StatusCode, Json<Response>){
    if let Err(e) = auth_check(payload.auth_token) {
        let error_response = Response {
            res_type: "Wrong token".to_string(),
            res_details: e.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(error_response));
    }

    match database::get_user_pubkey(payload.login, &pool).await{
        Ok(pubkey) => {
            let response = Response {
                res_type: "Pubkey".to_string(),
                res_details: pubkey,
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error_response = Response {
                res_type: "Failed to get pubkey from database".to_string(),
                res_details: e.to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

pub async fn add_to_team(State(pool): State<SqlitePool>, Json(payload): Json<AddUserToTeam> ) ->  (StatusCode, Json<Response>){
    if let Err(e) = auth_check(payload.auth_token) {
        let error_response = Response {
            res_type: "Wrong token".to_string(),
            res_details: e.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(error_response));
    }
    
    match database::add_to_team(payload.team_name, payload.masterkey, payload.login, &pool).await{
        Ok(()) => {
            let response = Response {
                res_type: "Ok".to_string(),
                res_details: "None".to_string(),
            };
            (StatusCode::CREATED, Json(response))
        }
        Err(e) => {
            let error_response = Response {
                res_type: "Failed to add user to team from database".to_string(),
                res_details: e.to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

pub async fn delete_from_team(State(pool): State<SqlitePool>, Json(payload): Json<DeleteFromTeam> ) ->  (StatusCode, Json<Response>){
    if let Err(e) = auth_check(payload.auth_token) {
        let error_response = Response {
            res_type: "Wrong token".to_string(),
            res_details: e.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(error_response));
    }
    
    match database::delete_from_team(payload.login, &pool).await{
        Ok(()) => {
            let response = Response {
                res_type: "Ok".to_string(),
                res_details: "None".to_string(),
            };
            (StatusCode::OK, Json(response))
        }
        Err(e) => {
            let error_response = Response {
                res_type: "Failed to delete user from team from database".to_string(),
                res_details: e.to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response))
        }
    }
}

pub async fn get_personal_secrets(State(pool): State<SqlitePool>, Json(payload): Json<GetPersonalSecrets> ) ->  impl IntoResponse {
    if let Err(e) = auth_check(payload.auth_token) {
        let error_response = Response {
            res_type: "Wrong token".to_string(),
            res_details: e.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(error_response)).into_response();
    }
    match database::get_personal_secrets(payload.login, &pool).await{
        Ok(secrets) => {
            (StatusCode::OK, Json(secrets)).into_response()
        }
        Err(e ) => {
            let error_response = Response {
                res_type: "Failed to get personal secrets from database".to_string(),
                res_details: e.to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}

pub async fn get_team_secrets(State(pool): State<SqlitePool>, Json(payload): Json<GetTeamSecrets> ) ->  impl IntoResponse{
    if let Err(e) = auth_check(payload.auth_token) {
        let error_response = Response {
            res_type: "Wrong token".to_string(),
            res_details: e.to_string(),
        };
        return (StatusCode::UNAUTHORIZED, Json(error_response)).into_response();
    }
    match database::get_team_secrets(payload.login, &pool).await{
        Ok(secrets) => {
            (StatusCode::OK, Json(secrets)).into_response()
        }
        Err(e ) => {
            let error_response = Response {
                res_type: "Failed to get team secrets from database".to_string(),
                res_details: e.to_string(),
            };
            (StatusCode::INTERNAL_SERVER_ERROR, Json(error_response)).into_response()
        }
    }
}