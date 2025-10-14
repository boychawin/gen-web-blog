use crate::constants::images;
use std::path::{Path, PathBuf};

pub fn get_image(image_url: Option<&str>, default_image: &str) -> String {
    fn strip_query_and_fragment(s: &str) -> &str {
        s.split(|c| ['?', '#'].contains(&c)).next().unwrap_or(s)
    }

    fn is_image_file(url: &str) -> bool {
        let cleaned = strip_query_and_fragment(url);
        if let Some(ext) = Path::new(cleaned).extension().and_then(|e| e.to_str()) {
            images::ALL_FORMATS.contains(&ext.to_lowercase().as_str())
        } else {
            false
        }
    }

    match image_url {
        Some(url) if !url.is_empty() && is_image_file(url) => url.to_string(),
        _ => default_image.to_string(),
    }
}

pub fn get_default_post_image() -> String {
    "/favicon/favicon.svg".to_string()
}

pub fn file_url(out_directory: &PathBuf, path: &Path) -> String {
    let out_pathbuf = match out_directory.canonicalize() {
        Ok(p) => p,
        Err(_) => out_directory.to_owned(),
    };

    let mut out = out_pathbuf.display().to_string();
    if out.starts_with('/') {
        out = out.trim_start_matches('/').to_string();
    }
    out = out.replace(' ', "%20");
    out = out.replace("\\\\?\\", "");
    out = out.replace(std::path::MAIN_SEPARATOR, "/");

    let mut p = path.to_string_lossy().to_string();
    p = p.replace(' ', "%20");
    p = p.replace("\\\\?\\", "");

    format!("file:///{out}/{p}")
}
