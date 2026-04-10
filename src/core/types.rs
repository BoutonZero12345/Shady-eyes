use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum Role {
    System,
    User,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Message {
    pub role: Role,
    pub content: String,
}