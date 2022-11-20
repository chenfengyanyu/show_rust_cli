#![allow(unused)]
use anyhow::{Context, Ok, Result};
use clap::Parser;
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct CLI {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf, // PathBuf is like a String but for file system paths that work cross-platform.
}
fn main() -> Result<()> {
    let path = "test.txt";
    let args = CLI::parse();
    println!("{:?} ", args);

    let contents =
        std::fs::read_to_string(path).with_context(|| format!("could not read file `{}`", path))?;

    // for line in contents.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }
    Ok(())
}
