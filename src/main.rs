use clap::Parser;
use eve_layout_sync::{copy_directory, sync_settings};
use std::io;
use std::path::PathBuf;

const USER_FILE_PREFIX: &str = "core_user_";
const CHAR_FILE_PREFIX: &str = "core_char_";
const EXEMPT_FILES: [&str; 3] = [
    "core_char__.dat",
    "core_char_('char', None, 'dat').dat",
    "core_user__.dat",
];

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

    let mut user_path = PathBuf::from(&args.settings_path);
    user_path.push(&args.user);
    if !user_path.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "User file does not exist.",
        ));
    }

    let mut char_path = PathBuf::from(&args.settings_path);
    char_path.push(&args.character);
    if !char_path.is_file() {
        return Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Character file does not exist.",
        ));
    }

    sync_settings(
        CHAR_FILE_PREFIX,
        &args.settings_path,
        &char_path,
        EXEMPT_FILES.to_vec(),
    )?;

    sync_settings(
        USER_FILE_PREFIX,
        &args.settings_path,
        &user_path,
        EXEMPT_FILES.to_vec(),
    )?;

    Ok(())
}
