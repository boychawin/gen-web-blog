use std::fs;
use std::path::{Path, PathBuf};

pub struct AssetManager {
    out_directory: PathBuf,
}

impl AssetManager {
    pub fn new(out_directory: impl AsRef<Path>) -> Self {
        Self {
            out_directory: out_directory.as_ref().to_path_buf(),
        }
    }

    pub fn copy_static_assets(&self) -> eyre::Result<()> {
        println!("│    📦 Copying static assets...");

        self.copy_public_directory()?;
        self.copy_favicon_files()?;
        self.copy_system_files()?;

        Ok(())
    }

    fn copy_public_directory(&self) -> eyre::Result<()> {
        let public_dir = Path::new("public");
        if public_dir.exists() {
            Self::copy_directory_recursive(public_dir, &self.out_directory)?;
        }
        Ok(())
    }

    fn copy_favicon_files(&self) -> eyre::Result<()> {
        let favicon_src = Path::new("public/favicon");
        let favicon_dest = self.out_directory.join("favicon");

        if favicon_src.exists() {
            Self::copy_directory_recursive(favicon_src, &favicon_dest)?;
        }
        Ok(())
    }

    fn copy_system_files(&self) -> eyre::Result<()> {
        let system_src = Path::new("public/_system_");
        let system_dest = self.out_directory.join("_system_");

        if system_src.exists() {
            Self::copy_directory_recursive(system_src, &system_dest)?;
        }
        Ok(())
    }

    fn copy_directory_recursive(src: &Path, dest: &Path) -> eyre::Result<()> {
        if !dest.exists() {
            fs::create_dir_all(dest)?;
        }

        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dest_path = dest.join(entry.file_name());

            if src_path.is_dir() {
                Self::copy_directory_recursive(&src_path, &dest_path)?;
            } else {
                fs::copy(&src_path, &dest_path)?;
            }
        }

        Ok(())
    }

    pub fn create_build_directories(&self) -> eyre::Result<()> {
        if self.out_directory.exists() {
            println!("│    🗑️ Cleaning previous build...");
            fs::remove_dir_all(&self.out_directory)?;
        }

        println!("│    📁 Creating build directories...");
        fs::create_dir_all(&self.out_directory)?;
        fs::create_dir_all(self.out_directory.join("_system_"))?;
        fs::create_dir_all(self.out_directory.join("images"))?;
        fs::create_dir_all(self.out_directory.join("favicon"))?;

        Ok(())
    }
}