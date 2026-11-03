//!Takes a .lol file and compiles it into an HTML file, then opens the HTML file in the default browser

mod lexer;
mod parser;
mod semantic;
mod traits;

#[cfg(test)]
mod tests;

use parser::LolCompiler;
use std::env;
use std::fs;
use std::process;
use std::process::Command;
use traits::Compiler;

///Checks if the given file name has a .lol extension
fn has_lol_extension(file_name: &str) -> bool {
    file_name.len() > 4 && file_name.to_lowercase().ends_with(".lol")
}

///Takes a .lol file, compiles it into an HTML file, and opens the HTML file in the default browser
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("Usage: lolcompiler <file>.lol");
        process::exit(1);
    }

    let file_name = &args[1];
    if !has_lol_extension(file_name) {
        eprintln!("Usage error: {} is not a .lol file", file_name);
        eprintln!("Usage: lolcompiler <file>.lol");
        process::exit(1);
    }

    let source = match fs::read_to_string(file_name) {
        Ok(text) => text,
        Err(_) => {
            eprintln!("Error: could not read {}", file_name);
            process::exit(1);
        }
    }; 
    ///compile exits on any error, so no HTML file gets made

    let mut compiler = LolCompiler::new(&source);
    compiler.compile(&source);

    let html_file = format!("{}.html", &file_name[..file_name.len() - 4]);
    if fs::write(&html_file, &compiler.html).is_err() {
        eprintln!("Error: could not write {}", html_file);
        process::exit(1);
    }
    println!("Compiled {} into {}", file_name, html_file);

    if Command::new("open").arg(&html_file).spawn().is_err() {
        println!("Could not open the browser automatically");
    }
}
