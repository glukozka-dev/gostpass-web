use serde::{Deserialize, Serialize};
use sqlx::{FromRow};

#[derive(Serialize,Deserialize )]
pub struct User{
    pub login: String,
    pub password: String,
    pub team_id: i64,
    pub pubkey: String,
    pub privkey: String,
    pub team_master_key: String,
}

#[derive(Serialize,Deserialize )]
pub struct TableUsers{
    pub id: i64,
    pub login: String,
    pub password: String,
    pub team_id: i64,
    pub pubkey: String,
    pub privkey: String,
    pub team_master_key: String,
}

#[derive(Serialize,Deserialize )]
pub struct TableTeams{
    pub id: i64,
    pub name: String,
    pub masterkey: String,
}

#[derive(Serialize,Deserialize,FromRow)]

pub struct TableBlobs{
    pub id: i64,
    pub secret_type: String,
    pub id_owner: i64,
    pub blob: String,
}

#[derive(Serialize,Deserialize )]
pub struct Response{
    pub res_type: String,
    pub res_details: String,
}

#[derive(Serialize,Deserialize )]
pub struct CreateUser{
    pub auth_token: String,
    pub login: String,
    pub password: String,
}