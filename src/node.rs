
use super::key::Key;
use std::cmp::Ordering;

#[derive(Debug, PartialEq, PartialOrd)]
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
    fn distance(&self, other: &NodeInfo) -> Key{
        self.id.distance(&other.id)
    }
}

impl Ord for NodeInfo {
    fn cmp(&self, other: &Self) -> Ordering {
        let distance_self = self.distance(other);
        let distance_other = other.distance(self);

        // Compare the two distances
        distance_self.cmp(&distance_other)
    }
}

impl Eq for NodeInfo {}