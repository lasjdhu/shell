#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::Command;
use std::{env, path};

fn main() {
    let mut input = String::new();
    let mut exit = false;

    while !exit {
        get_command(&mut input, &mut exit);
    }
}

fn get_command(input: &mut String, exit: &mut bool) {
    print!("$ ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(input).unwrap();

    let trimmed_input = input.trim();
    let command_option = trimmed_input.split_once(" ");

    if trimmed_input.is_empty() {
        input.clear();
        return;
    }

    parse_command(trimmed_input, command_option, exit);

    input.clear();
    return;
}

fn parse_command(trimmed_input: &str, command_option: Option<(&str, &str)>, exit: &mut bool) {
    let builtin_list: Vec<&str> = vec!["exit", "type", "cd", "echo", "pwd"];

    if let Some((command, rest)) = command_option {
        match command {
            "exit" => {
                if rest == "0" {
                    *exit = true;
                } else {
                    println!("{}: command not found", command);
                }
            }
            "type" => {
                let found_in_builtin = builtin_list.contains(&rest);

                if found_in_builtin {
                    println!("{} is a shell builtin", rest);
                } else if let Some(path) = find_in_path(rest) {
                    println!("{} is {}", rest, path);
                } else {
                    println!("{}: not found", rest);
                }
            }
            "cd" => {
                let mut target_dir = rest.to_string();
                if target_dir == "~" {
                    if let Ok(home_dir) = env::var("HOME") {
                        target_dir = home_dir;
                    }
                }
                if let Err(_e) = env::set_current_dir(&target_dir) {
                    println!("{}: No such file or directory", rest);
                }
            }
            "echo" => {
                println!("{}", rest);
            }
            _ => {
                if let Some(_path) = find_in_path(command) {
                    run_command(command, rest);
                } else {
                    println!("{}: command not found", command);
                }
            }
        }
    } else if trimmed_input == "echo" {
        println!();
    } else if trimmed_input == "pwd" {
        let current_dir = env::current_dir().unwrap();
        println!("{}", current_dir.display());
    } else if trimmed_input == "cd" {
        if let Ok(home_dir) = env::var("HOME") {
            if let Err(_e) = env::set_current_dir(&home_dir) {
                println!("{}: No such file or directory", trimmed_input);
            }
        }
    } else if trimmed_input == "type" {
        println!("type: argument is required");
    } else if let Some(_path) = find_in_path(trimmed_input) {
        run_command(trimmed_input, "");
    } else {
        println!("{}: command not found", trimmed_input);
    }
}

fn find_in_path(command: &str) -> Option<String> {
    if let Ok(path) = env::var("PATH") {
        for dir in path.split(':') {
            let full_path = format!("{}/{}", dir, command);
            if path::Path::new(&full_path).exists() {
                return Some(full_path);
            }
        }
    }
    None
}

fn run_command(command: &str, args: &str) {
    match Command::new(command).args(args.split_whitespace()).spawn() {
        Ok(mut child) => {
            child.wait().expect("Command wasn't running");
        }
        Err(e) => println!("Failed to execute {}: {}", command, e),
    }
}
