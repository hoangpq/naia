
use std::{
    net::SocketAddr,
};
use crate::gaia_server::Timestamp;

new_key_type! { pub struct UserKey; }

pub struct User {
    pub address: SocketAddr,
    pub timestamp: Timestamp,
}

impl User {
    pub fn new(address: SocketAddr, timestamp: Timestamp) -> User {
        User {
            address,
            timestamp,
        }
    }
}