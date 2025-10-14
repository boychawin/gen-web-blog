use serde::{Deserialize, Serialize};
use std::path::Path;
use toml;

fn default_true() -> bool {
    true
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct AppInfo {
    pub app_name: String,
    pub app_token: String,
    pub app_version: String,
    pub app_author: Option<String>,
    pub app_description: Option<String>,
    pub app_phone: Option<String>,
    pub app_email: Option<String>,
    pub app_domain: String,
    pub app_port: u16,
    pub app_facebook_link: Option<String>,
    pub app_instagram_link: Option<String>,
    pub app_x_link: Option<String>,
    pub app_line_link: Option<String>,
    pub app_youtube_link: Option<String>,
    pub app_github_link: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct GitHubConfig {
    pub token: String,
    pub user: String,
    pub repo_name: String,
    #[serde(default = "default_true")]
    pub private: bool,
    pub branch: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct CloudflareConfig {
    pub account_id: String,
    pub api_token: String,
    pub project_name: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct Domains {
    pub list: String,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct Facebook {
    pub facebook_id: Option<String>,
    pub facebook_app_id: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct Twitter {
    pub twitter_site: Option<String>,
    pub twitter_creator: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Languages {
    pub installed_languages: Vec<String>,
    pub default_language: String,
}

impl Default for Languages {
    fn default() -> Self {
        Self {
            installed_languages: vec!["en".to_string()],
            default_language: "en".to_string(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct Paths {
    pub source_layouts: Option<String>,
    pub source_pages: Option<String>,
    pub source_templates: Option<String>,
    pub contents_dir: Option<String>,
    pub public_dir: Option<String>,
    pub favicon_dir: Option<String>,
    pub system_dir: Option<String>,
    pub translations_dir: Option<String>,
    pub build_dir: Option<String>,
    pub use_directory_structure: Option<bool>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct Css {
    pub files: Option<Vec<String>>,
    pub tailwind_path: Option<String>,
    pub output_path: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct Favicon {
    pub ico: Option<String>,
    pub apple_touch_icon: Option<String>,
    pub ico_16: Option<String>,
    pub ico_32: Option<String>,
    pub svg: Option<String>,
    pub mask_icon: Option<String>,
    pub web_manifest: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct Directories {
    pub system: Option<String>,
    pub images: Option<String>,
    pub favicon: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct Seo {
    pub type_page_default: Option<String>,
    pub type_page_article: Option<String>,
    pub type_page_blog: Option<String>,
    pub type_page_home: Option<String>,
    pub type_page_about: Option<String>,
    pub type_page_contact: Option<String>,
    pub robots_content: Option<String>,
    pub sitemap_changefreq: Option<String>,
    pub sitemap_priority: Option<String>,
    pub google_adsense_client: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct SocialMeta {
    pub og_type: Option<String>,
    pub twitter_card: Option<String>,
    pub twitter_label1: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct Locales {
    pub th: Option<String>,
    pub en: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
#[allow(dead_code)]
pub struct AppConfig {
    pub app_info: AppInfo,
    pub deploy_github: GitHubConfig,
    pub deploy_cloudflare: CloudflareConfig,
    pub deploy_domains: Domains,
    pub facebook: Facebook,
    pub twitter: Twitter,
    #[serde(default)]
    pub languages: Languages,
    pub paths: Option<Paths>,
    pub css: Option<Css>,
    pub favicon: Option<Favicon>,
    pub directories: Option<Directories>,
    pub seo: Option<Seo>,
    pub locales: Option<Locales>,
    pub social_meta: Option<SocialMeta>,
}

#[must_use]
pub fn read_config() -> AppConfig {
    let path = "app.toml";

    if !Path::new(path).exists() {
        println!("│  ⚠️ Warning: No 'app.toml' found. use command ./genwebblog init");
        return AppConfig::default();
    }

    let Ok(config_str) = crate::shared::fs::read_file_to_string(path) else {
        println!("│  ⚠️  Warning: Unable to read '{path}'. Using default configuration.");
        return AppConfig::default();
    };

    match toml::from_str::<AppConfig>(&config_str) {
        Ok(config) => config,
        Err(e) => {
            eprintln!("│  ⚠️  Warning: Failed to parse '{path}': {e}. Using default configuration.");
            AppConfig::default()
        }
    }
}