use std::ops::Deref;

use ace::rope::{*};
use ace::testing::MyBox;

fn main(){
    let path = "asset/test.txt";
    let rope = Rope::from_file(path).unwrap();
    let my_box = MyBox::new("Hello world".to_string());
    let refer = my_box.deref();
    
}