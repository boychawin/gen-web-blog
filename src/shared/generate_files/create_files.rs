use eyre::{Result, WrapErr};
use std::fs;
use std::path::Path;

use super::app_toml_content::APP_TOML_CONTENT;
use super::head::HEAD;
use super::tailwind_config::TAILWIND_CONFIG;
use crate::app::{self, AppInfo};
use crate::shared::generate_files::common_pdpa_modal::COMMON_PDPA;
use crate::shared::generate_files::common_scroll_top::COMMON_SCROLL_TOP;
use crate::shared::generate_files::common_search_modal::COMMON_SEARCH_MODAL;
use crate::shared::generate_files::footer::FOOTER;
use crate::shared::generate_files::header::HEADER;
use crate::shared::generate_files::layouts_blog_index::LAYOUT_BLOG_INDEX;
use crate::shared::generate_files::layouts_landing_index::LAYOUT_LANDING_INDEX;
use crate::shared::generate_files::layouts_profile_index::LAYOUT_PROFILE_INDEX;
use crate::shared::generate_files::styles_app::STYLE_APP;
use crate::shared::generate_files::styles_fonts::STYLE_FONTS;
use crate::shared::generate_files::styles_noscript::STYLE_NOSCRIPT;
use crate::shared::generate_files::system_back_to_top::SYSTEM_BACK_TO_TOP;
use crate::shared::generate_files::system_highlight::SYSTEM_HIGHLIGHT;
use crate::shared::generate_files::system_script_article::SYSTEM_ARTICLE;
use crate::shared::generate_files::system_seach::SYSTEM_SEARCH;
use crate::shared::generate_files::system_theme::SYSTEM_THEME;
use crate::shared::generate_files::system_theme_switch::SYSTEM_THEME_SWITCH;
use crate::shared::generate_files::templates_about::TEMPLATE_ABOUT;
use crate::shared::generate_files::templates_articles::TEMPLATE_ARTICLES;
use crate::shared::generate_files::templates_contact::TEMPLATE_CONTACT;
use crate::shared::generate_files::templates_index::TEMPLATE_INDEX;
use crate::shared::generate_files::templates_layout::TEMPLATE_LAYOUT;
use crate::shared::generate_files::templates_policy::TEMPLATE_POLICY;
use crate::shared::generate_files::templates_post::TEMPLATE_POST;
use crate::shared::generate_files::templates_sitemap::TEMPLATE_SITEMAP;

fn generate_yaml_content(
    layout: &str,
    title: &str,
    description: &str,
    image: &str,
    keywords: &[&str],
) -> String {
    let mut result = String::new();

    if !layout.trim().is_empty() {
        result.push_str(&format!("layout: \"{layout}\"\n"));
    }

    result.push_str(&format!("title: \"{title}\"\n"));
    result.push_str(&format!("description: \"{description}\"\n"));
    result.push_str(&format!("image: \"{image}\"\n"));

    if !keywords.is_empty() {
        result.push_str("keywords:\n");
        for kw in keywords {
            result.push_str(&format!("  - \"{kw}\"\n"));
        }
    }

    result
}

pub fn create_files(is_full: bool) -> Result<()> {
    const REQUIRED_PATHS_BASE: &[&str] = &[
        "contents/",
        "contents/index.yml",
        "contents/about.yml",
        "contents/contact.yml",
        "contents/policy.yml",
        "public/",
        "public/favicon/",
        "public/_system_/fonts/",
        "public/_system_/scripts/",
        "public/_system_/scripts/back_to_top.js",
        "public/_system_/scripts/theme_switch.js",
        "public/_system_/scripts/theme.js",
        "public/_system_/scripts/seach.js",
        "public/_system_/scripts/highlight.js",
        "public/_system_/scripts/articles.js",
        "public/_system_/styles/",
        "public/images/",
        "public/javascripts/",
        "public/robots.txt",
        "public/site.webmanifest",
        "source/",
        "source/",
        "source/components/",
        "source/components/common/",
        "source/components/common/pdpa_modal.html",
        "source/components/common/scroll_top.html",
        "source/components/common/search_modal.html",
        "source/components/layouts/",
        "source/components/layouts/landing/index.html",
        "source/components/header.html",
        "source/components/footer.html",
        "source/layouts/",
        "source/layouts/head.html",
        "source/layouts/layout.html",
        "source/pages/about.html",
        "source/pages/articles.html",
        "source/pages/contact.html",
        "source/pages/index.html",
        "source/pages/policy.html",
        "source/pages/post.html",
        "source/pages/sitemap.html",
        "source/styles/",
        "source/styles/app.scss",
        "source/styles/fonts.scss",
        "source/styles/noscript.scss",
        "source/tailwind.config.toml",
        ".gitignore",
        "app.toml",
        "README.md",
    ];

    const REQUIRED_PATHS_FULL_EXT: &[&str] = &[
        "source/components/layouts/blog/index.html",
        "source/components/layouts/profile/index.html",
    ];

    let mut required_paths: Vec<&str> = Vec::with_capacity(
        REQUIRED_PATHS_BASE.len() + if is_full { REQUIRED_PATHS_FULL_EXT.len() } else { 0 },
    );
    required_paths.extend_from_slice(REQUIRED_PATHS_BASE);
    if is_full {
        required_paths.extend_from_slice(REQUIRED_PATHS_FULL_EXT);
    }

    let config = app::read_config();
    let app_info = config.app_info;

    for path_str in &required_paths {
        let path = Path::new(path_str);

        if !path.exists() {
            if path_str.ends_with('/') {
                fs::create_dir_all(path)
                    .wrap_err_with(|| format!("Failed to create directory: {path_str}"))?;
                println!("│  📁 Created directory: {path_str}");
            } else {
                create_file_with_default_content(path, path_str, &app_info)?;
            }
        }
    }

    Ok(())
}

