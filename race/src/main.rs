use std::thread;
fn main() {
    let mut data = 100;

    thread::spawn(move || { data = 500; println!("{}", data);}); // attempt to set data to 500
    thread::spawn(move || {data = 1000; println!("{}", data);}); // // attempt to set data to 1000
    println!("{}", data);
    // OS spawns threads not the program, its impossible to know which
    // AOT the value of data. Rust only allows a single writer to data.


    // example of overflow:
    buffer_overflow();
}


fn buffer_overflow() {
    let symbols = vec!["delta", "phi", "sigma"];
    let out_of_bounds_idx = symbols[4];
    assert_eq!(out_of_bounds_idx, "gamma");
}

fn iterator_invalidation() {
    // mutate an iterator while iterating through it
    let mut symbols = vec!["delta", "phi", "sigma"];
    for s in symbols {
        println!("{}", s);
        symbols.push(s.clone());
    }

    // Doesn't compile since Rust disallows iterator to be modified
    // within a for block
}