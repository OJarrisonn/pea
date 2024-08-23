use std::process::Command;

use anyhow::{bail, Context, Result};
use config::Config;

mod config;

fn main() -> Result<()> {
    let config = Config::load()?;

    let command = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    if command.is_empty() {
        bail!("Usage: pea <command> [args...]");
    }

    let piped_commands = format!("{} | {}", command, config.pager());

    let status = Command::new(config.shell())
        .args(config.shell_args(&piped_commands))
        .stdout(std::process::Stdio::inherit())
        .status()
        .with_context(|| format!("Failed to run shell `{}`", config.shell()))?;

    std::process::exit(status.code().unwrap_or(1));
}
