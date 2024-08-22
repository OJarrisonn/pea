use std::{error::Error, process::Command};

use config::Config;

mod config;

fn main() -> Result<(), Box<dyn Error>> {
    let mut config = Config::load()?.with_pager();

    if config.pager.is_none() {
        eprintln!("No pager found. Please set PAGER or PEA_PAGER environment variable.");
        std::process::exit(1);
    }

    let command = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    if command.is_empty() {
        eprintln!("Usage: pea <command> [args...]");
        std::process::exit(1);
    }

    
    let piped_commands = format!("{} | {}", command, config.pager.as_ref().unwrap());
    config.format_shell_args(&piped_commands);

    let status = Command::new(&config.shell)
        .args(&config.shell_args)
        .stdout(std::process::Stdio::inherit())
        .status()?;    

    std::process::exit(status.code().unwrap_or(1));
}
