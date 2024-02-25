use ace::rope::{*};

fn main(){
    let mut rope = Rope::new("hello gio").unwrap();
    rope = rope.append("string").unwrap();
    rope = rope.append("2").unwrap();
    rope = rope.append("2").unwrap();
    rope.helper_inorder();
    println!("{}", rope.get_height());
}