use std::fs;

fn main() {
    println!("Getting filenames");

    let paths = fs::read_dir("data").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display());
    }

}
