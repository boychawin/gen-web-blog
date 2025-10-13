use crate::error::{GenWebBlogError, Result};
use log::{info, error};
use std::fs;
use std::path::{Path, PathBuf};

/// Read a file to string with centralized error mapping
pub fn read_file_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    let path_ref = path.as_ref();
    fs::read_to_string(path_ref).map_err(|e| {
        GenWebBlogError::file_system(path_ref, format!("Failed to read file: {e}"))
    })
}

/// Write bytes to a file, creating parent directories as needed, with centralized error mapping
pub fn write_file<P: AsRef<Path>, D: AsRef<[u8]>>(path: P, data: D) -> Result<()> {
    let path_ref = path.as_ref();

    if let Some(parent) = path_ref.parent() {
        fs::create_dir_all(parent).map_err(|e| {
            GenWebBlogError::file_system(parent, format!("Failed to create parent directory: {e}"))
        })?;
    }

    fs::write(path_ref, data).map_err(|e| {
        GenWebBlogError::file_system(path_ref, format!("Failed to write file: {e}"))
    })
}

/// Copy static files from public directory to build output
pub fn copy_static_files(out_directory: &Path) -> Result<()> {
    let source_dir = Path::new("public");
    let dest_dir = out_directory;

    if !source_dir.exists() {
        return Err(GenWebBlogError::file_system(
            source_dir,
            format!("Public directory not found: {:?}", source_dir),
        ));
    }

    fs::create_dir_all(dest_dir).map_err(|e| {
        GenWebBlogError::file_system(dest_dir, format!("Failed to create destination directory: {e}"))
    })?;

    for entry in fs::read_dir(source_dir).map_err(|e| {
        GenWebBlogError::file_system(source_dir, format!("Failed to read public directory: {e}"))
    })? {
        let entry = entry.map_err(|e| {
            GenWebBlogError::file_system(source_dir, format!("Failed to read directory entry: {e}"))
        })?;
        let entry_path = entry.path();
        let dest_path = dest_dir.join(entry.file_name());

        let is_dir = entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

        if is_dir {
            if dest_path.exists() && dest_path.is_file() {
                let _ = fs::remove_file(&dest_path);
            }
            copy_dir_contents(&entry_path, &dest_path)?;
        } else {
            if let Some(parent) = dest_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if dest_path.exists() {
                let _ = fs::remove_file(&dest_path);
            }
            fs::copy(&entry_path, &dest_path).map_err(|e| {
                GenWebBlogError::file_system(&entry_path, format!("Failed to copy file: {e}"))
            })?;
        }
    }

    Ok(())
}

pub fn copy_dir_contents(src: &Path, dest: &Path) -> Result<()> {
    if dest.exists() && dest.is_file() {
        fs::remove_file(dest).map_err(|e| {
            GenWebBlogError::file_system(dest, format!("Failed to remove file to create directory: {e}"))
        })?;
    }

    if !dest.exists() {
        fs::create_dir_all(dest).map_err(|e| {
            GenWebBlogError::file_system(dest, format!("Failed to create directory: {e}"))
        })?;
    }

    for sub_entry in fs::read_dir(src).map_err(|e| {
        GenWebBlogError::file_system(src, format!("Failed to read directory: {e}"))
    })? {
        let sub_entry = sub_entry.map_err(|e| {
            GenWebBlogError::file_system(src, format!("Failed to read directory entry: {e}"))
        })?;
        let sub_entry_path = sub_entry.path();
        let sub_dest_path = dest.join(sub_entry.file_name());

        let is_dir = sub_entry.file_type().map(|ft| ft.is_dir()).unwrap_or(false);

        if is_dir {
            if sub_dest_path.exists() && sub_dest_path.is_file() {
                let _ = fs::remove_file(&sub_dest_path);
            }
            copy_dir_contents(&sub_entry_path, &sub_dest_path)?;
        } else {
            if let Some(parent) = sub_dest_path.parent() {
                let _ = fs::create_dir_all(parent);
            }
            if sub_dest_path.exists() {
                let _ = fs::remove_file(&sub_dest_path);
            }
            fs::copy(&sub_entry_path, &sub_dest_path).map_err(|e| {
                GenWebBlogError::file_system(&sub_entry_path, format!("Failed to copy file: {e}"))
            })?;
        }
    }

    Ok(())
}

pub fn delete_file_if_exists(file: &PathBuf) {
    if file.exists() {
        match fs::remove_file(file) {
            Ok(_) => info!("│  ✅ File deleted: {:?}", file),
            Err(err) => error!("│  ❌ Failed to delete file {:?}: {}", file, err),
        }
    }
}


 
