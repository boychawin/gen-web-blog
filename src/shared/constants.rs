
use std::collections::HashMap;

/// Application metadata constants
pub mod app {
    /// Current application version
    pub const VERSION: &str = "1.5.0";

    /// Application name
    pub const NAME: &str = "GenWebBlog";

    /// Default configuration file name
    pub const CONFIG_FILE: &str = "app.toml";

    /// Default port for development server
    pub const DEFAULT_PORT: u16 = 3000;

    /// User agent string for HTTP requests
    pub const USER_AGENT: &str = "GenWebBlog/1.5.0";
}

/// File and directory path constants
pub mod paths {
    /// Default source directory for content
    pub const CONTENTS_DIR: &str = "contents";

    /// Default build output directory
    pub const BUILD_DIR: &str = "build";

    /// Public assets directory
    pub const PUBLIC_DIR: &str = "public";

    /// Source templates and layouts directory
    pub const SOURCE_DIR: &str = "source";

    /// Translations directory
    pub const TRANSLATIONS_DIR: &str = "source/translations";

    /// Layouts directory
    pub const LAYOUTS_DIR: &str = "source/layouts";

    /// Templates directory
    pub const TEMPLATES_DIR: &str = "source/templates";

    /// Pages directory
    pub const PAGES_DIR: &str = "source/pages";

    /// Styles directory
    pub const STYLES_DIR: &str = "source/styles";

    /// System assets directory
    pub const SYSTEM_DIR: &str = "public/_system_";

    /// Favicon directory
    pub const FAVICON_DIR: &str = "public/favicon";

    /// Images directory
    pub const IMAGES_DIR: &str = "public/images";
}

/// File extension constants
pub mod extensions {
    /// Markdown file extension
    pub const MARKDOWN: &str = "md";

    /// YAML file extension
    pub const YAML: &str = "yml";

    /// TOML file extension
    pub const TOML: &str = "toml";

    /// HTML file extension
    pub const HTML: &str = "html";

    /// CSS file extension
    pub const CSS: &str = "css";

    /// SCSS file extension
    pub const SCSS: &str = "scss";

    /// JavaScript file extension
    pub const JS: &str = "js";

    /// JSON file extension
    pub const JSON: &str = "json";
}

/// File name constants
pub mod files {
    /// Article manifest file name
    pub const MANIFEST: &str = "index.yml";

    /// Site configuration file
    pub const SITE_CONFIG: &str = "site.yml";

    /// Sitemap file name
    pub const SITEMAP: &str = "sitemap.xml";

    /// Robots.txt file name
    pub const ROBOTS: &str = "robots.txt";

    /// RSS feed file name
    pub const RSS_FEED: &str = "feed.xml";

    /// Web manifest file name
    pub const WEB_MANIFEST: &str = "site.webmanifest";

    /// Releases JSON file name
    pub const RELEASES_JSON: &str = "releases.json";
}

/// Default favicon paths
pub mod favicons {
    /// Default favicon.ico path
    pub const ICO: &str = "/favicon/favicon.ico";

    /// Default Apple touch icon path
    pub const APPLE_TOUCH_ICON: &str = "/favicon/apple-touch-icon.png";

    /// Default 16x16 favicon path
    pub const ICO_16: &str = "/favicon/favicon-16x16.png";

    /// Default 32x32 favicon path
    pub const ICO_32: &str = "/favicon/favicon-32x32.png";

    /// Default SVG favicon path
    pub const SVG: &str = "/favicon/favicon.svg";

    /// Default mask icon path
    pub const MASK_ICON: &str = "/favicon/mask-icon.svg";

    /// Default web manifest path
    pub const WEB_MANIFEST: &str = "/site.webmanifest";
}

/// SEO and meta tag constants
pub mod seo {
    /// Default Open Graph type
    pub const DEFAULT_OG_TYPE: &str = "website";

    /// Default Twitter card type
    pub const DEFAULT_TWITTER_CARD: &str = "summary_large_image";

    /// Default robots content
    pub const DEFAULT_ROBOTS_CONTENT: &str = "follow, index";

    /// Default sitemap change frequency
    pub const DEFAULT_SITEMAP_CHANGEFREQ: &str = "weekly";

    /// Default sitemap priority
    pub const DEFAULT_SITEMAP_PRIORITY: &str = "0.8";

    /// Page type for blog posts
    pub const PAGE_TYPE_ARTICLE: &str = "BlogPosting";

