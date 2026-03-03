use serde::{Deserialize, Serialize};
#[derive(Serialize,Deserialize )]
pub struct User{
    pub login: String,
    pub password: String,
    pub team_id: u64,
    pub pubkey: String,
    pub privkey: String,
    pub team_master_key: String,
}

#[derive(Serialize,Deserialize )]
pub struct Table_user{
    pub id: u64,
    pub login: String,
    pub password: String,
    pub team_id: u64,
    pub pubkey: String,
    pub privkey: String,
    pub team_master_key: String,
}

#[derive(Serialize,Deserialize )]
pub struct Table_team{
    pub id: u64,
    pub name: String,
    pub masterkey: String,
}

#[derive(Serialize,Deserialize )]

pub struct Table_blob{
    pub id: u64,
    pub secret_type: String,
    pub id_owner: u64,
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