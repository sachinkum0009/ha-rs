/**
Utils for the project
 */

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity {
    pub entity_id: String,
    pub state: String,
    pub attributes: HashMap<String, serde_json::Value>,
    pub last_changed: String,
    pub last_reported: String,
    pub last_updated: String,
    pub context: Context,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Context {
    pub id: String,
    pub parent_id: Option<String>,
    pub user_id: Option<String>,
}