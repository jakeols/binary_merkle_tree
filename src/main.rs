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

    }

    // generateMerklePath @param key, @return vector 
    pub fn generate_merkle_path(&mut self, key: &'a str) -> Vec<Node> {
        let mut vec = Vec::new();
        let mut path = Vec::new();

        let mut temp_self = self.clone();

        // vector rep
        loop {
             if temp_self.l.is_none() == true || temp_self.l.is_none() == true {
                 break;
            }

            else {
                
            vec.push(temp_self.l.clone());
            temp_self.l = None;

            vec.push(temp_self.r.clone());
            temp_self.r = None;
          //  println!("{:?}", vec);
            }
        }


        

        return path;
        
    }

 pub fn delete(&mut self, key: &'a str) -> bool {
     

	if key == self.hash {
			return true;
		}

        match self.r {
            None => false,
            Some(ref mut node) => node.delete(key)
        }
		
		// else {
		// 	match self.val {
		// 		None => false,
		// 		Some(ref mut node) => node.delete(key)
		// 	}
		// }
        
	


}

// pub fn verify_merkle_path(&mut self, Vec<Node> merkle_path){

// }



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

    let hashed_two = hash_string(b"k");
    x.insert(&hashed_two, "k");
   
   println!("{:?}", x);
   println!("deleting...");
   let key_to_del = "2af8a9104b3f64ed640d8c7e298d2d480f03a3610cbc2b33474321ec59024a48592ea8545e41e09d5d1108759df48ede0054f225df39d4f0f312450e0aa9dd25";
   x.delete(key_to_del);
   println!("{:?}", x);

   x.generate_merkle_path("z");
}
