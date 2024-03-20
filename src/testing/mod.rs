use std::{cell::{Ref, RefCell}, ops::Deref};

pub struct MyBox<T>(T);
impl <T> MyBox<T> {
    pub fn new(val:T) -> MyBox<T>{
        MyBox(val)
    }
    
}
impl <T> Deref for MyBox<T>{
    //*(y.deref())
    type Target = T;
    fn deref(&self) -> &Self::Target {
       &self.0 
    }
}
pub trait Messenger {
    fn send(&self, message:&str); 
}
pub struct MockMessenger{
    sent_message: RefCell<Vec<String>>

}
impl MockMessenger {
    fn new() -> MockMessenger{
        MockMessenger{
            sent_message:RefCell::new(vec![])
        }
    } 
}
impl Messenger for MockMessenger {
    fn send(&self, message:&str){
        let immutable_borrow = self.sent_message.borrow();
        let mut mut_ref = self.sent_message.borrow_mut();
        mut_ref.push(message.to_string());
       println!("{:?}", immutable_borrow);
    }
}
pub struct LimitTracker<'a, T:Messenger>{
    messenger:&'a T,
    value:usize,
    max:usize
}
impl <'a, T> LimitTracker<'a, T>
where
    T:Messenger{
        pub fn new(messenger: &'a T, value:usize, max:usize) -> LimitTracker<'a, T>{
            LimitTracker{
                messenger,
                value,
                max 
            }
        }
        pub fn set_value(&mut self, value:usize){
            self.value = value;
            let percentage_of_max = self.value as f64 / self.max as f64;
            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
}
