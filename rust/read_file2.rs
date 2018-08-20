use std::fs::File;
use std::io::prelude::*;

fn read_file2() {
    let file_result = File::open("this/file/does/not/exist.nope");
    let mut contents = String::new();

    match file_result {
        Ok(mut file) => file.read_to_string(&mut contents),
        Err(e) => panic!("{}", e),
    };

    println!("{}", contents);
}


fn main() {
    read_file2();
}
