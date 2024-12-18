# How to use?

To write your first to-do you simply need to write `cargo run` in terminal and press Enter. After you write your first to-do, program will create a JSON file where will be stocked all yours to-dos.
## Flags
In order to perform actions and manipulate your to-dos, you must use "flags".

Also, before each flag, you must write another one, in which you specify the to-dos "id"
```bash
cargo run -- --id <number of selected to-do> "--flag"
```
---
Here is a list of all flags
```bash
--delete = delete the flag 

--done = mark to-do as completed

--undone = mark to-do as uncompleted

--due = added a deadline in format "YY-MM-DD"

--list = display all yours to-dos
```
Also, when using the "--list flag", you can sort all to-dos by date by adding the "-- sort" flag
```bash
--list --sort
```
## Examples
```bash
cargo run
cargo run -- --id 1 --delete
cargo run -- --id 1 --done
cargo run -- --id 1 --undone
cargo run -- --id 1 --due 12-12-12
cargo run -- --list
cargo run -- --list --sort
```