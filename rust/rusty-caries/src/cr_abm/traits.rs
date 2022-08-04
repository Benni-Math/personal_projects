// General file for multi-use Traits
use crate::{Error, Result};

pub trait Step {
    fn step(&mut self) -> Result<()> {}
}