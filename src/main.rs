// note: Rust will not dynamically allocate memory for the node at runtime, must wrap it / be explicit
// note: use SHA-512 for hash of val
// key is hash of the JSON string of the entry and value is the JSON string of the entry.

// -------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------
// ---------------------------------START-----------------------------------------------

//use sha2::{Sha512, Digest}; // use sha2 crate 
extern crate crypto;
#[macro_use] extern crate hex_literal;
use self::crypto::digest::Digest;
use self::crypto::sha2::Sha512; // use crypto lib for this bc of string implementation
use std::str;

// root hash, i think this is necessary


// define the struct for our node
#[derive(Debug, Clone)]
struct Node<'a> {
    hash: String, // the node's hash
    val: &'a str, // JSON string of the nodes value
    l: Option<Box<Node<'a>>>, // can use box / rc etc
    r: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {

    //  insert(key, value), @return string (new root hash)
  pub fn insert(&mut self, key: &'a str, new_val: &'a str) {
    
    let test_hash : String = calculate_root_hash(self.val.as_bytes(), new_val.as_bytes());

    // create new root
    let mut new_root_node = Node {hash: "".to_string(), val: "", l: None, r: None};

    // create new leaf
    let new_leaf_node = Node { hash: key.to_string(), val: new_val, l: None, r: None};
    let new_boxed_leaf_node = Some(Box::new(new_leaf_node));
    
    // set root nodes
    new_root_node.r = new_boxed_leaf_node;
    new_root_node.l = Some(Box::new(self.clone()));

    new_root_node.hash = test_hash;

    // re-assign x to our new tree
    *self = new_root_node;

    //return test_hash;

    }
    // generateMerklePath @param key, @return array/list
    pub fn generate_merkle_path(key: &'a str){

    }
    

}

pub fn hash_string(val: &[u8]) -> String{
    let mut hasher = Sha512::new();

    hasher.input(val);
    let hashed = hasher.result_str();

    return hashed;
}

pub fn calculate_root_hash<'a>(first: &[u8], second: &[u8]) -> String {
    let mut hasher = Sha512::new();

    hasher.input(first);
    hasher.input(second);
    let root_hash : String = hasher.result_str();

    return root_hash;
}

fn main () {
    let mut x = Node {hash: hash_string(b"mkey").to_string(), val: "m", l: None, r: None };
    
    let hashed = hash_string(b"z");
    x.insert(&hashed, "z");
   
   println!("{:?}", x);
}
