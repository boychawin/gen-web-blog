use crate::{app::AppConfig, shared};
use eyre::{eyre, WrapErr};
use log::{error, warn};
use serde_derive::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

fn default_false() -> bool {
    false
}

fn default_layout() -> String {
    "post".to_string()
}

fn default_locale() -> String {
    "th_TH".to_string()
}

#[derive(Debug, PartialEq, Deserialize)]
struct YamlHeader {
    #[serde(default = "default_false")]
    draft: bool,
    #[serde(default = "default_layout")]
    layout: String,
    #[serde(default)]
    title: String,
    #[serde(default)]
    author: String,
    #[serde(rename = "author_url")]
    #[serde(default)]
    author_url: String,
    #[serde(rename = "author_email")]
    #[serde(default)]
    author_email: String,
    #[serde(default)]
    description: String,
    #[serde(default)]
    keywords: Option<Vec<String>>,
    #[serde(default)]
    tags: Option<Vec<String>>,
    #[serde(default)]
    link: Option<String>,
    #[serde(default)]
    link_name: Option<String>,
    #[serde(default)]
    html_code: Option<String>,
    #[serde(default)]
    date_modified: Option<String>,
    #[serde(default)]
    date_published: Option<String>,
    #[serde(default)]
    category: Option<String>,
    #[serde(default)]
    image: String,
    #[serde(default)]
    image_secure_url: String,
    #[serde(default)]
    image_type: String,
    #[serde(default)]
    image_width: String,
    #[serde(default)]
    image_height: String,
    #[serde(default)]
    image_alt: String,

    #[serde(default = "default_locale")]
    locale: String,
    #[serde(default = "default_locale")]
    locale_alternate: String,
    #[serde(default)]
    link_video: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub(crate) filename: String,
    pub(crate) layout: String,
    pub(crate) title: String,
    pub(crate) year: i32,
    pub(crate) show_year: bool,
    pub(crate) month: u32,
    pub(crate) day: u32,
    pub(crate) contents: String,
    pub(crate) app_domain: String,
    pub(crate) url: String,
    pub(crate) published: String,
    pub(crate) updated: String,
    pub(crate) draft: bool,
    pub(crate) author: String,
    pub(crate) author_url: String,
    pub(crate) author_email: String,
    pub(crate) image: String,
    pub(crate) image_secure_url: Option<String>,
    pub(crate) image_type: Option<String>,
    pub(crate) image_width: Option<String>,
    pub(crate) image_height: Option<String>,
    pub(crate) image_alt: Option<String>,
    pub(crate) description: String,
    pub(crate) keywords: String,
    pub(crate) tags: Option<Vec<String>>,
    pub(crate) root: String,
    pub(crate) is_image: bool,
    pub(crate) image_resize: String,
    pub(crate) link: Option<String>,
    pub(crate) link_name: Option<String>,
    pub(crate) html_code: Option<String>,
    pub(crate) date_modified: Option<String>,
    pub(crate) date_published: Option<String>,
    pub(crate) category: Option<String>,
    pub(crate) locale: Option<String>,
    pub(crate) locale_alternate: Option<String>,
    pub(crate) link_video: Option<String>,
}

impl Post {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn open(path: &Path, app: &AppConfig, prefix: &Path) -> eyre::Result<Self> {
        let filename = path
            .file_name()
            .and_then(|name| name.to_str())
            .ok_or_else(|| eyre!("Invalid filename in path: {:?}", path))?;

        let mut split = filename.splitn(4, '-');

        let year = split
            .next()
            .ok_or_else(|| eyre!("Missing year in filename: {}", filename))?
            .parse::<i32>()
            .wrap_err_with(|| format!("Invalid year in filename: {filename}"))?;
        let month = split
            .next()
            .ok_or_else(|| eyre!("Missing month in filename: {}", filename))?
            .parse::<u32>()
            .wrap_err_with(|| format!("Invalid month in filename: {filename}"))?;
        let day = split
            .next()
            .ok_or_else(|| eyre!("Missing day in filename: {}", filename))?
            .parse::<u32>()
            .wrap_err_with(|| format!("Invalid day in filename: {filename}"))?;
        let filename = split
            .next()
            .ok_or_else(|| eyre!("Missing title in filename: {}", filename))?
            .to_string();

        let contents = crate::shared::fs::read_file_to_string(path)?;
        if contents.len() < 5 {
            return Err(eyre!(
                "{path:?} is empty, or too short to have valid front matter"
            ));
        }

        let end_of_yaml = contents[4..]
            .find("---")
            .ok_or_else(|| eyre!("Missing closing '---' in front matter of file: {:?}", path))?
            + 4;
        let yaml = &contents[..end_of_yaml];
        let YamlHeader {
            author_url,
            author_email,
            author,
            title,
            draft,
            layout,
            image,
            image_secure_url,
            image_width,
            image_height,
            image_type,
            image_alt,
            description,
            keywords,
            tags,
            link,
            link_name,
            html_code,
            date_modified,
            date_published,
            category,
            locale,
            locale_alternate,
            link_video,
        } = serde_yaml::from_str(yaml)?;

