use std::fs::File;
use std::io::prelude::*;

fn read_file() {
    let file = File::open("this/file/does/not/exist.nope");
    let mut contents = String::new();

    file.read_to_string(&mut contents);

    println!("{}", contents);
}


fn main() {
    read_file();
}
