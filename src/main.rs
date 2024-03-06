use ace::rope::{*};

fn main(){
    let path = "./asset/test.txt";
    let rope = Rope::from_file(path).unwrap();
    println!("Total weight: {}", rope.get_rope_length());

}