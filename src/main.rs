use std::env::args;
use std::fs;
use std::fs::{read_to_string,  OpenOptions};
use std::io;

fn main() {
    let mut todo = String::new();
    println!("New to-do?");
    io::stdin().read_line(&mut todo).expect("Read line failed.");

    let mut read_file: Vec<String> = match read_to_string("data.txt") {
        Ok(data) => data.lines().map(String::from).collect(),
        Err(_) => Vec::new(),
    };

    if todo.contains("--delete") {
        let line_number:usize = todo.split_whitespace().last().expect("Can't find line with this number").parse().unwrap();
        read_file.remove(line_number - 1);
    } else {
        read_file.push(todo.to_string());
    };

     
    let _todo_file = OpenOptions::new()
    .create(true)
    .read(true)
    .append(true)
    .open("data.txt").expect("Cannot open file");

    fs::write("data.txt", read_file.join("\n")).expect("Err");
}