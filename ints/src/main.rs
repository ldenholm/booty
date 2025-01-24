use std::rc::Rc;
use std::sync::{Arc, Mutex};
fn main() {
    let a = 1; // int on stack
    let b = Box::new(2); // int on heap (a boxed integer)
    let c = Rc::new(Box::new(3)); // boxed integer wrapped within reference counter 
    let d = Arc::new(Mutex::new(4)); // int wrapped in atomic rc with mutual exc lock
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
    int_things();
}


fn int_things() {
    let a = 10; // type inferred by compiler
    let b: i32 = 20; // explicit type
    let c = 30i32; // include type annotation in its literal form
    let d = 30_i32; // identical value to c, underscore has no functional impact 
    let e = add(a, b) + add(c, d);
    println!("(a + b) + (c + d) = {}. Associativity of addition.", e);
}

// must declare types when constructing functions
fn add(x: i32, y: i32) -> i32 {
    // return is implicit
    x + y
    // note x + y; with semicolon produces return type of unit: ()
    // rather than i32.
}