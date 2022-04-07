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
    let list:[Option<Node>;3] = [None,None,None];
    let lv:Vec<Option<Node>> = vec![None;3];
}
