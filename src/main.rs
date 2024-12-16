use clap::Parser;
use core::option::Option;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct Todo {
    name: String,
    status: bool,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    delete: Option<usize>,

    #[arg(short = 'D', long)]
    done: Option<usize>
}

fn save_todo(todos: &mut Vec<Todo>) {
    let mut todo = String::new();
        println!("New to-do?");
        io::stdin().read_line(&mut todo).expect("Read line failed.");
        todos.push(Todo {
            name: todo.trim().to_string(),
            status: false
        });
}

fn main() {
    
    let args = Args::parse();

    let mut read_file: Vec<Todo> = match read_to_string("data.json") {
        Ok(data) => serde_json::from_str(&data).expect("Cannot deserialize file"),
        Err(_) => Vec::new(),
    };

    if let Some(_todo_to_delete) = args.delete {
        if read_file.len() < _todo_to_delete {
                println!("Cannot delete this todos: todo doesn't exist")
        } else {
            read_file.remove(_todo_to_delete - 1);
        }
    } else if let Some(_todo_to_done) = args.done {
        if read_file.len() < _todo_to_done {
                println!("Cannot done this todos: todo doesn't exist")
        } else {
            read_file[_todo_to_done - 1].status = true
        }
    } else {
            save_todo(&mut read_file);
    }

    let mut _data_file = File::create("data.json").expect("Cannot create file");
    let json_content = serde_json::to_string(&read_file).expect("Cannot write in file");

    _data_file
        .write_all(json_content.as_bytes())
        .expect("Cannot write in file");
}