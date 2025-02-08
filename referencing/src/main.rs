fn main() {
    let a = 42; // dec and assignment, a has val 42
    let r = &a; // r is a ref to a, using '&' ref operator
    let sum = a + *r;
    let deref = *r;
    println!("sum of a + a: {sum}");
    println!("is dereferencing destructive?: {deref}. No.");

    let needle = 42;
    let haystack = [1, 1, 2, 5, 15, 42, 203, 877, 4140, 21147];
    
    for item in &haystack {
        if *item == needle {
            println!("{item}");
        }
    }

    for something in haystack {
        println!("something is: {something}");
    }

    for another_thing in &haystack {
        let test = *another_thing;
        let cool = another_thing;
        println!("anotherThing is: {another_thing}");
        println!("test is: {test}");
        println!("cool is: {cool}");
    }

    /*
        let's talk about whats going on here


     */
}
