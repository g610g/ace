use ace::rope::{*};
fn main(){
    let mut rope = Rope::new("hello gio").unwrap();
    rope = rope.append("string").unwrap();
    rope = rope.append("2").unwrap();
    rope = rope.append("hello world again").unwrap();
    rope = rope.append("string").unwrap();
    // rope = rope.append("2").unwrap();
    rope = rope.append("hello world again").unwrap();
    rope = rope.append("hello another again").unwrap();    
    rope = rope.append("hello another again").unwrap();    
    rope = rope.append("hello another again").unwrap();    
    rope = rope.append("hello another again").unwrap();    
    rope = rope.append("hello another again").unwrap();    
    rope = rope.append("hello another again").unwrap();    
    rope = rope.append("hello another again").unwrap();    
    rope = rope.append("hello another again").unwrap();    
    rope = rope.append("hello another again").unwrap();    
    rope = rope.append("hello another again").unwrap();    

    // // rope.helper_inorder();}
    println!("Total weight: {}", rope.get_rope_length());

}