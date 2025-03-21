extern crate kademlia_dht;

use kademlia_dht::node::NodeInfo;



fn main() {
    let my_node = NodeInfo::new(String::from("127.0.0.1"),  9001);
    let my_node_2 = NodeInfo::new(String::from("127.0.0.1"),  9002);

    let d = my_node.distance(&my_node_2);
    

    print!("{:?}", my_node);
}
