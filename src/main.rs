use std::{env, fs, process};
use unit_converter_parser::convert_units;

fn show_help() {
    println!("Usage: unit-converter [command] [options]");
    println!();
    println!("Commands:");
    println!("  parse <file>   Parse the specified file for unit conversions.");
    println!("  help           Show this help message.");
}

fn parse_file(file_path: &str) {
    match fs::read_to_string(file_path) {
        Ok(content) => {
            for line in content.lines() {
                match convert_units(line) {
                    Ok(result) => println!("{}", result),
                    Err(e) => eprintln!("Error converting '{}': {}", line, e),
                }
            }
        }
        Err(e) => eprintln!("Error reading file '{}': {}", file_path, e),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: unit-converter <command> [options]");
        process::exit(1);
    }

    let command = &args[1];

    match command.as_str() {
        "parse" => {
            if args.len() < 3 {
                eprintln!("Error: Missing file path.");
                process::exit(1);
            }
            let file_path = &args[2];
            parse_file(file_path);
        }
        "help" => {
            show_help();
        }
        _ => {
            eprintln!("Error: Unknown command '{}'.", command);
            show_help();
            process::exit(1);
        }
    }
}
