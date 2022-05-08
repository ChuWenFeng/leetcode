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
    

    // let mut vd:VecDeque<i32> = std::collections::VecDeque::new();
    // vd.push_back(1);
    // vd.push_front(2);

    let rc = RefCell::new(5);
    
    get_mut_refcell(&rc);
    let refrc = rc.borrow();
    println!("{}",refrc);
    
}

fn get_mut_refcell(rc:&RefCell<i32>){
    let mut mutrc =  rc.borrow_mut();
    *mutrc = 7;
}
