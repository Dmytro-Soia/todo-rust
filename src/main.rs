use chrono::naive::NaiveDate;
use clap::Parser;
use core::option::Option;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
struct Todo {
    name: String,
    status: bool,
    deadline: Option<NaiveDate>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    id: Option<usize>,

    #[arg(short, long)]
    delete: bool,

    #[arg(short = 'D', long)]
    done: bool,

    #[arg(short, long)]
    undone: bool,

    #[arg(long)]
    due: Option<String>,
}

fn save_todo(todos: &mut Vec<Todo>) {
    let mut todo = String::new();
    println!("New to-do?");
    io::stdin().read_line(&mut todo).expect("Read line failed.");
    todos.push(Todo {
        name: todo.trim().to_string(),
        status: false,
        deadline: None,
    });
}

fn main() {
    let args = Args::parse();

    let mut read_file: Vec<Todo> = match read_to_string("data.json") {
        Ok(data) => serde_json::from_str(&data).expect("Cannot deserialize file"),
        Err(_) => Vec::new(),
    };

    if let Some(id) = args.id {
        if args.delete {
            if read_file.len() < id {
                println!("Cannot delete this todos: todo doesn't exist")
            } else {
                read_file.remove(id - 1);
            }
        } else if args.done {
            if read_file.len() < id {
                println!("Cannot done this todos: todo doesn't exist")
            } else {
                read_file[id - 1].status = true
            }
        } else if args.undone {
            if read_file.len() < id {
                println!("Cannot undone this todos: todo doesn't exist")
            } else {
                read_file[id - 1].status = false
            }
        } else if let Some(due_date) = args.due {
            if read_file.len() < id {
                println!("Cannot date this todos: todo doesn't exist")
            } else {
                let format = "%y-%m-%d";
                let due_date =
                    NaiveDate::parse_from_str(&due_date, format).expect("Cannot write date");
                read_file[id - 1].deadline = Some(due_date);
            }
        } else {
            println!("Set an action!")
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
