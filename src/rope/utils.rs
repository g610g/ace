use super::{Rope, WeightNode, LeafNode};
use std::{error::Error, fs::File, io::BufReader};
impl WeightNode {
        //rotates left side which is your self
        pub fn rotate_right(mut self) -> Option<Box<Rope>>{
            //self is a weight struct
            let  box_new_root = self.left.unwrap();   
            let mut new_root = box_new_root.return_weight_struct();
            let new_root_right = new_root.right.unwrap();
            self.weight = new_root_right.get_weight();
            self.left = Some(new_root_right);
            new_root.right = Rope::new_weight_node(self);
            Rope::new_weight_node(new_root)
        }
        pub fn rotate_left(mut self)-> Option<Box<Rope>>{
            let box_new_root = self.right.unwrap();
            let mut new_root = box_new_root.return_weight_struct();      
            self.right = new_root.left; 
            new_root.left = Rope::new_weight_node(self);
            return Rope::new_weight_node(new_root);
        }
}
impl Rope {
    pub fn new_leaf_node(str: &str) -> Option<Box<Rope>>{
        Some(Box::new(
            LeafNode(LeafNode::new(str))
        ))
    }
    pub fn open_file(path:&str)->Result<File, Box<dyn Error>>{
        let file_instance = File::open(path)?;
        return Ok(file_instance);        
    }
    //checks if the split n is within the range of string len
    pub fn in_range(string:&str, len:usize) -> bool{
        if string.len() > len{
            return true;
        } 
        false
    }
    pub fn is_leaf(&self) -> bool{
        match self {
            LeafNode(lf) => true,
            WeightNode(w) => false
        }
    }
    pub fn get_weight(&self) -> usize{
        match self{
            WeightNode(w) => {
                return w.weight;
            },
            LeafNode(l) => {
                return l.length;
            }
        }
    }
    pub fn set_weight(&mut self, weight:usize) -> Result<(), &str>{
        if let WeightNode(w)  = self{
            w.weight = weight;
            return Ok(());
        }
        return Err("Cannot set weight to the node");
        
    }
    pub fn get_height(&self) ->  i32{
        if self.is_leaf(){
            return  0;
        }
        if let WeightNode(w)  = self{
            let mut  left_height = 0;
            let mut right_height = 0;
            // let mut weight_left = 0;
            // let mut weight_right = 0;
            if let Some(left) = w.left.as_ref(){
                left_height = left.get_height();
            }
            if let Some(right) = w.right.as_ref(){
              right_height = right.get_height();
            }
            return (left_height.max(right_height) + 1);
            
        }
        return 0;
    }
    pub fn return_weight_struct(self) -> WeightNode{
        match self {
            WeightNode(w) => {
               return  w; 
            }
            _ => {panic!("Not a weight node")}
        }
    }
    pub fn _is_weight(&self) -> bool{
        match self {
            LeafNode(lf) => false,
            WeightNode(w) => true
        }
    }
    pub fn new_weight_node(weight_node: WeightNode) -> Option<Box<Rope>>{
        Some(
            Box::new(
                WeightNode(weight_node)
            )
        )
    }
     //utility function that returns the overall length of the rope
     pub fn get_rope_length(&self)-> usize{
        if self.is_leaf(){
            return self.get_weight();
        }
        let mut left_len = 0;
        let mut right_len = 0;
        match self{
            WeightNode(w) => {
                left_len = w.left.as_ref().unwrap().get_rope_length();
                right_len = w.right.as_ref().unwrap().get_rope_length();
                return left_len + right_len;
            },
            _ => {return 0}
        }
    }
    pub fn get_height_difference(&self) -> i32{
        if self.is_leaf(){
            return 0;
        }
        let mut left_height = 0;
        let mut right_height = 0;
        if let WeightNode(w) = self{
            if let Some(w_left) = w.left.as_ref(){
                left_height = w_left.get_height();
            }
            if let Some(w_right) = w.right.as_ref(){
                right_height = w_right.get_height();
            }
        }
        return left_height - right_height;
    }

 
}
