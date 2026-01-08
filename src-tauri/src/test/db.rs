use crate::commands::message::greet_logic;
use crate::models::game::Game;
use sqlx::sqlite::SqlitePool;

#[tokio::test]
async fn test_greet_external() {
    let pool = SqlitePool::connect("sqlite://testing.db").await.expect("failed to connect");
    let result = greet_logic(&pool, "Universe");
    assert_eq!(result.await.unwrap(), "Hello, Universe! Found game in DB: Monster Hunter (Year: 2004)");
}

#[tokio::test]
async fn init_db_connection() {
    let pool = SqlitePool::connect("sqlite://testing.db").await;
    assert!(pool.is_ok());
}

#[tokio::test]
async fn db_write() -> Result<(), Box<dyn std::error::Error>> {
    let pool = SqlitePool::connect("sqlite://testing.db").await?;

    sqlx::query("DROP TABLE IF EXISTS mh_games")
        .execute(&pool)
        .await?;

    sqlx::query(
        r#"CREATE TABLE mh_games (
                  id    INTEGER PRIMARY KEY AUTOINCREMENT,
                  name  TEXT NOT NULL,
                  year  INTEGER NOT NULL
                  )"#,
    )
    .execute(&pool)
    .await?;

    let insert_query = "INSERT INTO mh_games (name, year) VALUES ($1, $2)";
    let games_data = vec![
        ("Monster Hunter", 2004),
        ("Monster Hunter Freedom", 2005),
        ("Monster Hunter Tri", 2009),
        ("Monster Hunter: World", 2018),
    ];

    for (name, year) in games_data {
        sqlx::query(insert_query)
            .bind(name)
            .bind(year)
            .execute(&pool)
            .await?;
    }


    let games: Vec<Game> = sqlx::query_as("SELECT * FROM mh_games ORDER BY year")
        .fetch_all(&pool)
        .await?;

    assert_eq!(games.len(), 4);
    assert_eq!(games[0].name, "Monster Hunter");
    assert_eq!(games[0].year, 2004);
    assert_eq!(games[3].name, "Monster Hunter: World");
    assert_eq!(games[3].year, 2018);

    Ok(())
}