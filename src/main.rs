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


// define the struct for our node
#[derive(Debug)]
struct Node<'a> {
    hash: &'a str, // the node's hash
    val: &'a str, // JSON string of the nodes value
    l: Option<Box<Node<'a>>>, // can use box / rc etc
    r: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {

    //  insert(key, value), @return string (new root hash)
 
  pub fn insert(&mut self, key: &'a str, new_val: &'a str) {
        let target_node = if new_val < self.val { &mut self.l } else { &mut self.r };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(key, new_val),
            &mut None => {
                let new_node = Node {hash: key, val: new_val, l: None, r: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
        // re-calculate hashes
    }

    // delete @param key, @return key (updated root hash)
    pub fn delete(key: &'a str) -> &'a str {

        return "key";

    }
    // generateMerklePath @param key, @return array/list
    pub fn generate_merkle_path(key: &'a str){

    }

    // verifyMerklePath(key, location, MerklePath) , @return bool 
}

fn main () {
    let mut hasher = Sha512::new();
    hasher.input(b"hello world");
    let hash_result = hasher.result_str(); // result of hashing the value
   println!("{}", hash_result);
    

    let mut x = Node {hash: &hasher.result_str(), val: "m", l: None, r: None };
    x.insert(&hash_result, "z");
   
    println!("{:?}", x);
}
