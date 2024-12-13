use clap::Parser;
use core::option::Option;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct Todo {
    name: String,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    delete: Option<usize>,
}

fn main() {
    let args = Args::parse();

    let mut read_file: Vec<Todo> = match read_to_string("data.json") {
        Ok(data) => serde_json::from_str(&data).expect("Cannot deserialize file"),
        Err(_) => Vec::new(),
    };

    match args.delete {
        None => {
            let mut todo = String::new();
            println!("New to-do?");
            io::stdin().read_line(&mut todo).expect("Read line failed.");
            read_file.push(Todo {
                name: todo.trim().to_string(),
            });
        }
        Some(_todo_to_delete) => {
            if read_file.len() < _todo_to_delete {
                println!("Cannot delete line: line doesn't exist")
            } else {
                read_file.remove(_todo_to_delete - 1);
            }
        }
    }

    let mut _data_file = File::create("data.json").expect("Cannot create file");
    let json_content = serde_json::to_string(&read_file).expect("Cannot write in file");

    _data_file
        .write_all(json_content.as_bytes())
        .expect("Cannot write in file");
}
