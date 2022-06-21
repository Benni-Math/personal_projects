// To support higher efficiency, I want to try implementing as one vector
// I will need to look into how the Rust compiler optimizes certain numerical
// operations, along with what I can do with pointers.

#[derive(Debug)]
pub struct matrix {
    // Dimension is 'row x column', i.e. mxn
    dimension: (usize, usize),
    // Currently only support float64
    // Need to check for appropriate 'arithmetic' trait bounds
    entries: Vec<f64>
}

impl matrix {
    
}
