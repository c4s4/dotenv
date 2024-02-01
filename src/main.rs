use clap::Parser;
use dotenv;
use std::env;
use std::process;
use std::process::Command;

const VERSION: &str = env!("RUNENV_VERSION");

/// Run command in environment loaded from dotenv file
#[derive(Parser)]
struct Cli {
    /// The runenv version
    #[arg(short, long)]
    version: bool,
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
    // print version and exit
    if args.version {
        println!("{}", VERSION);
        return
    }
    // clear environment
    if args.clear {
        for (key, _) in env::vars().into_iter() {
            env::remove_var(key)
        }
    }
    // load environment from dotenv file
    for path in &args.env {
        if let Err(err) = dotenv::from_filename(&path) {
            eprintln!("ERROR loading dotenv file '{}': {err}", &path);
            process::exit(1);
        }
    }
    // check command is not empty
    if args.cmd.is_empty() {
        return;
    }
    if args.shell {
        // run command
        if env::consts::OS == "windows" {
            // on windows
            if let Err(err) = Command::new("cmd")
                .arg("/c")
                .arg(&args.cmd.join(" "))
                .status()
            {
                eprintln!("ERROR running command: {err}");
                process::exit(1);
            };
        } else {
            // on unix
            if let Err(err) = Command::new("sh")
                .arg("-c")
                .arg(&args.cmd.join(" "))
                .status()
            {
                eprintln!("ERROR running command: {err}");
                process::exit(1);
            };
        }
    } else {
        // run command
        if let Err(err) = Command::new(&args.cmd[0]).args(&args.cmd[1..]).status() {
            eprintln!("ERROR running command: {err}");
            process::exit(1);
        };
    }
}
