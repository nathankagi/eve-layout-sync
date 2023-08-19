use std::fs;
use std::io;
use std::path::{Path, PathBuf};

pub fn sync_settings(
    file_prefix: &str,
    path: &PathBuf,
    reference_path: &Path,
    exepmt_files: Vec<&str>,
) -> io::Result<()> {
    let matching_paths = find_file_paths(file_prefix, path, &exepmt_files)?;
    if let Some(file_name) = reference_path.file_name() {
        println!(
            "Copying data from {} to the following files:",
            file_name.to_string_lossy()
        )
    }

    for path in &matching_paths {
        if let Some(file_name) = path.file_name() {
            println!("{}", file_name.to_string_lossy())
        }
    }
    println!();

    let reference_contents = fs::read(reference_path)?;
    for path in &matching_paths {
        fs::write(path, &reference_contents)?;
    }

    Ok(())
}

pub fn find_file_paths(
    search_string: &str,
    path: &std::path::PathBuf,
    file_exceptions: &[&str],
) -> Result<Vec<PathBuf>, io::Error> {
    let mut matching_paths = Vec::new();

    if path.is_dir() {
        for item in fs::read_dir(path).unwrap() {
            let item = item?;
            let file_name = item.file_name().to_string_lossy().to_string();
            let file_path = item.path();

            if !file_exceptions.contains(&file_name.as_str()) && file_name.contains(search_string) {
                matching_paths.push(file_path);
            }
        }
    }

    Ok(matching_paths)
}

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
