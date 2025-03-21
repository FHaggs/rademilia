
// use super::ke
struct RoutingTable {
    local_id: NodeID,
    buckets: Vec<KBucket>, // One bucket per distance range
}