fn create_file_with_default_content(path: &Path, path_str: &str, app_info: &AppInfo) -> Result<()> {
    let app_name = &app_info.app_name;
    let app_description = &app_info.app_description;

    if let Some(parent_dir) = path.parent() {
        fs::create_dir_all(parent_dir)
            .wrap_err_with(|| format!("Failed to create parent directory for file: {path_str}"))?;
    }

    let gitignore = "genwebblog";

    let robots_txt_content = "User-agent: *\nAllow: /";

    let site_manifest_content = format!(
        r#"{{
            "name": "{}",
            "short_name": "GenWeb",
            "description": "{}",
            "icons": [
                {{
                    "src": "/favicon/android-chrome-192x192.png",
                    "sizes": "192x192",
                    "type": "image/png"
                }},
                {{
                    "src": "/favicon/android-chrome-512x512.png",
                    "sizes": "512x512",
                    "type": "image/png"
                }}
            ],
            "start_url": "/",
            "display": "standalone",
            "orientation": "portrait"
        }}"#,
        &app_name,
        app_description
            .as_deref()
            .unwrap_or("No description available")
    );

    let default_content: String = match path_str {
        "public/_system_/scripts/back_to_top.js" => SYSTEM_BACK_TO_TOP.to_string(),
        "public/_system_/scripts/theme_switch.js" => SYSTEM_THEME_SWITCH.to_string(),
        "public/_system_/scripts/theme.js" => SYSTEM_THEME.to_string(),
        "public/_system_/scripts/seach.js" => SYSTEM_SEARCH.to_string(),
        "public/_system_/scripts/highlight.js" => SYSTEM_HIGHLIGHT.to_string(),
        "public/_system_/scripts/articles.js" => SYSTEM_ARTICLE.to_string(),
        "source/components/layouts/blog/index.html" => LAYOUT_BLOG_INDEX.to_string(),
        "source/components/layouts/profile/index.html" => LAYOUT_PROFILE_INDEX.to_string(),
        "source/components/layouts/landing/index.html" => LAYOUT_LANDING_INDEX.to_string(),
        "source/components/common/search_modal.html" => COMMON_SEARCH_MODAL.to_string(),
        "source/components/common/pdpa_modal.html" => COMMON_PDPA.to_string(),
        "source/components/common/scroll_top.html" => COMMON_SCROLL_TOP.to_string(),
        "source/pages/index.html" => TEMPLATE_INDEX.to_string(),
        "source/layouts/head.html" => HEAD.to_string(),
        "source/layouts/layout.html" => TEMPLATE_LAYOUT.to_string(),
        "source/pages/post.html" => TEMPLATE_POST.to_string(),
        "source/pages/sitemap.html" => TEMPLATE_SITEMAP.to_string(),
        "source/pages/policy.html" => TEMPLATE_POLICY.to_string(),
        "source/pages/about.html" => TEMPLATE_ABOUT.to_string(),
        "source/pages/articles.html" => TEMPLATE_ARTICLES.to_string(),
        "source/pages/contact.html" => TEMPLATE_CONTACT.to_string(),
        "source/components/header.html" => HEADER.to_string(),
        "source/components/footer.html" => FOOTER.to_string(),
        "source/styles/app.scss" => STYLE_APP.to_string(),
        "source/styles/fonts.scss" => STYLE_FONTS.to_string(),
        "source/styles/noscript.scss" => STYLE_NOSCRIPT.to_string(),
        "source/tailwind.config.toml" => TAILWIND_CONFIG.to_string(),
        ".gitignore" => gitignore.to_string(),
        "app.toml" => APP_TOML_CONTENT.to_string(),
        "public/robots.txt" => robots_txt_content.to_string(),
        "public/site.webmanifest" => site_manifest_content.to_string(),
        "contents/about.yml" => generate_yaml_content(
            "",
            "About Us",
            "Information about our company.",
            "about-image.jpg",
            &["about", "company"],
        ),
        "contents/contact.yml" => generate_yaml_content(
            "",
            "Contact Us",
            "Get in touch with us.",
            "contact-image.jpg",
            &["contact", "support"],
        ),
        "contents/policy.yml" => generate_yaml_content(
            "",
            "Privacy Policy",
            "Your privacy is important to us.",
            "policy-image.jpg",
            &["privacy", "policy"],
        ),
        "contents/index.yml" => generate_yaml_content(
            "index",
            "Home",
            "Welcome to our website!",
            "home-image.jpg",
            &["home", "landing"],
        ),
        _ => "Content not found for the specified path.".to_string(),
    };

    fs::write(path, default_content)
        .wrap_err_with(|| format!("Failed to create file: {path_str}"))?;
    println!("│  📄 Created file: {path_str}");
    Ok(())
}
