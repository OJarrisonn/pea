use std::{
    io::Write,
    process::{Command, Stdio},
};

use anyhow::{bail, Result};
use config::Config;

mod config;

fn main() -> Result<()> {
    let config = Config::load()?;

    let command = std::env::args().skip(1).collect::<Vec<_>>().join(" ");

    if command.is_empty() {
        bail!("Usage: pea <command> [args...]");
    }

    let target = Command::new(config.shell())
        .args(config.shell_args(&command))
        .stdout(Stdio::piped())
        .spawn()?;

    let target_stdout = target.wait_with_output()?;
    let target_output = String::from_utf8_lossy(&target_stdout.stdout);

    let mut pager = Command::new(config.shell())
        .args(config.shell_args(&config.pager()))
        .stdin(Stdio::piped())
        .stdout(Stdio::inherit())
        .spawn()?;

    if let Some(mut stdin) = pager.stdin.take() {
        stdin.write_all(&target_output.as_bytes())?;
    }

    let status = pager.wait()?;

    std::process::exit(status.code().unwrap_or(1));
    //Ok(())
}
