// This is envisioned as a general purpose ABM library.
mod model_frame;
mod experiment;
mod optimizer;
mod helpers;
mod traits;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[cfg(test)]
mod tests {

}
