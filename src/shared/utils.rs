pub use crate::shared::css::{compile_sass, concat_vendor_css};
pub use crate::shared::fs::{copy_dir_contents, copy_static_files, delete_file_if_exists};
pub use crate::shared::url::{file_url, get_default_post_image, get_image};

use std::path::Path;
use std::time::Duration;

/// Serialize a path as a string and ensure it has a single trailing slash when non-empty.
pub fn add_postfix_slash<S>(path: &Path, serializer: S) -> std::result::Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    let mut str_repr = path.to_string_lossy().to_string();
    if !str_repr.is_empty() && !str_repr.ends_with('/') {
        str_repr.push('/');
    }
    serializer.serialize_str(&str_repr)
}

/// Quick online check with a short timeout. Returns true when a successful response is received.
pub async fn is_online() -> bool {
    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(3))
        .build()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    client
        .get("https://www.google.com")
        .send()
        .await
        .map(|resp| resp.status().is_success())
        .unwrap_or(false)
}

/// Join keywords with a comma separator. `join` already returns an empty string for empty slices.
pub fn join_keywords(keywords: &[String]) -> String {
    keywords.join(", ")
}

/// Return a cloned String from an Option<String> or the default
pub fn get_string_or_default(opt: &Option<String>, default: &str) -> String {
    opt.as_deref()
        .filter(|s| !s.is_empty())
        .unwrap_or(default)
        .to_string()
}

/// Return a string slice from an Option<String> or the default
pub fn get_string_ref_or_default<'a, U>(opt: &'a Option<U>, default: &'a str) -> &'a str
where
    U: AsRef<str> + 'a,
{
    opt.as_ref()
        .map(|u| u.as_ref())
        .filter(|s| !s.is_empty())
        .unwrap_or(default)
}

// Constants moved to constants module - re-export for backward compatibility
pub const DEFAULT_FAVICON_ICO: &str = "/favicon/favicon.ico";
pub const DEFAULT_APPLE_TOUCH_ICON: &str = "/favicon/apple-touch-icon.png";
pub const DEFAULT_FAVICON_16: &str = "/favicon/favicon-16x16.png";
pub const DEFAULT_FAVICON_32: &str = "/favicon/favicon-32x32.png";
pub const DEFAULT_FAVICON_SVG: &str = "/favicon/favicon.svg";
pub const DEFAULT_MASK_ICON: &str = "/favicon/mask-icon.svg";
pub const DEFAULT_WEB_MANIFEST: &str = "/site.webmanifest";
pub const DEFAULT_OG_TYPE: &str = "website";
pub const DEFAULT_TWITTER_CARD: &str = "summary_large_image";
pub const DEFAULT_ROBOTS_CONTENT: &str = "follow, index";

/// Calculate reading time based on word count
pub fn calculate_reading_time(word_count: usize, wpm: u32) -> u32 {
    let wpm = if wpm == 0 { 200 } else { wpm };
    ((word_count as f64 / wpm as f64).ceil() as u32).max(1)
}

/// Extract excerpt from content using character-aware truncation so multi-byte UTF-8 isn't split.
pub fn extract_excerpt(content: &str, max_length: usize) -> String {
    let char_count = content.chars().count();
    if char_count <= max_length {
        content.to_string()
    } else {
        let truncated: String = content.chars().take(max_length).collect();
        if let Some(last_space_idx) = truncated.rfind(' ') {
            let prefix = &truncated[..last_space_idx];
            format!("{prefix}...")
        } else {
            format!("{truncated}...")
        }
    }
}

/// Validate file extension against allowed formats (case-insensitive). Avoids extra allocations.
pub fn validate_file_extension(path: &Path, allowed_extensions: &[&str]) -> bool {
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        allowed_extensions
            .iter()
            .any(|a| ext.eq_ignore_ascii_case(a))
    } else {
        false
    }
}
