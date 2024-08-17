use std::rc::Rc;
use std::cell::RefCell;

struct Vertex {
  value: i32,
  connection1: Option<Rc<RefCell<Vertex>>>,
}

fn main() {
    // 將 value 設為 1
    let a = RefCell::new(Vertex {
      value: 1,
      connection1: None,
    });
    println!("{}", a.borrow().value);
    
    // 將 value 設為 2
    let b = Rc::new(RefCell::new(Vertex {
      value: 2,
      connection1: None,
    }));
    
    // 將 value 設為 3
    let c = Rc::new(RefCell::new(Vertex {
      value: 3,
      connection1: None,
    }));
    
    // a.connection1 = b
    a.borrow_mut().connection1 = Some(Rc::clone(&b));
    println!("{}", a.borrow().value); 
    println!("{}", 
        a.borrow().connection1.clone().expect("REASON").borrow().value);
        
    // 改為 a.connection1 = c
    a.borrow_mut().connection1 = Some(Rc::clone(&c));
    println!("{}", 
        a.borrow().connection1.clone().expect("REASON").borrow().value);
    
}
