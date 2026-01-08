use crate::models::game::Game;
use sqlx::sqlite::SqlitePool;

#[tauri::command]
pub async fn greet(name: &str) -> Result<String, String> {
    let pool = SqlitePool::connect("sqlite://testing.db")
        .await
        .map_err(|e| format!("Failed to connect to DB: {}", e))?;

    let game: Option<Game> = sqlx::query_as("SELECT * FROM mh_games LIMIT 1")
        .fetch_optional(&pool)
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