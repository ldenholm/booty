use std::rc::Rc;
use std::sync::{Arc, Mutex};
fn main() {
    let a = 1; // int on stack
    let b = Box::new(2); // int on heap (a boxed integer)
    let c = Rc::new(Box::new(3)); // boxed integer wrapped within reference counter 
    let d = Arc::new(Mutex::new(4)); // int wrapped in atomic rc with mutual exc lock
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
}
