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

fn main() {
 
    let mut vec = vec![3,2,1];
    vec.sort_by(|a,b|{
        return b.cmp(a)
    });

   println!("{:?}",vec);

    let refcell_vec = RefCell::new(vec.clone());
    refcell_vec.borrow_mut().push(1);
    refcell_vec.borrow();

    let mut cell_vec = Cell::new(vec);
    cell_vec.borrow_mut();
    cell_vec.borrow()
    
    // let mut hm = HashMap::new();
    
}
