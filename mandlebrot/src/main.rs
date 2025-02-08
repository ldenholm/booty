use num::Complex;
fn main() {
    /*  how to construct mandlebrot set.
        first consider a dynamical system: (T, M, X)
        T a group, M a set, X the evolution function

        ^ disregard this aspect for now, lets focus on constructing
        the complex valued function f_c(z) = z^2 + c and test
        divergence as we iterate c from zero.

        Once we have this function defined we can search for elements
        of the Mandlebrot set. Those elements are the results which
        are bounded and do not grow unboundedly.

        We will also need to construct the complex plan which we will
        use to render the elements of the mandlebrot set (z vs c?).
    
    */

    complex_quadratic_map(0, 1);
}


fn complex_quadratic_map(z: i128, c: i128) {
    let result = z*z + c;
    println!("result is: {result}");
    if z > 1000000000000 { return }
    else {
        return complex_quadratic_map(result , c);
    }
}