use std::{
    fs::File,
    io::{Error, Read},
};

fn main() -> Result<(), Error> {
    let mut file = File::open("/etc/hosts")?;
    let mut text = String::new();
    file.read_to_string(&mut text)?;
    println!("{}", text);

    Ok(())
}

