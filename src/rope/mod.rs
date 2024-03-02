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
    //rotates left side which is your self
    fn rotate_right(mut self) -> Option<Box<Rope>>{
        //self is a weight struct
        let  box_new_root = self.left.unwrap();   
        let mut new_root = box_new_root.return_weight_struct();
        let new_root_right = new_root.right.unwrap();
        self.weight = new_root_right.get_weight();
        self.left = Some(new_root_right);
        new_root.right = Rope::new_weight_node(self);
        Rope::new_weight_node(new_root)
    }
    fn rotate_left(mut self)-> Option<Box<Rope>>{
        let box_new_root = self.right.unwrap();
        let mut new_root = box_new_root.return_weight_struct();      
        self.right = new_root.left; 
        new_root.left = Rope::new_weight_node(self);
        return Rope::new_weight_node(new_root);
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
    fn set_weight(&mut self, weight:usize) -> Result<(), &str>{
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
    fn _is_weight(&self) -> bool{
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
    fn get_height_difference(&self) -> i32{
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

