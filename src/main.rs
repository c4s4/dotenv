use clap::Parser;
use dotenv;
use std::env;
use std::process;
use std::process::Command;

/// Run command in environment loaded from dotenv file
#[derive(Parser)]
struct Cli {
    /// The dotenv file to load
    #[arg(short, long, default_value = ".env")]
    env: std::path::PathBuf,
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
    if let Err(err) = dotenv::from_filename(&args.env) {
        eprintln!("ERROR loading dotenv file '{}': {err}", &args.env.display());
        process::exit(1);
    }
    // check command is not empty
    if args.cmd.is_empty() {
        return;
    }
    if args.shell {
        // run command
        if let Err(err) = Command::new("sh")
            .arg("-c")
            .arg(&args.cmd.join(" "))
            .status() {
                eprintln!("ERROR running command: {err}");
                process::exit(1);
            };
    } else {
        // run command
        if let Err(err) = Command::new(&args.cmd[0])
            .args(&args.cmd[1..])
            .status() {
                eprintln!("ERROR running command: {err}");
                process::exit(1);
            };
    }
}
