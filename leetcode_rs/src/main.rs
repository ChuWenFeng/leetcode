use std::{any::{Any, TypeId}, fmt::Debug, clone, collections::HashMap, cell::{RefCell, Cell}, borrow::{BorrowMut, Borrow}};

trait T1 {
    type SF;
    type F2;
    fn test(&self);
}
trait T2 {
    type SF;
    fn test(&self);
    
}
// #[derive(Clone)]
struct Node{
    b:Box<usize>,
}
impl Node{
    fn new()->Self{
        Node {b:Box::new(0)}
    }
}
fn main() {
    let mut hash = HashMap::new();
    hash.insert(0, 0);
    hash.remove(&0);
    println!("{:?}",hash.get(&0));
}
