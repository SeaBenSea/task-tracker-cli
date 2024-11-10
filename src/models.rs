use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Task {
    pub id: u32,
    pub description: String,
    pub status: Status,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub enum Status {
    Todo,
    InProgress,
    Done,
}
