use std::{
    collections::HashMap,
    net::Ipv4Addr,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
pub struct Friends {
    friends: Arc<Mutex<HashMap<String, Ipv4Addr>>>,
}
impl Friends {
    pub fn new(localhost: Ipv4Addr) -> Self {
        let mut friends = HashMap::new();
        friends.insert(String::from("me"), localhost);

        Self {
            friends: Arc::new(Mutex::new(friends)),
        }
    }

    pub fn parse_address(&self, addr: &str) -> Option<Ipv4Addr> {
        match addr.parse::<Ipv4Addr>() {
            Ok(addr) => {
                return Some(addr);
            }
            Err(_) => {
                let friends = self.friends.lock().expect("Failed to get friends lock");
                if let Some(addr) = friends.get(addr) {
                    return Some(*addr);
                }
            }
        }
        None
    }

    pub fn new_friend(&mut self, name: &str, addr: &str) -> Option<Ipv4Addr> {
        if let Some(addr) = self.parse_address(addr) {
            let mut friends = self.friends.lock().expect("Failed to get friends  lock");
            friends.insert(String::from(name), addr);
            return Some(addr);
        } else {
            return None;
        }
    }

    pub fn lookup_addr(&self, addr: Ipv4Addr) -> Option<String> {
        let friends = self.friends.lock().expect("Failed to get friends lock");
        if let Some((name, _)) = friends.iter().find(|(_, a)| **a == addr) {
            return Some(name.clone());
        }
        return None;
    }
}
