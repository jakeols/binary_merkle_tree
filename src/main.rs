// note: Rust will not dynamically allocate memory for the node at runtime, must wrap it / be explicit
// note: use SHA-512 for hash of val
// key is hash of the JSON string of the entry and value is the JSON string of the entry.

// -------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------
// -------------------------------------------------------------------------------------
// ---------------------------------START-----------------------------------------------


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
 
  pub fn insert(&mut self, new_val: &'a str) {
        let target_node = if new_val < self.val { &mut self.l } else { &mut self.r };
        match target_node {
            &mut Some(ref mut subnode) => subnode.insert(new_val),
            &mut None => {
                let new_node = Node { val: new_val, l: None, r: None };
                let boxed_node = Some(Box::new(new_node));
                *target_node = boxed_node;
            }
        }
    }

    // delete @param key, @return key (updated root hash)
    pub fn delete(key: &'a str) -> &'a str {

        return "key";

    }
    // generateMerklePath @param key, @return array/list
    pub fn generateMerklePath(key: &'a str){

    }

    // verifyMerklePath(key, location, MerklePath) , @return bool 
}

fn main () {
    let mut x = Node { val: "m", l: None, r: None };
    x.insert("z");
    x.insert("b");
    x.insert("c");
    x.insert("z");
   
    println!("{:?}", x);
}
