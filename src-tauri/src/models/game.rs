use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow, PartialEq, Eq)]
#[allow(dead_code)]
pub struct Game {
    pub id: i64,
    pub name: String,
    pub year: i64,
}
