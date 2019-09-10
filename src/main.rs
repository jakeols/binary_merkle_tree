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
#[derive(Debug, Clone)]
struct Node<'a> {
    hash: &'a str, // the node's hash
    val: &'a str, // JSON string of the nodes value
    l: Option<Box<Node<'a>>>, // can use box / rc etc
    r: Option<Box<Node<'a>>>,
}

impl<'a> Node<'a> {

    //  insert(key, value), @return string (new root hash)
  pub fn insert(&mut self, key: &'a str, new_val: &'a str) {

    // create new root
    let mut new_root_node = Node {hash: "roothash", val: "", l: None, r: None};

    // create new leaf
    let new_leaf_node = Node { hash: key, val: new_val, l: None, r: None};
    let new_boxed_leaf_node = Some(Box::new(new_leaf_node));
    new_root_node.r = new_boxed_leaf_node;
    new_root_node.l = Some(Box::new(self.clone()));
    *self = new_root_node;
    // println!("{:?}", new_root_node);
    

      
      // traverse to the right and insert
        // let target_node = &mut self.r;
        // match target_node {
        //     &mut Some(ref mut subnode) => subnode.insert(key, new_val),
        //     &mut None => {

        //         let new_node = Node {hash: key, val: new_val, l: None, r: None };
        //         let boxed_node = Some(Box::new(new_node));

        //         // create parent with target
                
        //         *target_node = boxed_node;


        //     }
        // }

    

       // println!("{:?}", self);

    }
    // generateMerklePath @param key, @return array/list
    pub fn generate_merkle_path(key: &'a str){

    }

}

fn main () {
    let mut hasher = Sha512::new();
    hasher.input(b"m");
    let hash_result = hasher.result_str(); // result of hashing the value
    // println!("{}", hash_result);
    

    let mut x = Node {hash: "mhash", val: "m", l: None, r: None };
    x.insert("zkey", "z");
    x.insert("jkey", "j");
    x.insert("lkey", "l");
    x.insert("zkey", "z");
    println!("{:?}", x);
}
