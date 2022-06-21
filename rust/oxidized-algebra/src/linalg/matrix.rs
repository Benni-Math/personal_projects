// To support higher efficiency, I want to try implementing as one vector
// I will need to look into how the Rust compiler optimizes certain numerical
// operations, along with what I can do with pointers.

#[derive(Debug)]
pub struct matrix {
    // Dimension is 'row x column', i.e. mxn
    dimensions: (usize, usize),
    // Currently only support float64
    // Need to check for appropriate 'arithmetic' trait bounds
    entries: Vec<f64>
}

// TODO: look into operator overloading
impl matrix {
    pub fn default() -> Self {
        matrix {
            dimensions: (0, 0),
            entries: Vec::new()
        }
    }

    // TODO: how do I reference a slice of an array?
    // there is a sizing issue - may want to use Vec<Vec<f64>>
    // pub fn new(entries: &[[f64]]) -> Self {}

    pub fn id(dimension: usize) -> Self {
        let mut identity = Vec::new();

        matrix {
            dimensions: (dimension, dimension),
            entries: identity,
        }
    }
}
