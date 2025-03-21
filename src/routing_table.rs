use super::kbucket::KBucket;
use super::key::Key;
use crate::node::NodeInfo;
use std::collections::HashMap;
use super::K;


#[derive(Debug)]
pub struct RoutingTable {
    pub buckets: HashMap<Key, KBucket>,
}

impl RoutingTable {
    pub fn new() -> Self {
        RoutingTable {
            buckets: HashMap::new(),
        }
    }
    pub fn get_closest_nodes(&self, key: &Key, count: usize) -> Vec<(&NodeInfo, Key)> {
        todo!();
    }
    pub fn remove(&mut self, node: &NodeInfo) {
        todo!();
    }
    fn lookup_bucket_index(&self, key: &Key) -> usize {
        todo!();
    }

}
