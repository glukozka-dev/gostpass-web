use sqlx::{SqlitePool};
use anyhow::Result;
use crate::models::*;
const PERSONAL_TYPE:  &str = "PERSONAL";
const TEAM_TYPE:  &str = "TEAM";


pub async fn init_db(pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            login TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            privkey TEXT NOT NULL,
            pubkey TEXT NOT NULL,
            team_id INTEGER NOT NULL DEFAULT 0,
            team_master_key TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;
    println!("Table users created");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS teams (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            name TEXT NOT NULL UNIQUE,
            masterkey TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;
    println!("Table teams created");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS blobs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            secret_type TEXT NOT NULL,
            id_owner INTEGER NOT NULL,
            blob TEXT NOT NULL
        )
        "#
    )
    .execute(pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO teams (name, masterkey) VALUES (?,?)
        "#
    )
    .bind("NOTEAM")
    .bind("None")
    .execute(pool)
    .await?;

    println!("Table blobs created");

    println!("Database initialized");
    
    Ok(())
}

pub async fn create_user(user: User, pool: &SqlitePool) -> Result<()> {
    sqlx::query(
        "INSERT INTO users (login, password, privkey, pubkey, team_id, team_master_key) VALUES (?, ?, ?, ?, ?, ?)"
    )
    .bind(user.login)
    .bind(user.password)
    .bind(user.privkey)
    .bind(user.pubkey)
    .bind(user.team_id)
    .bind(user.team_master_key)
    .execute(pool)
    .await?;
    Ok(())

}

pub async fn delete_user(login: String, pool: &SqlitePool) -> Result<()>  {
    let user_id: i32 = get_user_id(&login, pool).await?;

    sqlx::query(
        "DELETE FROM users WHERE login = ?"
    )
    .bind(login)
    .execute(pool)
    .await?;

    sqlx::query(
        "DELETE FROM blobs WHERE id_owner = ? AND secret_type = ?"
    )
    .bind(user_id)
    .bind(PERSONAL_TYPE)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_team(team_name: String, master_key: String, pool: &SqlitePool) -> Result<()>  {
    sqlx::query(
        "INSERT INTO teams (name, masterkey) VALUES (?, ?)"
    )
    .bind(team_name)
    .bind(master_key)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn delete_team(team_name: String, pool: &SqlitePool) -> Result<()>  {
    let team_id: i32 = get_team_id(&team_name, pool).await?;
    sqlx::query(
        "DELETE FROM teams WHERE name = ?"
    )
    .bind(team_name)
    .execute(pool)
    .await?;

    sqlx::query(
        "DELETE FROM blobs WHERE id_owner = ? AND secret_type = ?"
    )
    .bind(team_id)
    .bind(TEAM_TYPE)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn clear_db() -> Result<()>  {
    Ok(())
}

pub async fn create_personal_secret(login: String, secret: String, pool: &SqlitePool) -> Result<()>  {
    let user_id: i32 = get_user_id(&login, pool).await?;

    sqlx::query(
        "INSERT INTO blobs (secret_type, id_owner, blob) VALUES (?,?,?)"
    )
    .bind(PERSONAL_TYPE)
    .bind(user_id)
    .bind(secret)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn create_team_secret(team_name: String, secret: String, pool: &SqlitePool) -> Result<()>  {
    let team_id: i32 = get_team_id(&team_name, pool).await?;

    sqlx::query(
        "INSERT INTO blobs (secret_type, id_owner, blob) VALUES (?,?,?)"
    )
    .bind(TEAM_TYPE)
    .bind(team_id)
    .bind(secret)
    .execute(pool)
    .await?;
    Ok(())
}

pub async fn get_user_pubkey(login: String, pool: &SqlitePool) -> Result<String> {
    let pubkey: String = sqlx::query_scalar(
        "SELECT pubkey FROM users WHERE login = ?"
    )
    .bind(login)
    .fetch_one(pool)
    .await?;

    Ok(pubkey)
}

pub async fn add_to_team(team_name: String, master_key: String, login: String, pool: &SqlitePool) -> Result<()>  {
    let team_id: i32 = get_team_id(&team_name, pool).await?;
    sqlx::query(
        "UPDATE users SET team_id = ?, team_master_key = ? WHERE login = ?"
    )
    .bind(team_id)
    .bind(master_key)
    .bind(login)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn delete_from_team(login: String, pool: &SqlitePool) -> Result<()>  {
    sqlx::query(
        "UPDATE users SET team_id = 0, team_master_key = ? WHERE login = ?"
    )
    .bind("None")
    .bind(login)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_personal_secrets(login: String, pool: &SqlitePool) -> Result<Vec<TableBlobs>>  {
    let user_id: i32 = get_user_id(&login, pool).await?;
    let all_secrets: Vec<TableBlobs> = sqlx::query_as::<_, TableBlobs>(
        "SELECT * FROM blobs WHERE id_owner = ? AND secret_type = ?"
    )
    .bind(user_id)
    .bind(PERSONAL_TYPE)
    .fetch_all(pool)
    .await?;
    Ok(all_secrets)
}

pub async fn get_team_secrets(login: String, pool: &SqlitePool) -> Result<Vec<TableBlobs>>  {
    let team_id: i32 = sqlx::query_scalar(
        "SELECT team_id FROM users WHERE login = ?"
    )
    .bind(login)
    .fetch_one(pool)
    .await?;

    let all_secrets: Vec<TableBlobs> = sqlx::query_as::<_, TableBlobs>(
        "SELECT * FROM blobs WHERE id_owner = ? AND secret_type = ?"
    )
    .bind(team_id)
    .bind(TEAM_TYPE)
    .fetch_all(pool)
    .await?;
    Ok(all_secrets)
}


// Support funcs

async fn get_team_id(team_name: &String, pool: &SqlitePool) -> Result<i32> {
    let team_id: i32 = sqlx::query_scalar(
        "SELECT id FROM teams WHERE name = ?"
    )
    .bind(team_name)
    .fetch_one(pool)
    .await?;

    Ok(team_id)
}

async fn get_user_id(login: &String, pool: &SqlitePool) -> Result<i32> {
    let user_id: i32 = sqlx::query_scalar(
        "SELECT id FROM users WHERE login = ?"
    )
    .bind(login)
    .fetch_one(pool)
    .await?;

    Ok(user_id)
}
