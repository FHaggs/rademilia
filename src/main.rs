mod kbucket;
mod node;

fn main() {
    let my_node = node::NodeInfo {
        ip: String::from("128.0.0.1"),
        port: 8080,
        id: 120,
    };
    let my_kbucket = kbucket::KBucket {
        nodes: vec![my_node],
    };
    println!("{:?}", my_kbucket);
}
