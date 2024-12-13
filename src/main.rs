use serde::{Serialize, Deserialize};
use std::fs::{read_to_string, File};
use std::io::{self, Write};


#[derive(Serialize, Deserialize)]
struct Todo {
    name: String,
}

fn main() {              
    
    let mut todo = String::new();
    println!("New to-do?");
    io::stdin().read_line(&mut todo).expect("Read line failed.");
    
    let mut read_file: Vec<Todo> = match read_to_string("data.json") {
        Ok(data) => serde_json::from_str(&data).expect("Cannot deserialize file"),
        Err(_) => Vec::new(),
    };

    let todo  = todo.trim();

    if todo.contains("--delete") {
        let line_number:usize = todo.split_whitespace().last().expect("Can't find line with this number").parse().unwrap();
        read_file.remove(line_number - 1);
    } else {
        read_file.push(Todo { name: todo.to_string() });
    };

    let mut _data_file = File::create("data.json").expect("Cannot create file");
    let json_content = serde_json::to_string(&read_file).expect("Cannot write in file");

    _data_file.write_all(json_content.as_bytes()).expect("Cannot write in file");
}