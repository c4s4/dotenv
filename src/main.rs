use anyhow::{Context, Result};
use clap::Parser;
use std::env;
use std::process;
use std::process::Command;

const DEFAULT_SHELL_UNIX: &str = "sh";
const DEFAULT_SHELL_WINDOWS: &str = "cmd";

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
    /// Shell to run command in (default: sh on Unix or cmd on Windows)
    #[clap(short, long)]
    shell: Option<String>,
    /// The command to run
    cmd: Vec<String>,
}

fn main() {
    // parse command line arguments
    let args = Cli::parse();
    match run(args) {
        Ok(_) => println!("OK"),
        Err(e) => {
            eprintln!("ERROR {:#}", e);
            std::process::exit(1);
        }
    }
}

fn run(args: Cli) -> Result<()> {
    // check command is not empty
    if args.cmd.is_empty() {
        ()
    }
    // clear environment
    if args.clear {
        for (key, _) in env::vars().into_iter() {
            env::remove_var(key)
        }
    }
    // load environment from dotenv file
    for path in &args.env {
        load_dotenv_file(path)?;
    }
    // run command
    let exit_code = run_command(args.cmd, args.shell).context("running command")?;
    process::exit(exit_code);
}

/// Load environment from dotenv file
fn load_dotenv_file(path: &str) -> Result<()> {
    // load file content
    let content =
        std::fs::read_to_string(path).with_context(|| format!("loading dotenv file: {}", path))?;
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
    Ok(())
}

/// Run command
fn run_command(cmd: Vec<String>, shell: Option<String>) -> Result<i32> {
    // run command in shell
    if env::consts::OS != "windows" {
        // on unix
        let shell = shell.unwrap_or(DEFAULT_SHELL_UNIX.to_string());
        match Command::new(shell).arg("-c").arg(&cmd.join(" ")).status() {
            Ok(status) => return Ok(status.code().unwrap()),
            Err(err) => anyhow::bail!(err),
        }
    } else {
        // on windows
        let shell = shell.unwrap_or(DEFAULT_SHELL_WINDOWS.to_string());
        match Command::new(shell).arg("/c").arg(&cmd.join(" ")).status() {
            Ok(status) => return Ok(status.code().unwrap()),
            Err(err) => anyhow::bail!(err),
        }
    }
}
