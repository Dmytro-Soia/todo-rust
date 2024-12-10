use std::{fs::File, io,};
use std::io::Write;
fn main() {
    let mut name = String::new();
    println!("New to-do?");
    io::stdin().read_line(&mut name).expect("Read line failed.");

    let mut data_file = File::create("data.txt").expect("Cannot create file");
    data_file.write(name.as_bytes()).expect("Cannot write in file");
}
