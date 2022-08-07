pub use self::gaussian::*;

use crate::genetic_algorithm::*;

mod gaussian;

pub trait MutationMethod {
    fn mutate(&self, rng: &mut dyn RngCore, child: &mut Chromosome);
}