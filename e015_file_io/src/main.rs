use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

fn main() {
    let path = Path::new("output.txt");

    let mut file = match File::create(&path) {
        Err(why) => panic!("Couldn't create {}: {}", path.display(), why.description()),
        Ok(file) => file,
    };

    let lorem_ipsum: &str = "Lorem ipsum dolor sit amet, consectetur adipiscing elit. 
    Phasellus sem orci, pellentesque quis arcu nec, pharetra cursus tortor. Sed orci eros, porta id ante sed.";

    match file.write_all(lorem_ipsum.as_bytes()) {
        Err(why) => panic!("Couldn't write {}: {}", path.display(), why.description()),
        Ok(_) => println!("Successfully wrote"),
    }
}
