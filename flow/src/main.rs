use std::ops::MulAssign;


fn main() {
    let mut myarr: Vec<i8> = vec![];
    let mut t: Vec<i8> = vec![];
    /* myarr[0] = 5;
       myarr[1] = 10;
       this is wrong we need to push
    */
    // let's construct sequence of 2n for 1= < n <= 5.
    // note: 1..5 = [1) while 1..=5 = [1,5].
    for n in 1..=5 {
        myarr.push(n * 2);
    }

    t.push(1);
    t.push(2);
    t.push(3);

    for x in &t {
        // do something with x
        print!("{x}\n");
    }

    // now container t should not be accessible twice

    for x in &mut t {
        println!("We may access container t twice when using a reference.");
        //x *= 2; // illegal since we don't use mut reference.

        // by invoking &mut t we can now mutate our collection elements:
        x.mul_assign(2);
    }

    for x in 0..t.len() {
        println!("value of idx: {}", x);
        println!("{}", t[x]);
        println!("\n");
    }

    // Above block fails to compile. We can change the first
    // for loop to use a reference to the container &t allowing
    // us to loop over the container n times.

    println!("length of myarr = {}", myarr.len());

    for x in myarr {
        print!("{x}");
        print!("\n");
    }
}