    /// Page type for regular pages
    pub const PAGE_TYPE_DEFAULT: &str = "WebPage";

    /// Page type for home page
    pub const PAGE_TYPE_HOME: &str = "WebSite";

    /// Page type for about page
    pub const PAGE_TYPE_ABOUT: &str = "AboutPage";

    /// Page type for contact page
    pub const PAGE_TYPE_CONTACT: &str = "ContactPage";
}

/// Supported image formats
pub mod images {
    /// Modern image formats (preferred)
    pub const MODERN_FORMATS: &[&str] = &["webp", "avif"];

    /// Traditional image formats
    pub const TRADITIONAL_FORMATS: &[&str] = &["jpg", "jpeg", "png", "gif"];

    /// Vector image formats
    pub const VECTOR_FORMATS: &[&str] = &["svg"];

    /// All supported image formats
    pub const ALL_FORMATS: &[&str] = &[
        "webp", "avif", "jpg", "jpeg", "png", "gif", "svg", "bmp",
    ];
}

/// Language and locale constants
pub mod languages {
    /// Default language code
    pub const DEFAULT_LANGUAGE: &str = "th";

    /// Supported language codes
    pub const SUPPORTED_LANGUAGES: &[&str] = &["th", "en", "ja", "zh", "ko"];

    /// Language configuration file pattern
    pub const LANG_CONFIG_PATTERN: &str = "source/translations/{}/main.toml";
}

/// Validation constants
pub mod validation {
    /// Forbidden characters in filenames
    pub const FORBIDDEN_FILENAME_CHARS: &[char] = &['<', '>', ':', '"', '|', '?', '*', ' '];

    /// Maximum filename length (without extension)
    pub const MAX_FILENAME_LENGTH: usize = 80;

    /// Date format regex pattern for filename validation
    pub const DATE_PATTERN: &str = r"^\d{4}-\d{2}-\d{2}$";

    /// Full filename pattern for blog posts
    pub const BLOG_FILENAME_PATTERN: &str = r"^\d{4}-\d{2}-\d{2}-[a-z0-9]+(-[a-z0-9]+)*\.md$";

    /// Valid year range for blog posts
    pub const VALID_YEAR_RANGE: std::ops::RangeInclusive<u32> = 2020..=2030;
}

/// HTTP and network constants
pub mod network {
    /// Request timeout in seconds
    pub const REQUEST_TIMEOUT_SECONDS: u64 = 30;

    /// Maximum retry attempts for network operations
    pub const MAX_RETRY_ATTEMPTS: u32 = 3;

    /// Retry delay in milliseconds
    pub const RETRY_DELAY_MS: u64 = 1000;

    /// GitHub API base URL
    pub const GITHUB_API_BASE: &str = "https://api.github.com";

    /// Cloudflare API base URL
    pub const CLOUDFLARE_API_BASE: &str = "https://api.cloudflare.com/client/v4";

    /// Connectivity check URL
    pub const CONNECTIVITY_CHECK_URL: &str = "https://www.google.com";
}

/// Build and compilation constants
pub mod build {
    /// CSS vendor file name
    pub const VENDOR_CSS: &str = "vendor.css";

    /// Tailwind CSS file name
    pub const TAILWIND_CSS: &str = "tailwind.css";

    /// App CSS file name
    pub const APP_CSS: &str = "app.css";

    /// Fonts CSS file name
    pub const FONTS_CSS: &str = "fonts.css";

    /// Default CSS files to concatenate
    pub const DEFAULT_CSS_FILES: &[&str] = &["tailwind", "fonts", "app"];
}

/// Template and layout constants
pub mod templates {
    /// Default layout for articles
    pub const DEFAULT_ARTICLE_LAYOUT: &str = "articles";

    /// Default layout for pages
    pub const DEFAULT_PAGE_LAYOUT: &str = "page";

    /// Default layout for home page
    pub const DEFAULT_HOME_LAYOUT: &str = "home";

    /// Handlebars file extension
    pub const HANDLEBARS_EXT: &str = "hbs";
}

/// Content processing constants
pub mod content {
    /// Default excerpt length in characters
    pub const DEFAULT_EXCERPT_LENGTH: usize = 160;

    /// Default reading time words per minute
    pub const READING_TIME_WPM: u32 = 200;

    /// Keywords separator
    pub const KEYWORDS_SEPARATOR: &str = ", ";
}

