#![allow(unused)]
use anyhow::{Context, Ok, Result};
use clap::Parser;
use std::thread;
use std::time::Duration;
use log::{info, trace, warn};
/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser, Debug)]
struct CLI {
    /// The pattern to look for
    pattern: String,
    /// The path to the file to read
    path: std::path::PathBuf, // PathBuf is like a String but for file system paths that work cross-platform.
}
fn main() -> Result<()> {
    let args = CLI::parse();
    println!("{:?} ", args);
    let path = args.path;

    let contents = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{:?}`", path))?;

    // for line in contents.lines() {
    //     if line.contains(&args.pattern) {
    //         println!("{}", line);
    //     }
    // }

    // progress_func();
    user_log();

    Ok(())
}
// 进度条 indicatif 文档地址：https://crates.io/crates/indicatif
fn progress_func() {
    let pb = indicatif::ProgressBar::new(100);
    for i in 0..100 {
        thread::sleep(Duration::from_millis(5));
        // pb.println(format!("[+] finished #{}", i));
        pb.inc(1);
    }
    pb.finish_with_message("done");
}

// 日志 log 文档地址：https://crates.io/crates/log
// https://docs.rs/env_logger/0.9.0/env_logger/index.html
fn user_log() {
    env_logger::init();
    trace!("Progress Bar is start!");
    thread::sleep(Duration::from_secs(1));
    info!("Time is out!");
    thread::sleep(Duration::from_secs(1));
    warn!("oops, nothing implemented!");
}

