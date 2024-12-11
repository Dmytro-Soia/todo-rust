use std::fs::OpenOptions;
use std::io;
use std::io::Write;
fn main() {
    let mut name = String::new();
    println!("New to-do?");
    io::stdin().read_line(&mut name).expect("Read line failed.");

    let mut todo_file = OpenOptions::new()
    .create(true)
    .read(true)
    .write(true)
    .append(true)
    .open("data.txt").expect("Cannot open file");
    
    todo_file.write_all(name.as_bytes()).expect("Cannot write in file");
}
