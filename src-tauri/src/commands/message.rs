use crate::models::game::Game;
use sqlx::sqlite::SqlitePool;
use tauri::path::BaseDirectory;
use tauri::Manager;

// Will be in db folder once proper 
pub async fn greet_logic(pool: &SqlitePool, name: &str) -> Result<String, String> {
    let game: Option<Game> = sqlx::query_as("SELECT * FROM mh_games LIMIT 1")
        .fetch_optional(pool)
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    match game {
        Some(g) => Ok(format!(
            "Hello, {}! Found game in DB: {} (Year: {})",
            name, g.name, g.year
        )),
        None => Ok(format!("Hello, {}! Connected to DB, but no games found.", name)),
    }
}

#[tauri::command]
pub async fn greet(app: tauri::AppHandle, name: &str) -> Result<String, String> {
    let resource_path = app.path().resolve("database/testing.db", BaseDirectory::Resource)
        .map_err(|e| format!("Failed to resolve resource path: {}", e))?;

    let connection_string = format!("sqlite://{}", resource_path.to_string_lossy());

    let pool = SqlitePool::connect(&connection_string)
        .await
        .map_err(|e| format!("Failed to connect to DB: {}", e))?;

    greet_logic(&pool, name).await
}