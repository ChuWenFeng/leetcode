
use super::Solution;
/*
 * @lc app=leetcode.cn id=146 lang=rust
 *
 * [146] LRU 缓存
 */

// @lc code=start
use std::{collections::HashMap, cell::RefCell, rc::Rc, borrow::BorrowMut};

struct LRUCache {
    size:i32,
    capacity:i32,
    cache:HashMap<i32,Rc<RefCell<DLinkedNode>>>,
    head:Rc<RefCell<DLinkedNode>>,
    tail:Rc<RefCell<DLinkedNode>>,
}

struct DLinkedNode{
    key:i32,
    val:i32,
    prev:Option<Rc<RefCell<DLinkedNode>>>,
    next:Option<Rc<RefCell<DLinkedNode>>>
}

impl DLinkedNode{
    fn new(key:i32,val:i32)->Self{
        DLinkedNode { key: key, val: val, prev: None, next: None }
    }
    fn new_rc_refcell(key:i32,val:i32)->Rc<RefCell<Self>>{
        Rc::new(RefCell::new(DLinkedNode::new(key,val)))
    }
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl LRUCache {

    fn new(capacity: i32) -> Self {
        let mut lru = LRUCache { 
            size: 0, 
            capacity: capacity, 
            cache: HashMap::new(), 
            head: DLinkedNode::new_rc_refcell(-1, -1), 
            tail: DLinkedNode::new_rc_refcell(-1, -1), 
        };
        lru.head.as_ref().borrow_mut().next = Some(lru.tail.clone());
        lru.tail.as_ref().borrow_mut().prev = Some(lru.head.clone());
        lru
    }
    
    fn get(&mut self, key: i32) -> i32 {
        if let Some(node) = self.cache.get(&key){
            self.move_to_head(Rc::clone(node));
            
            return node.as_ref().borrow().val;

        }else{
            return -1;
        }
    }
    
    fn put(&mut self, key: i32, value: i32) {
        if let Some(node) = self.cache.get(&key){
            node.as_ref().borrow_mut().val = value;
            self.move_to_head(node.clone());

        }else{
            let mut new_node = DLinkedNode::new_rc_refcell(key, value);
            
            self.cache.insert(key, new_node.clone());

            self.move_to_head(new_node.clone());


            if self.size == self.capacity{
                let tail = self.tail.as_ref().borrow().prev.clone().unwrap();
                tail.as_ref().borrow_mut().prev.clone().unwrap().as_ref().borrow_mut().next = Some(self.tail.clone());
                self.tail.as_ref().borrow_mut().prev = tail.as_ref().borrow().prev.clone();
                self.cache.remove(&tail.as_ref().borrow().key);

                self.size -= 1;
            }

            self.size +=1;
        }

    }

    fn move_to_head(&self,node:Rc<RefCell<DLinkedNode>>){
        let mut node_mut = node.as_ref().borrow_mut();
        if let Some(pre) = node_mut.prev.clone(){
            pre.as_ref().borrow_mut().next = node_mut.next.clone();
        }
        if let Some(next) = node_mut.next.clone(){
            next.as_ref().borrow_mut().prev = node_mut.prev.clone();
        }
        node_mut.next = self.head.as_ref().borrow().next.clone();

        self.head.as_ref().borrow_mut().next.clone().unwrap().as_ref().borrow_mut().prev = Some(node.clone());

        self.head.as_ref().borrow_mut().next = Some(node.clone());

        node_mut.prev = Some(self.head.clone());
    }
}

/*
 * Your LRUCache object will be instantiated and called as such:
 * let obj = LRUCache::new(capacity);
 * let ret_1: i32 = obj.get(key);
 * obj.put(key, value);
 */
// @lc code=end

#[test]
fn test(){
    let mut lru = LRUCache::new(2);
    lru.put(1, 1);
    lru.put(2, 2);
    assert_eq!(lru.get(1),1);
    lru.put(3, 3);
    assert_eq!(lru.get(2),-1);
    lru.put(4, 4);
    assert_eq!(lru.get(1),-1);

}