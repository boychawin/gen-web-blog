use minify_html::{minify, Cfg};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub struct OutputWriter {
    out_directory: PathBuf,
    minify_config: Cfg,
}

impl OutputWriter {
    pub fn new(out_directory: impl AsRef<Path>) -> Self {
        let minify_config = Cfg {
            minify_css: true,
            minify_js: false,
            ..Default::default()
        };

        Self {
            out_directory: out_directory.as_ref().to_path_buf(),
            minify_config,
        }
    }

    pub fn write_html_file(&self, relative_path: &str, content: &str) -> eyre::Result<()> {
        let file_path = self.out_directory.join(relative_path);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let minified_content = minify(content.as_bytes(), &self.minify_config);

        let mut file = File::create(&file_path)?;
        file.write_all(&minified_content)?;

        println!("│    ✅ Generated: {relative_path}");
        Ok(())
    }

    #[allow(dead_code)]
    pub fn write_text_file(&self, relative_path: &str, content: &str) -> eyre::Result<()> {
        let file_path = self.out_directory.join(relative_path);

        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::write(&file_path, content)?;

        println!("│    ✅ Generated: {relative_path}");
        Ok(())
    }

    pub fn write_json_file(
        &self,
        relative_path: &str,
        data: &serde_json::Value,
    ) -> eyre::Result<()> {
        let content = serde_json::to_string_pretty(data)?;
        self.write_text_file(relative_path, &content)
    }

    #[allow(dead_code)]
    pub fn create_directory(&self, relative_path: &str) -> eyre::Result<()> {
        let dir_path = self.out_directory.join(relative_path);
        fs::create_dir_all(&dir_path)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn copy_file(&self, src: &Path, relative_dest: &str) -> eyre::Result<()> {
        let dest_path = self.out_directory.join(relative_dest);

        if let Some(parent) = dest_path.parent() {
            fs::create_dir_all(parent)?;
        }

        fs::copy(src, &dest_path)?;
        Ok(())
    }

    #[must_use]
    #[allow(dead_code)]
    pub fn get_output_path(&self, relative_path: &str) -> PathBuf {
        self.out_directory.join(relative_path)
    }
}