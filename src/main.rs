/**
 * Vector implementation with vec and hashmap of binary merkle tree
 */

use std::collections::HashMap;

#[derive(Debug, Clone)]
struct Node {
    hash: String,
    value: String,
}

#[derive(Debug, Clone)]
struct Tree {
    nodes: Vec<Node>,
    leaf_nodes: u128,
}

impl Node {
    pub fn get_hash(self) -> String {
        return self.hash;
    }

    pub fn get_value(self) -> String {
        return self.value;
    }
}

impl Tree {

    pub fn insert(&mut self, node: Node) {
        self.nodes.push(node);
        self.leaf_nodes = self.leaf_nodes + 1;

        // create a hash 

        let new_hash = Node {hash: "hash".to_string(), value: "val".to_string()};
        self.nodes.push(new_hash);
    }

    pub fn delete(&mut self, key: String){
        let v = &mut self.nodes;

        // find the node to remove
        for i in 0..v.len() - 1 {
            if v[i].hash == key {
                v.swap_remove(i);
            }
        }

        // new remove hashes   

    }

    pub fn calculate_length(mut self) -> usize{

        let v = &mut self.nodes;

        let mut len = 0; 
        for i in 0..v.len(){
            len += i;
        }
        return len;

    }

   
}

fn main() {
    let root = Node {hash: "teshhash".to_string(), value: "test value".to_string()};
    let merkle_nodes = Vec::new();

    let mut MerkleTree = Tree {leaf_nodes: 0, nodes: merkle_nodes};
    MerkleTree.insert(root);
    let new_leaf = Node {hash: "teshhashtwo".to_string(), value: "tesvaltwo".to_string()};
    MerkleTree.insert(new_leaf);

    println!("{:?}", MerkleTree);

    MerkleTree.delete("teshhashtwo".to_string());

    println!("{:?}", MerkleTree);

    println!("{}", MerkleTree.calculate_length());

}
