use clap::Parser;
use std::env;
use std::process;
use std::process::Command;

/// Run command in environment loaded from dotenv file
#[derive(Parser)]
#[command(version)]
struct Cli {
    /// The dotenv file to load
    #[arg(short, long, default_value = ".env")]
    env: Vec<String>,
    /// Clear environment before loading env file
    #[arg(short, long, default_value_t = false)]
    clear: bool,
    /// Run command in a shell
    #[arg(short, long, default_value_t = false)]
    shell: bool,
    /// The command to run
    cmd: Vec<String>,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    // clear environment
    if args.clear {
        for (key, _) in env::vars().into_iter() {
            env::remove_var(key)
        }
    }
    // load environment from dotenv file
    for path in &args.env {
        load_dotenv_file(path);
    }
    // check command is not empty
    if args.cmd.is_empty() {
        return;
    }
    // run command
    process::exit(run_command(args.cmd, args.shell));
}

/// Load environment from dotenv file
fn load_dotenv_file(path: &str) {
    // load file content
    let content = match std::fs::read_to_string(path) {
        Ok(content) => content,
        Err(err) => {
            eprintln!("ERROR loading dotenv file: {err}");
            process::exit(1);
        }
    };
    // parse file content
    for line in content.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let key = line.splitn(2, '=').next().unwrap().trim();
        let value = line.splitn(2, '=').nth(1).unwrap().trim();
        env::set_var(key, value);
    }
}

/// Run command
fn run_command(cmd: Vec<String>, shell: bool) -> i32 {
    if shell {
        // run command
        if env::consts::OS == "windows" {
            // on windows
            match Command::new("cmd").arg("/c").arg(&cmd.join(" ")).status() {
                Ok(status) => return status.code().unwrap(),
                Err(err) => {
                    eprintln!("ERROR running command: {err}");
                    process::exit(1);
                }
            }
        } else {
            // on unix
            match Command::new("sh").arg("-c").arg(&cmd.join(" ")).status() {
                Ok(status) => return status.code().unwrap(),
                Err(err) => {
                    eprintln!("ERROR running command: {err}");
                    process::exit(1);
                }
            }
        }
    } else {
        // run command
        match Command::new(&cmd[0]).args(&cmd[1..]).status() {
            Ok(status) => return status.code().unwrap(),
            Err(err) => {
                eprintln!("ERROR running command: {err}");
                process::exit(1);
            }
        }
    }
}