/// Server and development constants
pub mod server {
    /// Default host for development server
    pub const DEFAULT_HOST: &str = "127.0.0.1";

    /// Default port range for development server
    pub const DEFAULT_PORT_RANGE: std::ops::RangeInclusive<u16> = 3000..=3999;

    /// Server startup timeout in seconds
    pub const STARTUP_TIMEOUT_SECONDS: u64 = 10;
}

/// Get default language translations
pub fn get_default_translations() -> HashMap<&'static str, HashMap<&'static str, &'static str>> {
    let mut translations = HashMap::new();

    // Thai translations
    let mut th_translations = HashMap::new();
    th_translations.insert("home", "หน้าหลัก");
    th_translations.insert("about", "เกี่ยวกับ");
    th_translations.insert("contact", "ติดต่อ");
    th_translations.insert("articles", "บทความ");
    th_translations.insert("read_more", "อ่านเพิ่มเติม");
    th_translations.insert("previous", "ก่อนหน้า");
    th_translations.insert("next", "ถัดไป");
    th_translations.insert("search", "ค้นหา");
    th_translations.insert("published_on", "เผยแพร่เมื่อ");
    th_translations.insert("updated_on", "อัปเดตเมื่อ");
    th_translations.insert("category", "หมวดหมู่");
    th_translations.insert("tags", "แท็ก");
    th_translations.insert("author", "ผู้เขียน");
    th_translations.insert("reading_time", "เวลาในการอ่าน");
    th_translations.insert("minutes", "นาที");

    // English translations
    let mut en_translations = HashMap::new();
    en_translations.insert("home", "Home");
    en_translations.insert("about", "About");
    en_translations.insert("contact", "Contact");
    en_translations.insert("articles", "Articles");
    en_translations.insert("read_more", "Read More");
    en_translations.insert("previous", "Previous");
    en_translations.insert("next", "Next");
    en_translations.insert("search", "Search");
    en_translations.insert("published_on", "Published on");
    en_translations.insert("updated_on", "Updated on");
    en_translations.insert("category", "Category");
    en_translations.insert("tags", "Tags");
    en_translations.insert("author", "Author");
    en_translations.insert("reading_time", "Reading time");
    en_translations.insert("minutes", "minutes");

    translations.insert("th", th_translations);
    translations.insert("en", en_translations);

    translations
}

/// Performance optimization constants
pub mod performance {
    /// String interning pool initial capacity
    pub const STRING_POOL_CAPACITY: usize = 1000;

    /// File buffer size for reading operations
    pub const FILE_BUFFER_SIZE: usize = 8192;

    /// Image processing chunk size
    pub const IMAGE_CHUNK_SIZE: usize = 1024 * 1024; // 1MB

    /// Template cache size
    pub const TEMPLATE_CACHE_SIZE: usize = 100;
}

/// Logging and monitoring constants
pub mod logging {
    /// Default log level
    pub const DEFAULT_LOG_LEVEL: &str = "info";

    /// Log file pattern
    pub const LOG_FILE_PATTERN: &str = "genwebblog.%Y-%m-%d.log";

    /// Maximum log file size in bytes
    pub const MAX_LOG_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB

    /// Maximum number of log files to keep
    pub const MAX_LOG_FILES: u32 = 7; // One week
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constants_values() {
        assert_eq!(app::NAME, "GenWebBlog");
        assert_eq!(app::VERSION, "1.5.0");
        assert_eq!(paths::CONTENTS_DIR, "contents");
        assert_eq!(extensions::MARKDOWN, "md");
    }

    #[test]
    fn test_image_formats() {
        assert!(images::ALL_FORMATS.len() >= 3);
        assert!(images::ALL_FORMATS.contains(&"webp"));
        assert!(images::ALL_FORMATS.contains(&"jpg"));
    }

    #[test]
    fn test_validation_ranges() {
        assert!(validation::VALID_YEAR_RANGE.contains(&2024));
        assert!(!validation::VALID_YEAR_RANGE.contains(&2019));
        assert!(!validation::VALID_YEAR_RANGE.contains(&2031));
    }

    #[test]
    fn test_default_translations() {
        let translations = get_default_translations();
        assert!(translations.contains_key("th"));
        assert!(translations.contains_key("en"));

        if let Some(th) = translations.get("th") {
            assert!(th.contains_key("home"));
            assert_eq!(th.get("home"), Some(&"หน้าหลัก"));
        }
    }
}
