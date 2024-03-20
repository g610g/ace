use std::{fs};

use super::rope::Rope::{*};
mod utils;
#[derive(Debug)]
pub struct LeafNode{
    string: String,
    length:usize,
    start:usize
}
impl LeafNode{
    fn new(str: &str) -> Self{
        LeafNode{
            string:str.to_string(),
            length: str.len(),
            start: 0
        }
    }
}
#[derive(Debug)]
pub struct WeightNode{
    weight:usize,
    height:i32,
    pub left: Option<Box<Rope>>,
    pub right:Option<Box<Rope>>
}
impl WeightNode{
    fn new(weight:usize, height:i32)-> Self{
        WeightNode{
            weight,
            height,
            left:None,
            right:None
        }
    }

}
#[derive(Debug)]
pub enum Rope{
    LeafNode(LeafNode),
    WeightNode(WeightNode),
}
impl Rope{
    pub fn from_file(path:&str)-> Result<Box<Rope>, &str>{
        let string_contents = fs::read_to_string(path).unwrap();
        let n_split:usize = 15;
        let mut rope:Box<Rope>;
        let mut first = "";
        let mut second = "";
        if !Rope::in_range(&string_contents, n_split){
            rope = Rope::new(&string_contents).unwrap();
            return Ok(rope);  
        }else{
            //initializing the rope before going to the loop
            (first, second) = string_contents.split_at(n_split);
            rope = Rope::new(first).unwrap();
        }
        loop {
            if second.len() < n_split{
                rope = rope.append( second).unwrap();
                break;
            }
            (first, second) = second.split_at(n_split);
            rope = rope.append(first).unwrap();
        }
        return Ok((rope));
    } 
    pub fn new(string: &str) -> Result<Box<Rope>, &str>{
        // let leaf_node = Self::new_leaf_node(string);
        let leaf_node = LeafNode::new(string);
        Ok(Box::new(LeafNode(leaf_node)))
    }
    pub fn helper_inorder(&self){
        if self.is_leaf(){
            println!("leaf");
            return;
        }
        match self{
            WeightNode(w) => {
                if let Some(left) = w.left.as_ref(){
                    left.helper_inorder();
                    println!("{}", w.weight);
                }
                if let Some(right) = w.right.as_ref(){
                    right.helper_inorder();
                }
            },
            _ => {} 
        }
    }
    pub fn append(self, string: &str)-> Result<Box<Rope>, &str>{
        //takes O(1) without balancing O(logn) prolly when doing balancing since getting the height would traverse each of the node
        match self{
            WeightNode(w) => {
                //creates the new leafnode for the string
                let new_ln = Self::new_leaf_node(string);
                let mut left_weight:usize = 0;
                left_weight += w.weight;
                if let Some(right) = w.right.as_ref(){
                    left_weight += right.get_weight();
                }
                // println!("Total left weight:{left_weight}");
                let mut new_root = WeightNode::new(left_weight, 0);
                //new root right is the new leaf
                new_root.right = new_ln;
                //new root left is the current rope the we made
                //find the height difference of your left side and then balance it 
                new_root.left = Self::new_weight_node(w);  
                return Ok(WeightNode(new_root).balance().unwrap());
            },
            LeafNode(l) => {
                let mut weight_root = WeightNode::new(l.length, 0);
                weight_root.left = Self::new_leaf_node(&l.string);
                weight_root.right = Self::new_leaf_node(string);
                let root = WeightNode(weight_root);
                return Ok(Box::new(root));
            }
        }
    }
    fn balance(self) -> Result<Box<Rope>, &'static str>{
        if self.is_leaf(){
            panic!("Cannot balance or rotate leaf node!");
        }
        
        let height_differ = self.get_height_difference();
        if height_differ > 1 || height_differ < -1{
            //get height difference of our left side. If it is zero or greater then do not rotate your left. If it is -1 or less than then rotate it to the left
            let mut root_weight = self.return_weight_struct();
            //the left of the root is moved here
            if let Some(left) = root_weight.left{
                if left.get_height_difference() <= -1 {
                    root_weight.left = left.return_weight_struct().rotate_left();
                    let left_ref = root_weight.left.as_mut().unwrap();
                    let left_weight = left_ref.get_rope_length();
                    println!("left_weight on ratation:{left_weight}");
                    match left_ref.set_weight(left_weight) {
                        Ok(()) => {}
                        Err(_e) => {panic!("Error")}    
                    }
                    //get the newly updated weight here for the left side
                    let new_root = root_weight.rotate_right().unwrap(); 
                    //get the height of the root new root here
                    return Ok(new_root);
                }
                else{
                    //this needs to be updated also for this kind of rotate                   
                    root_weight.left = Some(left);
                    let mut new_root = root_weight.rotate_right().unwrap();
                    let root_weight = new_root.get_rope_length();
                    match new_root.set_weight(root_weight){
                        Ok(()) => {},
                        Err(_e) => panic!("Error") 
                    }
                    return Ok(new_root);
                }
            //else part must be fixed LOGIC PROBLEM
            }else{
                return Err("There is an error");
            }

        }else{
            Ok(Box::new(self))
        }
    }


}

