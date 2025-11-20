use std::env;
use std::io::{self, Write};
use std::process::Command;

fn main() {
    println!("...Welcome to our mini shell...");

    loop {
        print!("myminishell> ");
        io::stdout().flush().unwrap();
        let mut input: String = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Failed to read input");
            continue;
        }

        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let inputs: Vec<&str> = input.split_whitespace().collect();
        match inputs[0] {
            "help" => show_help(),
            "exit" => {
                println!("Exiting...");
                break;
            }
            "cd" => {
                let dir = inputs.get(1).copied();
                change_directory(dir);
            }
            "pwd" => print_current_directory(),
            _ => run_external_command(inputs[0], &inputs[1..]),
        }
    }
}

pub fn show_help() {
    println!("Following commands are available!!!!");
    println!("help\nexit\ncd <dir>\npwd");
}

pub fn change_directory(directory: Option<&str>) {
    match directory {
        Some(dir) => {
            if let Err(e) = env::set_current_dir(dir) {
                eprintln!("cd: {}", e);
            }
        }
        None => {
            eprintln!("cd: missing argument");
        }
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

pub fn run_external_command(cmd: &str, args: &[&str]) {
    let output = Command::new(cmd).args(args).output();

    match output {
        Ok(out) => {
            if !out.stdout.is_empty() {
                print!("{}", String::from_utf8_lossy(&out.stdout));
            }
            if !out.stderr.is_empty() {
                eprint!("{}", String::from_utf8_lossy(&out.stderr));
            }
        }
        Err(e) => eprintln!("{}: command not found ({})", cmd, e),
    }
}
