/*
Concurrency and mutability
Concurrency means that parts of a program run independently of each other. 
Parallelism refers to these part executing at the same time.
For ease both these can be considered as concurrency itself
*/

use std::thread;// For threading

pub fn threading() {
    // the two pipes (||) where the parameters go, akin to a funcion signature's parameters,
    // without the need to always declare tpyes explicity. This was the variable can move from
    // the outer scope to inner scope 

    let handle = thread::spawn(|| {
        println!("Hello from a thread");
    });
    handle.join().unwrap();
}

pub fn Sanitize(s: String) -> String {
    let s = s.trim();
    let s = s.replace(" ", "_");
    s
}

/*
Interior Immutability
Can a variable be immutable and mutable at the same time ?
Ofcourse. Boxed variables (Box, Rc, and so on) are an immutable reference to the heap and they contain
the actual value
This concept of Interior Immutability is often used in combination with Rc in order to provide a value
to multiple owners with mutability at will.
Wrapping a RefCell in an Rc acts as the gatekeeper for having multiple owners, including a way to change
the contents. 
*/

use std::cell::RefCell;
use std::rc::Rc;

type Link = Option<Rc<RefCell<Node>>>;

#[derive(Clone)]
struct Node {
    value: String,
    next: Link,
    prev: Link,
}

pub fn append(&mut self, value: String) {
    let new = Rc::new(RefCell::new(Node::new(value)));
    match self.tail.take() {
        Some(old) => {
            old.borrow_mut().next = Some(new.clone());
            new.borrow_mut().prev = Some(old); 
        }
        None => self.head = Some(new.clone());
        
    }
}

