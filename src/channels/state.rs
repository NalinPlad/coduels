use std::collections::{HashMap, VecDeque};
use tokio::sync::RwLock;


/// Role of a session object. The user is either a owner(who can change lobby rules) or a member(who can just participate in games)
enum Role {
    Owner,
    Member
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct Session {
    pub name: String,
    pub token: String,
    pub role: Role,
}

pub type SessionStore {
 
}