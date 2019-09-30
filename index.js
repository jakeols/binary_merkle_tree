/**
 * Merkle Tree implementation in JavaScript
 * Yes, I sadly failed at finishing this in rust. here's it in JS :/
 * Using array representation, I'm aware of efficiency problems. I will try and correct
 * them before submission 
 */
var sha512 = require('js-sha512').sha512;
var _ = require('underscore');
//our utilities
var utilities = require('util');


/**
 * takes props object with hash / val keys
 */
class Node {
    constructor(props){
        this.hash = props.hash;
        this.value = props.value;
    }

}

/**
 * merkle tree, using syntatic sugar class keyword
 */
class Tree {

    constructor(){
        this.nodes = [];
        this.tree = [];

        // create initial node with null val
        let root = new Node({hash: sha512("root"), value: null,});
        this.nodes.push(root);
    }

    /**
     * 
     * @param {object} props 
     */
    insert(props){
        // check to see if last node was duplicated, remove if true
       if(this.nodes.length > 2 && this.nodes[this.nodes.length - 1].hash == this.nodes[this.nodes.length - 2].hash ){
           this.nodes.pop();
       }
        let toInsert = new Node(props); // new node
        this.nodes.push(toInsert);

        this.generateTree();

        return props.hash;

  }
  

  /**
   * generates tree from leaf nodes
   */
  generateTree(){
    this.tree = []; // make sure we're starting with a blank slate
    let num_nodes = this.nodes.length; // number of nodes we have
    if(num_nodes < 2){ // no tree available
        return this.nodes;
    }
    
    if(num_nodes % 2 !== 0){ // not even, duplicate node
        let toDuplicate = this.nodes[this.nodes.length -1];
        this.nodes.push(toDuplicate);
    }
    // add all nodes to tree
    this.tree = this.nodes.slice(0);
    var workingTree = this.nodes.slice(0);

    while(true){ // could do recursively also
        if(workingTree.length == 1){
            break;
        }

        // always right sibling
        let sibling = workingTree[1];
        let parent = new Node({hash: sha512(workingTree[0].hash, workingTree[1].hash), value: `parent(${workingTree[0].value} | ${workingTree[1].value})`});
        this.tree.push(parent);
        workingTree.push(parent);
        workingTree.splice(0, 1);
        workingTree.splice(0, 1);
    }

  }

    /**
     * deletes from tree via specified key
     * @param {string} key 
     */
    delete(key){
        // filter out leaf node
        this.nodes = this.nodes.filter((node) => {
            if(node.hash !== key) {
                return node;
            }
        });
        this.generateTree();

        // determine if root hash has changed

    }

    /**
     * generates path
     * @param {string} key 
     */
    generateMerklePath(key){
        let merklePath = [];
        let leafPosition = null;

        if(key === null){
            return merklePath;
        }
        

        // get sibling leaf node
        this.tree.forEach((node, i) => {
           if(node.hash === key){
               leafPosition = i;
               // if leaf needs right sibling
               if(i % 2 == 0){
                let sibling = this.tree[i - 1];
                merklePath.push(sibling);
            }
               else {
                    let sibling = this.tree[i - 1];
                    merklePath.push(sibling);
               }
           }
        });

        let level = 1;
        var workingLeaves = this.nodes.slice(0);
        let workingTree = this.tree.slice(0);
        // now loop until we get the parent 
        while(true){

            if(workingTree.length == 0){// break when we get to the root
                break;
            }

            // remove level from tree (we already added at this level)
            let num_parents = this.nodes.length / 2;
           
            workingTree.splice(0, this.workingLeaves.length); 

            

            // set leaves
            workingLeaves = workingTree.slice(0, num_parents);

            console.log(`the leaves for level ${level} are: `, workingLeaves);
            // now working leaeves is correct, we just need to find the correct parent
            
            level++;







        }

        return merklePath;

    }


    /**
     * verifies path
     * @param {key} key 
     * @param {path} path 
     */
    verifyMerklePath(key, path){

    

    }


}


/**
 * testing stuff here
 */ 
let MerkleTree = new Tree();
MerkleTree.insert({hash: sha512("first"), value: "first node"});
MerkleTree.insert({hash: sha512("second"), value: "second node"});
MerkleTree.insert({hash: sha512("third"), value: "third"});
console.log(MerkleTree);

console.log(MerkleTree.generateMerklePath(sha512("first")));


