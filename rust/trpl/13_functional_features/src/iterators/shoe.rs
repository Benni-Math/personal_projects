#[derive(PartialEq, Debug)]
pub struct Shoe {
    pub size: u32,
    pub style: String,
}

pub fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    // use of the iterator.filter() method
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}
