use std::collections::btree_map::Range;

fn main() {
    let mut myarr: Vec<i8> = vec![];
    /* myarr[0] = 5;
       myarr[1] = 10;
       this is wrong we need to push
    */
    // let's construct sequence of 2n for 1= < n <= 5.
    // note: 1..5 = [1) while 1..=5 = [1,5].
    for n in 1..=5 {
        myarr.push(n * 2);
    }

    println!("length of myarr = {}", myarr.len());

    for x in myarr {
        print!("{}", x);
        print!("\n");
    }
}
