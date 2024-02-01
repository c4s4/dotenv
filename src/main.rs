use clap::Parser;
use dotenv;
use std::process::Command;

/// Run command in environment loaded from dotenv file
#[derive(Parser)]
struct Cli {
    /// The dotenv file to load
    #[arg(short, long, default_value = ".env")]
    env: std::path::PathBuf,
    /// The command to run
    cmd: Vec<String>,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    // load environment from dotenv file
    dotenv::from_filename(&args.env).expect("error loading dotenv file");
    // check command is not empty
    if args.cmd.is_empty() {
        return;
    }
    // get command and arguments arguments
    let command = &args.cmd[0];
    let arguments = &args.cmd[1..];
    // run command
    Command::new(command).args(arguments).status().expect("failed to execute process");
}
