use std::{any::{Any, TypeId}, fmt::Debug, clone, collections::{HashMap, VecDeque}, cell::{RefCell, Cell}, borrow::{BorrowMut, Borrow}, rc::{Rc, Weak}, sync::Arc};

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
macro_rules! statements {
    ($($stmt:stmt)*) => ($($stmt)*);
}

fn main() {
    // statements! {
    //     struct Foo;
    //     fn foo() {}
    //     let zig = 3
    //     let zig = 3;
    //     if true {} else {}
    //     {}
    // }

   
    let strong = Rc::new(Box::new("Hello".to_string()));
    let weak = Rc::downgrade(&strong);
    let raw = weak.into_raw();
    
    assert_eq!(1, Rc::weak_count(&strong));
    assert_eq!("Hello", unsafe { &**raw });
    
    drop(unsafe { Weak::from_raw(raw) });
    assert_eq!(0, Rc::weak_count(&strong));
    Arc::new(1);
}

fn get_mut_refcell(rc:&RefCell<i32>){
    let mut mutrc =  rc.borrow_mut();
    *mutrc = 7;
}
