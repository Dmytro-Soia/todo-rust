use chrono::naive::NaiveDate;
use clap::Parser;
use core::option::Option;
use serde::{Deserialize, Serialize};
use std::fs::{read_to_string, File};
use std::io::{self, Write};

#[derive(Serialize, Deserialize)]
//                                                Define the structure of our to-do
struct Todo {
    name: String,
    status: bool,
    deadline: Option<NaiveDate>,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
//                                              Define the arguments which we will use
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

    #[arg(short, long)]
    list: bool,

    #[arg(short, long)]
    sort: bool,
}
//                                                    Function that saves to-dos
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
//                   Reading and creation(if it does not exist) a vector in which to store data and deserialization the json file
    let mut read_file: Vec<Todo> = match read_to_string("data.json") {
        Ok(data) => serde_json::from_str(&data).expect("Cannot deserialize file"),
        Err(_) => Vec::new(),
    };
//                                                      Functions of each argument
    if let Some(id) = args.id {
        if args.delete {
            if read_file.len() < id {
                println!("Cannot delete this to-dos: to-do doesn't exist")
            } else {
                read_file.remove(id - 1);
            }
        } else if args.done {
            if read_file.len() < id {
                println!("Cannot done this to-dos: to-do doesn't exist")
            } else {
                read_file[id - 1].status = true
            }
        } else if args.undone {
            if read_file.len() < id {
                println!("Cannot undone this to-dos: to-do doesn't exist")
            } else {
                read_file[id - 1].status = false
            }
        } else if let Some(due_date) = args.due {
            if read_file.len() < id {
                println!("Cannot date this to-dos: to-do doesn't exist")
            } else {
                let format = "%y-%m-%d";
                let due_date =
                    NaiveDate::parse_from_str(&due_date, format).expect("Cannot parse date");
                read_file[id - 1].deadline = Some(due_date);
            }
        } else {
            println!("Set an action!")
        }
    } else if args.list {
        if args.sort {
            read_file.sort_by(|a, b| a.deadline.cmp(&b.deadline));
        }
        for todo in read_file.iter() {
            let status = match todo.status {
                true => "Done",
                false => "Undone",
            };
            let due = match todo.deadline {
                Some(date) => date.format("%y-%m-%d").to_string(),
                None => "No due date".to_string(),
            };
            println!("{} - {} - {}", todo.name, status, due)
        }
    } else {
        save_todo(&mut read_file);
    }
//                                              Creating json file and seralization it
    let mut _data_file = File::create("data.json").expect("Cannot create file");
    let json_content = serde_json::to_string(&read_file).expect("Cannot write in file");
//                                                  Writing to-dos into json file
    _data_file
        .write_all(json_content.as_bytes())
        .expect("Cannot write in file");
}
