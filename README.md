# To-Do List

Simple to-do list which works using the command line.

## Installation
## Rust
### macOS, Linux, or another Unix-like OS
To download Rustup and install Rust, run the following in your terminal, then follow the on-screen instructions. 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
### Windows
Download and run [rustup-init.exe](https://static.rust-lang.org/rustup/dist/i686-pc-windows-gnu/rustup-init.exe)

`rustup-init` can be configured interactively, and all options can additionally be controlled by command-line arguments, which can be passed through the shell script. Pass `--help` to `rustup-init` as follows to display the arguments rustup-init accepts:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --help
```

For more information: [use this documentation](https://forge.rust-lang.org/infra/other-installation-methods.html)

## Project
After installing rust, you need to download this project by using this command:
```bash
git clone git@github.com:Dmytro-Soia/todo-rust.git
```
Then you need to go to the program directory:
```bash
cd todo-rust
```
# Technical documentation
* **serde**  -  is used to serialize and deserialize a JSON file that i used to store data.
* **lap**  -  used to more easily create flags(commands) that will later be used in the command line.
* **chrono**  -   used for easier work with time