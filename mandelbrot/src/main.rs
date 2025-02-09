use num::{range, Complex};
/*  how to construct mandelbrot set.
        first consider a dynamical system: (T, M, X)
        T a group, M a set, X the evolution function

        ^ disregard this aspect for now, lets focus on constructing
        the complex valued function f_c(z) = z^2 + c and test
        divergence as we iterate c from zero.

        Once we have this function defined we can search for elements
        of the mandelbrot set. Those elements are the results which
        are bounded and do not grow unboundedly.

        We will also need to construct the complex plane (output space)
        which we will use to render the elements of the mandelbrot set (z vs c?).
*/

/// Initial quadratic map to test divergence of some complex number z
fn complex_quadratic_map(z: i128, c: i128) {
    let result = z * z + c;
    println!("result is: {result}");
    if z > 1000000000000 {
        return;
    } else {
        return complex_quadratic_map(result, c);
    }

    // Is in mandelbrot if ||z|| <= 2, l2 norm
}

/// Accepts x,y and maximum iterations
/// and determines if the point is in Mandelbrot
fn determine_mandelbrot_at_point(cx: f64, cy: f64, max_iters: usize) -> usize {
    // create complex num at origin
    let mut z = Complex { re: 0.0, im: 0.0 };
    let c = Complex::new(cx, cy);
    for i in 0..=max_iters {
        if z.norm() > 2.0 {
            // Element not in Mandelbrot set, escape iteration.
            return i;
        }
        // Iterate for max_iters to find Mandelbrot members
        // using this recurrence relation:
        z = z * z + c;
    }
    max_iters
}

// We are finding both Mandelbrot members and non-members
// and encoding a visual symbol to represent them. The symbols
// will represent how quickly they diverge to infinity, and
// be rendered to the console, hence we will see something
// pretty as we discover them.

/// Params describe search space (x and y) and
/// the output space (height/width). Search the
/// provided space for Mandelbrot members and
/// build matrix representation of the output space.
/// Aka this is a complex space on which we will plot
/// non-Mandelbrot elements using symbolic encoding
/// that represents how quickly they diverged.
fn calculate_mandelbrot_grid(
    max_iters: usize,
    x_min: f64,
    x_max: f64,
    y_min: f64,
    y_max: f64,
    width: usize,
    height: usize,
) -> Vec<Vec<usize>> {
    // build the grid (output space)
    let mut rows: Vec<_> = Vec::with_capacity(width);
    for img_y in 0..height {
        // Print row by row as we find members
        let mut row: Vec<usize> = Vec::with_capacity(height);
        for img_x in 0..width {
            // get output space scaling percentages
            let x_perc = img_x as f64 / width as f64;
            let y_perc = img_y as f64 / height as f64;
            // next construct our complex number by setting
            // the x and y equal to a point in the search space
            // relative to the output space
            let cx = x_min + (x_max - x_min) * x_perc;
            let cy = y_min + (y_max - y_min) * y_perc;
            // find number of iterations to escape mandelbrot constraint
            let escaped_at = determine_mandelbrot_at_point(cx, cy, max_iters);
            row.push(escaped_at);
        }
        rows.push(row);
    }
    // Essentially we're returning a 2x2 grid of varying integers
    // which represent the number of iterations the current complex
    // num z took to escape the chosen threshold (<=2).
    rows
}

fn render_mandelbrot(escape_vals: Vec<Vec<usize>>) {
    for row in escape_vals {
        // build and print line by line
        let mut line = String::with_capacity(row.len());
        for column in row {
            let val = match column {
                // encoding for ranges of iterations
                // it took to escape mandelbrot bound |z| <= 2
                0..=2 => ' ',
                3..=5 => '.',
                6..=10 => '•',
                11..=30 => '*',
                31..=100 => '+',
                101..=200 => 'x',
                201..=400 => '$',
                401..=700 => '#',
                _ => '%',
            };
            //print!("val is: {}", val);
            line.push(val);
        }
        println!("{}", line);
    }
}

fn main() {
    let mandelbrot = calculate_mandelbrot_grid(
        2500, -2.0, 4.0, -1.0, 1.0, 120, 40);

        render_mandelbrot(mandelbrot);
}