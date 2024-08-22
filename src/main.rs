use std::collections::LinkedList;

fn main() {
    let shell = "sh -c";
    let pager = std::env::var("PEA_PAGER")
        .unwrap_or_else(|_| std::env::var("PAGER").unwrap_or_else(|_| "less".to_string()));

    let command = std::env::args().skip(1).collect::<Vec<_>>().join(" ");
    
    if command.is_empty() {
        eprintln!("Usage: pea <command> [args...]");
        std::process::exit(1);
    }

    println!("{} '{} | {}'", shell, command, pager);
}
