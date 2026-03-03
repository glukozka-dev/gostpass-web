use sqlx::{SqlitePool, sqlite::SqlitePoolOptions, FromRow};
use serde::{Serialize, Deserialize};
use std::path::Path;
const DB_URL: &str = "sqlite://database.db";

// Функция инициализации базы данных
pub async fn init_db() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(DB_URL)
        .await?;
    
    println!("Connected to database");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            login TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL,
            team_id INTEGER NOT NULL DEFAULT 0,
            pubkey TEXT NOT NULL,
            privkey TEXT NOT NULL,
            team_master_key TEXT NOT NULL
        )
        "#
    )
    .execute(&pool)
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
    .execute(&pool)
    .await?;
    println!("Table teams created");

    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS blobs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            secret_type TEXT NOT NULL,
            id_owner INTEGER NOT NULL,
            blob TEXT NOT NULL,
        )
        "#
    )
    .execute(&pool)
    .await?;
    println!("Table blobs created");

    println!("Database initialized");
    
    Ok(())
}