
use super::key::Key;

#[derive(Debug)]
pub struct NodeInfo {
    pub ip: String,
    pub port: u16,
    pub id: Key,
}

impl NodeInfo {
    pub fn new(ip: String, port: u16) -> Self{
        let addr = format!("{}:{}", ip, port);
        let id = Key::new(addr);

        NodeInfo { ip, port, id }
    }
    pub fn get_address(&self) -> String {
        format!("{}:{}", self.ip, self.port)
    }
    pub fn distance(&self, other: &NodeInfo) -> Key{
        self.id.distance(&other.id)
    }
}

