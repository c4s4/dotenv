use clap::Parser;
use std::env;
use std::process;
use std::process::Command;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Run command in environment loaded from dotenv file
#[derive(Parser)]
struct Cli {
    /// The dotenv version
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
        return;
    }
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
            error(&format!("loading dotenv file: {err}"));
            return;
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
            let result = Command::new("cmd").arg("/c").arg(&cmd.join(" ")).status();
            if result.as_ref().is_err()  {
                error(&format!("running command: {}", result.as_ref().err().unwrap()));
            };
            return result.unwrap().code().unwrap();
        } else {
            // on unix
            let result = Command::new("sh").arg("-c").arg(&cmd.join(" ")).status();
            if result.as_ref().is_err()  {
                error(&format!("running command: {}", result.as_ref().err().unwrap()));
            };
            return result.unwrap().code().unwrap();
        }
    } else {
        // run command
        let result = Command::new(&cmd[0]).args(&cmd[1..]).status();
        if result.as_ref().is_err()  {
            error(&format!("running command: {}", result.as_ref().err().unwrap()));
        };
        return result.unwrap().code().unwrap();
    }
}

/// Print error message and exit
fn error(msg: &str) {
    eprintln!("ERROR {msg}");
    process::exit(1);
}
