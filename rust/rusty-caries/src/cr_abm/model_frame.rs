mod agent;
mod data_dict;
mod sequences;

pub struct ModelFrame {
    name: String,
    params: HashMap<String, f64>,
}