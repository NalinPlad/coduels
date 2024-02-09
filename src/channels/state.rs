use std::collections::{HashMap, VecDeque};
use serde::Serialize;
use tokio::sync::RwLock;


/// Role of a session object. The user is either a owner(who can change lobby rules) or a member(who can just participate in games)
#[derive(Serialize, Clone, Debug)]
pub enum RoomRole {
    Owner,
    Member
}

#[derive(Serialize, Clone, Debug)]
pub struct Session {
    pub name: String,
    pub token: String,
    pub role: RoomRole,
}

pub type RoomStore = HashMap<String, VecDeque<Session>>;


// Sessions stored like this
/*

    Session Store --------
        Room Store --------
            Room a ---
                Session 1 Name: nms token: abc role: owner
                Session 2 Name: player token: bca role: Member
            Room b ---
                Session 1 Name: nms token: abc role: owner
                Session 2 Name: player token: bca role: Member


*/

#[derive(Default)]
pub struct SessionStore {
    pub sessions: RwLock<RoomStore>
}

impl SessionStore {
    pub async fn new(&self, room: &str, session: Session){
        let mut binding = self.sessions.write().await;
        let sessions = binding.entry(room.to_owned()).or_default();
        sessions.push_front(session);
        // sessions.truncate(20);
    }

    // Todo, call when client stop sending health packets(indicating that they closed tab or something), when the delete lobby, or when they close the tab(detect client side)
    pub async fn delete(){
        todo!()
    }
}