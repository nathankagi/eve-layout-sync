use clap::Parser;
use eve_layout_sync::copy_directory;
use std::io;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    #[arg(short, long)]
    pub user: std::path::PathBuf,

    #[arg(short, long)]
    pub character: std::path::PathBuf,

    #[arg(short, long, default_value = PathBuf::from(".").into_os_string())]
    pub settings_path: std::path::PathBuf,

    #[arg(short, long, default_value = PathBuf::from("./archive/").into_os_string())]
    pub archive_path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Args::parse();

    copy_directory(&args.settings_path, &args.archive_path, false)?;

    Ok(())
}
