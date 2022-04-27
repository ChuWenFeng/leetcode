use std::{any::{Any, TypeId}, fmt::Debug, clone, collections::{HashMap, VecDeque}, cell::{RefCell, Cell}, borrow::{BorrowMut, Borrow}, rc::Rc};

use rand::Rng;

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
    // let mut hash = HashMap::new();
    // hash.insert(0, 0);
    // hash.remove(&0);
    // println!("{:?}",hash.get(&0));
    let mut vd:VecDeque<i32> = std::collections::VecDeque::new();
    vd.push_back(1);
    vd.push_front(2);
    // vd.allocator()
    // Some(Rc::new(RefCell::new(1))).as_ref().map(|r|{
    
    // });
    // let x = "5".to_string();
    // let mut xc = Cell::new(x);
    
    let mut r = rand::thread_rng();
    println!("{}",r.gen_range(0, 1));

}
