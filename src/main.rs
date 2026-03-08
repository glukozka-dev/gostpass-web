mod handlers;
mod models;
mod crypto;
mod database;
use axum::{ routing::post, Router,};
use sqlx::sqlite::SqlitePoolOptions;
use tokio::fs::File;
use std::path::Path;
const DB_URL: &str = "sqlite://database.db";


#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    
    let pool = if !Path::new("database.db").exists() {
        File::create("database.db").await.unwrap();
        println!("Created new database file");
        let pool = SqlitePoolOptions::new()
            .max_connections(5)
            .connect(DB_URL)
            .await
            .unwrap();
        database::init_db(&pool).await.unwrap();
        pool
    } else {
        SqlitePoolOptions::new()
            .max_connections(5)
            .connect(DB_URL)
            .await
            .unwrap() 
    };
    println!("Pool is ready to use");
    
    // build our application with a route
    let app = Router::new()
        .route("/create_user", post(handlers::create_user))
        .route("/delete_user", post(handlers::delete_user))
        .route("/create_team", post(handlers::create_team))
        .route("/delete_team", post(handlers::delete_team))
        .route("/clear_db", post(handlers::clear_db))
        .route("/create_personal_secret", post(handlers::create_personal_secret))
        .route("/create_team_secret", post(handlers::create_team_secret))
        .route("/get_user_pubkey", post(handlers::get_user_pubkey))
        .route("/add_to_team", post(handlers::add_to_team))
        .route("/delete_from_team", post(handlers::delete_from_team))
        .route("/get_personal_secrets", post(handlers::get_personal_secrets))
        .route("/get_team_secrets", post(handlers::get_team_secrets))
        .with_state(pool.clone());

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}