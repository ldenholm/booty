fn main() {
    let a = 42;
    let r = &a;
    let sum = a + *r;
    println!("sum of a + a: {sum}");

    let needle = 42;
    let haystack = [1, 1, 2, 5, 15, 42, 203, 877, 4140, 21147];
    
    for item in &haystack {
        if *item == needle {
            println!("{item}");
        }
    }
}
