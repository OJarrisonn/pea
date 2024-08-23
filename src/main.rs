use std::{error::Error, process::Command};

use config::Config;

mod config;

fn main() -> Result<(), Box<dyn Error>> {
    let config = Config::load()?;

    let command = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    if command.is_empty() {
        eprintln!("Usage: pea <command> [args...]");
        std::process::exit(1);
    }

    let piped_commands = format!("{} | {}", command, config.pager());

    let status = Command::new(config.shell())
        .args(config.shell_args(&piped_commands))
        .stdout(std::process::Stdio::inherit())
        .status()?;

    std::process::exit(status.code().unwrap_or(1));
}
