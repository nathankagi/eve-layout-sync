use std::fs;
use std::io;
use std::path::Path;

pub fn copy_directory(src: &Path, dest: &Path, directories: bool) -> io::Result<()> {
    if !src.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "source is not a directory",
        ));
    }

    println!("{}", dest.to_string_lossy());

    if !dest.exists() {
        fs::create_dir_all(dest)?;
    }

    for item in fs::read_dir(src)? {
        let item = item?;
        let item_path = item.path();
        let file_name = item_path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .into_owned();
        let dest_path = dest.join(&file_name);

        if item_path.is_dir() {
            if !item_path.ends_with("archive") && directories {
                copy_directory(&item_path, &dest_path, directories)?;
            }
        } else {
            fs::copy(&item_path, &dest_path)?;
        }
    }

    Ok(())
}
