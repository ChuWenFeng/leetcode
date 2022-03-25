use std::{any::{Any, TypeId}, fmt::Debug, clone, collections::HashMap};

trait T1 {
    type SF;
    type F2;
    fn test(&self);
}
trait T2 {
    type SF;
    fn test(&self);
    
}
struct Point{
    pub value:i32,
    d: Option<dyn T2<SF = Point>>,
}
impl Point{
    fn test(&self){
        println!("self test");
    }
}
impl T1 for Point{
    type SF = Self;
    type F2 = i32;
    fn test(&self){
        println!("t1 trait test:{}",self.value);
    }
}
impl T2 for Point{
    type SF = Self;
    fn test(&self){
        println!("t2 trait test{}",self.value);
    }
}
fn main() {
    let reflect_i31 = 0;
    let i32_type:i32 = 0;
    println!("i32 type_id:{:?}",i32_type.type_id());
    let p = Point{value:1,d:None};

    let tt:&dyn T1<SF = Point, F2 = i32> = &p;

    println!("Point type_id:{:?}",p.type_id());
    T1::test(&p);

    // print_type_id(&reflect_i31);
    print_type_id(&p);

    let tmp  =((14 as u8) & (1 << 1)) << 5;
    println!("tmp:{:b}",tmp);

    let mut hm = HashMap::new();
    
}




fn print_type_id<'a>(input:&'a dyn Any)->&dyn T2<SF = Point>{
    println!("{:?}",input.type_id());
    let dc:&dyn T2<SF = Point<'a>> =  input.downcast_ref::<Point<'a>>().unwrap();
    return dc;
}

struct StructVisitor{
    fields_seq:Vec<(String,u8)>,
    size:usize,
}
impl StructVisitor{
    fn new()->Self{
        StructVisitor { fields_seq: vec![], size:0}
    }
}
