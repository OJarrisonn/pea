<h1 align="center">Page 'em All</h1>

<div align="center">
    <img src="https://img.shields.io/badge/RUST-E67C19.svg?style=for-the-badge&logo=Rust&logoColor=white" alt="Rust" />
    <img alt="GitHub License" src="https://img.shields.io/github/license/OJarrisonn/pea?style=for-the-badge">
    <img alt="GitHub forks" src="https://img.shields.io/github/forks/OJarrisonn/pea?style=for-the-badge">
    <img alt="GitHub Repo stars" src="https://img.shields.io/github/stars/OJarrisonn/pea?style=for-the-badge">
</div>

`pea` is a CLI tool that helps users to page the output of every command.

## What is paging?

Some commands may output really long texts that might be hard to navigate or even extrapolate the terminal line limit. Paging stores the content in a separate buffer for the terminal that is easier to navigate and won't polute the terminal output.

The most used pagers are probably `less` and [`bat`](https://github.com/sharkdp/bat), and those are the recommended pagers to be used with `pea` (`less` is the default one).

## Installation

Currently the only installation method is by building from source

```sh
$ cargo install --git https://github.com/OJarrisonn/pea.git
```

## Usage

`pea` comes with a config file at `~/.config/pea/config.toml` with the following structure:

```toml
shell = "sh"
shell_args = ["-c", ""]
pager = "less"
```

This config file will use `less` as the pager and `sh` as the shell to run your command. To actually make use of `pea` type, for instance:

```sh
$ pea ls -halF
```

This will execute `ls -halF` and page the output in a nice way (if the output overflows your current terminal window). 

You might notice some commands output without colors, this has nothing (directly) to do with `pea` but each program implementation. Most CLI tools with colored output disable colors when the output is piped to another command (`pea` do pipe). So check if your command has a `--color=always` option. This may let `pea` page colored. Also check if your pager do support colors (`less` don't).

## Configuring PEA

Just to mention that the `pager` config may be ommited. If so, `pea` will try to use `$PEA_PAGER` if set, otherwise `$PAGER` and if this is also not set will fallback to `less`. So the precedence order is `config.pager` > `$PEA_PAGER` > `$PAGER` > `less`

In the `shell_args` list, the empty arg `""` shows where the command you've written shall be inserted.

Here you have some recommended configurations:

### Bash

```toml
shell = "bash"
shell_args = ["-c", ""]
```

### Nushell

Some additional parameters are needed to actually make use of your `config.nu` and `env.nu`

```toml
shell = "nu"
shell_args = ["-c", "", "--config", "/path/to/your/config.nu", "--env-config", "/path/to/your/env.nu"]
```

### Less

```toml
pager = "less"
```

### Bat

```toml
pager = "bat -p"
```

## Future Plans

- **Aliases**: support custom aliases in `pea` to always use flags like `--color=always` for given commands
- **Pre-process statements**: allow the definition of a preprocess command to run before paging the output
- Built-in pager