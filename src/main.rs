use clap::Parser;
use std::env;
use std::error::Error;
use std::fs;

#[derive(Parser)]
pub struct Config {
    pub user: std::path::PathBuf,
    pub character: std::path::PathBuf,
    pub settings_path: std::path::PathBuf,
    pub archive_path: std::path::PathBuf,
}

fn main() {
    let args = Config::parse();

    let user = std::fs::read(&args.user).expect("could not read user file");
    let character = std::fs::read(&args.character).expect("could not read character file");
}
