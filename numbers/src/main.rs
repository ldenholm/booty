use std::convert::TryInto;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
fn main() {
    let a = 1; // int on stack
    let b = Box::new(2); // int on heap (a boxed integer)
    let c = Rc::new(Box::new(3)); // boxed integer wrapped within reference counter 
    let d = Arc::new(Mutex::new(4)); // int wrapped in atomic rc with mutual exc lock
    println!("a: {:?}, b: {:?}, c: {:?}, d: {:?}", a, b, c, d);
    //int_things();
    //ints_and_floats();
    base_2_8_16();
    equalities_inequalities();
    try_into();
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

fn ints_and_floats() {
    let twenty = 20;
    let twenty_one: i32 = 21;
    let twenty_two = 22i32;
    let addition = twenty + twenty_one + twenty_two;
    println!("{} + {} + {} = {}", twenty, twenty_one, twenty_two, addition);

    let one_mil: i64 = 1_000_000; // we can use underscores here for readability
    println!("{}", one_mil.pow(2));
    
    let forty_twos = [
        42.0,
        42f32,
        42.0_f32,
    ];

    println!("{:02}", forty_twos[0]);
}

fn base_2_8_16() {
    // prefix ints with numeric literals during assignment
    let three = 0b11;
    let thirty = 0o36;
    let three_hundred = 0x12C;
    println!("base 10: {} {} {}", three, thirty, three_hundred);
    println!("base 2: {:b} {:b} {:b}", three, thirty, three_hundred);
    println!("base 8: {:o} {:o} {:o}", three, thirty, three_hundred);
    println!("base 16: {:x} {:x} {:x}", three, thirty, three_hundred);
}

fn equalities_inequalities() {
    // rust strict typing restricts comparators to a single type only
    let a: i8 = 10;
    let b: u32 = 40000;

    /* 
    if a < b: {
    //     println!("10 is less than 40000");
        }
    */

    // we can cast b to the same type as a:
    // note: its better to promote to the larger
    // type versus demoting to a smaller type.
    if (a as u32) < b {
        println!("10 is less than 40000");
    }
}

fn try_into() {
    let a: i32 = 10;
    let b: u16 = 100;

    // use try_into() for greater control of type conversion
    let b_ = b.try_into().unwrap();
    // we unwrap to handle successful Result monad and extract value
    if a < b_ {
        println!("Ten is less than one hundred.");
    }
}