use serde_derive::{Deserialize, Serialize};
use rusqlite::Connection;
use std::sync::Arc;
use tokio::sync::Mutex;

pub const RESERVED_NICKNAMES: [&str; 3] = ["register", "login", "whoami"];
pub type Database = Arc<Mutex<Connection>>;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Player {
    pub nickname: String,
    pub email: String,
    pub password_hash: String,
    pub has_avatar: bool,
    pub rating: Option<u32>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PlayerUpdateForm {
    pub email: Option<String>,
    pub password_hash: Option<String>,
    pub has_avatar: bool,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct WhoAmIJSON {
    pub nickname: String,
    pub has_avatar: bool,
}

pub fn open_db(name_db: &str) -> Database {
    let db = Connection::open(name_db).unwrap();
    let db = Arc::new(Mutex::new(db));
    db
}