        let options = comrak::Options {
            render: comrak::RenderOptions::builder().unsafe_(true).build(),
            extension: comrak::ExtensionOptions::builder()
                .header_ids(String::new())
                .strikethrough(true)
                .footnotes(true)
                .table(true)
                .build(),
            ..comrak::Options::default()
        };

        let contents = shared::markdown_to_html::comrak_custom::markdown_to_html(
            &contents[end_of_yaml + 4..],
            &options,
        );

        let mut url = PathBuf::from(&*filename);
        url.set_extension("html");

        // {:04}/{:02}/{:02}/
        let url = url
            .to_str()
            .ok_or_else(|| eyre!("Invalid URL path: {:?}", url))?
            .to_string();

        let published = build_time(year, month, day, 0);
        let updated = published.clone();
        let keywords: Vec<String> = keywords.unwrap_or_else(Vec::new);
        let formatted_keywords = keywords.join(", ");

        let full_url = if prefix.as_os_str().is_empty() {
            format!("/{url}")
        } else {
            let valid_prefix_clone = prefix;
            format!("/{}/{}", valid_prefix_clone.display(), url)
                .to_string()
                .to_string()
        };

        let is_image = !image.is_empty();
        let default_image = if image.is_empty() {
            shared::utils::get_default_post_image(&title)
        } else {
            "/favicon/favicon.svg".to_string()
        };
        let image = shared::utils::get_image(Some(&image), &default_image);
        let image_resize = if is_image && !image.ends_with(".svg") {
            shared::process_image::process_image(&image).unwrap_or_else(|e| {
                warn!("│  ⚠️ Error processing image: {e}");
                String::new()
            })
        } else {
            String::new()
        };

        let clean_image = image.trim_start_matches('/');
        let image_url = format!("{}/{}", app.app_info.app_domain, clean_image);

        let image_secure_url = if image_secure_url.is_empty() {
            Some(image_url.clone())
        } else {
            Some(image_secure_url.clone())
        };

        let image_width = if image_width.is_empty() {
            Some("1200".to_string())
        } else {
            Some(image_width.clone())
        };
        let image_height = if image_height.is_empty() {
            Some("650".to_string())
        } else {
            Some(image_height.clone())
        };
        let image_type = if image_type.is_empty() {
            Some("image/*".to_string())
        } else {
            Some(image_type.clone())
        };
        let image_alt = if image_alt.is_empty() {
            Some(title.clone())
        } else {
            Some(image_alt.clone())
        };

        Ok(Self {
            filename,
            title,
            author,
            author_url,
            author_email,
            year,
            show_year: false,
            month,
            day,
            contents,
            app_domain: app.app_info.app_domain.clone(),
            url: full_url,
            published,
            updated,
            draft,
            layout,
            image,
            image_secure_url,
            image_width,
            image_height,
            image_type,
            image_alt,
            image_resize,
            is_image,
            description,
            keywords: formatted_keywords,
            tags,
            link,
            link_name,
            html_code,
            root: "../".to_string(),
            date_modified,
            date_published,
            category,
            locale: Some(locale),
            locale_alternate: Some(locale_alternate),
            link_video,
        })
    }

    pub fn set_updated(&mut self, seconds: u32) {
        self.updated = build_time(self.year, self.month, self.day, seconds);
    }
}

fn build_time(year: i32, month: u32, day: u32, seconds: u32) -> String {
    let date = match chrono::NaiveDate::from_ymd_opt(year, month, day) {
        Some(d) => d,
        None => {
            warn!("Warning: Invalid date {year}-{month}-{day}, using current date");
            chrono::Utc::now().date_naive()
        }
    };

    let date_time_opt = date
        .and_hms_opt(0, 0, seconds)
        .or_else(|| {
            warn!("Warning: Invalid time with {seconds} seconds, trying 0 seconds");
            date.and_hms_opt(0, 0, 0)
        })
        .or_else(|| {
                error!("Failed to create valid time with given date/time, trying epoch 1970-01-01 00:00:00");
            chrono::NaiveDate::from_ymd_opt(1970, 1, 1).and_then(|d| d.and_hms_opt(0, 0, 0))
        });

    let date_time = match date_time_opt {
        Some(dt) => dt,
        None => {
            error!("All attempts to build a valid NaiveDateTime failed, using current UTC time");
            chrono::Utc::now().naive_utc()
        }
    };

    chrono::DateTime::<chrono::Utc>::from_naive_utc_and_offset(date_time, chrono::Utc).to_rfc3339()
}
