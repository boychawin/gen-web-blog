use crate::app::AppConfig;
use eyre::Result;
use std::fs;
use std::path::Path;

pub fn generate_robots_txt(app: &AppConfig) -> Result<()> {
    let mut robots_txt_content = r"
User-agent: *
Allow: /
"
    .to_string();

    let contents_dir = Path::new("contents");
    let app_domain = &app.app_info.app_domain;
    // Ensure no trailing slash on domain
    let app_domain = app_domain.trim_end_matches('/');

    if contents_dir.exists() && contents_dir.is_dir() {
        for entry in fs::read_dir(contents_dir)? {
            let entry = entry?;
            if entry.path().is_dir() {
                let folder_name = entry.file_name().to_string_lossy().to_string();
                let sitemap_path = format!("{app_domain}/{folder_name}/sitemap.xml");
                robots_txt_content.push_str(&format!("Sitemap: {sitemap_path}\n"));
            }
        }
    }

    robots_txt_content.push_str(&format!("Sitemap: {app_domain}/sitemap.xml\n"));

    let robots_txt_path = Path::new("public/robots.txt");

    if !robots_txt_path.exists() {
        if let Some(parent) = robots_txt_path.parent() {
            fs::create_dir_all(parent)?;
        }
    }

    crate::shared::fs::write_file(robots_txt_path, robots_txt_content.as_bytes())?;

    Ok(())
}
