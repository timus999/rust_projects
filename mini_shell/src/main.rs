use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;

fn main() {
    println!("...Welcome to our mini shell...");

    loop {
        print!("myminishell> ");
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read the input");
        let input = input.trim();
        let inputs: Vec<&str> = input.split_whitespace().collect();
        match inputs[0] {
            "help" => show_help(),
            "exit" => {
                println!("Exiting...");
                break;
            }
            "cd" => change_directory(inputs[1]),
            "pwd" => print_current_directory(),
            _ => {
                if inputs.len() > 1 {
                    run_external_command(inputs[0], Some(inputs[1]))
                } else {
                    run_external_command(inputs[0], None)
                }
            }
        }
    }
}

pub fn show_help() {
    println!("Following commands are available!!!!");
    println!("help\nexit\ncd <dir>\npwd");
}

pub fn change_directory(directory: &str) {
    let mut path = env::current_dir().expect("could not get the current dir");

    path.push(directory);

    if let Err(e) = env::set_current_dir(&path) {
        eprintln!("failed to change directory: {} ", e);
    } else {
        println!("Changed directory to {:?}", path);
    }
}

pub fn print_current_directory() {
    let output = Command::new("pwd")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        println!("{}", output_str);
    }
}

pub fn run_external_command(cmd: &str, args: Option<&str>) {
    if let Some(args) = args {
        let output = Command::new(cmd)
            .arg(args)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            println!("{}", output_str);
        }
    } else {
        let output = Command::new(cmd)
            .output()
            .expect("Failed to execute command");

        if output.status.success() {
            let output_str = String::from_utf8_lossy(&output.stdout);
            println!("{}", output_str);
        }
    }
}
