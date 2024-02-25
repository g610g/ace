use super::rope::Rope::{*};
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
pub struct WeightNode{
    weight:usize,
    height:i32,
    left: Option<Box<Rope>>,
    right:Option<Box<Rope>>
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
    //rotates left side which is your self
    fn rotate_right(mut self) -> Option<Box<Rope>>{
        //self is a weight struct
        let mut new_root = self.left.unwrap();   
        let mut w = new_root.return_weight_struct();
        if let Some(right) = w.right.as_mut(){
            self.weight =  right.get_weight();
        }   
        self.left = w.right;
        w.right = Rope::new_weight_node(self);
        return Rope::new_weight_node(w);
    }
}
pub enum Rope{
    LeafNode(LeafNode),
    WeightNode(WeightNode),
}
impl Rope{
    fn new_leaf_node(str: &str) -> Option<Box<Rope>>{
        Some(Box::new(
            LeafNode(LeafNode::new(str))
        ))
    }
    fn is_leaf(&self) -> bool{
        match self {
            LeafNode(lf) => true,
            WeightNode(w) => false
        }
    }
    fn get_weight(&self) -> usize{
        if let WeightNode(w) = self{
            return w.weight;
        }
        return 0;
    }
    fn return_weight_struct(self) -> WeightNode{
        match self {
            WeightNode(w) => {
               return  w; 
            }
            _ => {panic!("Not an weight node")}
        }
    }
    fn is_weight(&self) -> bool{
        match self {
            LeafNode(lf) => false,
            WeightNode(w) => true
        }
    }
    fn new_weight_node(weight_node: WeightNode) -> Option<Box<Rope>>{
        Some(
            Box::new(
                WeightNode(weight_node)
            )
        )
    }
    pub fn new(string: &str) -> Result<Rope, &str>{
        let leaf_node = Self::new_leaf_node(string);
        if leaf_node.is_some(){
            //creates the struct weightNode as mutable
            let mut w = WeightNode::new(string.len(), 1);
            //assign the weight_node to the newly leaf_node
            w.left = leaf_node;
            //wrap enum weight_node the struct weight_node
            let weight_node = WeightNode(w);
            return Ok(weight_node);
        }
        Err("error creating rope")
    }
    pub fn append(self, string: &str)-> Result<Rope, &str>{
        match self{
            WeightNode(mut w) => {
                let new_ln = Self::new_leaf_node(string);
                w.right = new_ln;
                let mut new_w = WeightNode::new(w.weight + string.len(), w.height + 1);
               new_w.left =  WeightNode(w).balance();
                //rebalance should be made here
                //also change the weight of the new_w since after balancing weights will be adjusted
                return Ok(WeightNode(new_w));
            }
            _ => {Err("Error appending a string")}
        }
    }
    pub fn get_height(&mut self) -> i32{
        if self.is_leaf(){
            return 0;
        }
        if let WeightNode(w)  = self{
            let mut  left_height = 0;
            let mut right_height = 0;
            if let Some(left) = w.left.as_mut(){
                left_height = left.get_height();
            }
            if let Some(right) = w.right.as_mut(){
                right_height = right.get_height();
            }
            return left_height.max(right_height) + 1;
            
        }
        return 0;
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
    
       fn balance(self) -> Option<Box<Rope>>{
        if self.is_leaf(){
            panic!("Cannot balance or rotate leaf node!");
        }
        match self{
            WeightNode(mut w) => {
                let mut left_height = 0;
                let mut right_height = 0;
                if let Some(left) = w.left.as_mut(){
                   left_height = left.get_height();

                }
                if let Some(right) = w.right.as_mut(){
                    right_height = right.get_height();
                }
                if left_height - right_height > 1 || left_height - right_height < -1{
                                    
                    //you have to find a way to extract the weight struct and rotate it to the left and will return a Rope num
                    return w.rotate_right();
                }else{
                    return Rope::new_weight_node(w);
                }
                
            },
            _ => {panic!("Leaf node ni")}
        };
        
     
        // return self;
    }


}

