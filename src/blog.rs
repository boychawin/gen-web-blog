use crate::app::AppConfig;
use crate::shared::utils::add_postfix_slash;
use log::info;

use super::posts::Post;
use serde_derive::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

static MANIFEST_FILE: &str = "index.yml";
static POSTS_EXT: &str = "md";

fn default_layout() -> String {
    "articles".to_string()
}

fn default_lang() -> String {
    use crate::language::LanguageManager;

    match LanguageManager::load_from_file() {
        Ok(manager) => manager
            .get_default_language()
            .unwrap_or_else(|| "en".to_string()),
        Err(_) => "en".to_string(),
    }
}
fn default_false() -> bool {
    false
}
#[derive(Deserialize)]
#[serde(rename_all = "kebab-case", deny_unknown_fields)]
pub struct Manifest {
    #[serde(default = "default_layout")]
    layout: String,

    #[serde(default = "default_lang")]
    lang: String,

    #[serde(rename = "title")]
    #[serde(default)]
    title: String,

    #[serde(rename = "description")]
    #[serde(default)]
    description: String,

    #[serde(rename = "image")]
    #[serde(default)]
    image: String,

    #[serde(rename = "link_text")]
    #[serde(default)]
    link_text: String,

    #[serde(rename = "keywords")]
    #[serde(default)]
    keywords: Vec<String>,

    #[serde(rename = "author")]
    #[serde(default)]
    author: String,

    #[serde(rename = "author_url")]
    #[serde(default)]
    author_url: String,

    #[serde(rename = "author_email")]
    #[serde(default)]
    author_email: String,

    #[serde(default = "default_false")]
    pub draft: bool,

    #[serde(default)]
    #[serde(rename = "date_modified")]
    date_modified: Option<String>,
    #[serde(default)]
    #[serde(rename = "date_published")]
    date_published: Option<String>,
    #[serde(default)]
    category: Option<String>,
}

#[derive(Serialize)]
pub struct Article {
    pub layout: String,
    pub title: String,
    pub description: String,
    pub image: String,
    #[serde(serialize_with = "add_postfix_slash")]
    pub prefix: PathBuf,
    pub app_domain: String,
    pub posts: Vec<Post>,
    pub link_text: String,
    pub keywords: Vec<String>,
    pub author: String,
    pub author_url: String,
    pub author_email: String,
    pub lang: String,
    pub draft: bool,
    pub date_modified: String,
    pub date_published: String,
    pub category: String,
}

impl Article {
    fn load(prefix: PathBuf, dir: &Path, app: &AppConfig) -> eyre::Result<Self> {
    let manifest_content = crate::shared::fs::read_file_to_string(dir.join(MANIFEST_FILE))?;
        let manifest: Manifest = serde_yaml::from_str(&manifest_content)?;

        let mut posts = Vec::new();
        for entry in std::fs::read_dir(dir)? {
            let path = entry?.path();
            let ext = path.extension().and_then(|e| e.to_str());
            if path.metadata()?.file_type().is_file() && ext == Some(POSTS_EXT) {
                posts.push(Post::open(&path, app, &prefix)?);
            }
        }

        let releases = if posts.is_empty() {
            Vec::new()
        } else {
            process_posts(&mut posts)
        };
        Ok(Self {
            layout: manifest.layout,
            title: manifest.title,
            description: manifest.description,
            image: manifest.image,
            author: manifest.author,
            author_url: manifest.author_url,
            author_email: manifest.author_email,
            prefix,
            app_domain: app.app_info.app_domain.clone(),
            posts: releases,
            link_text: manifest.link_text,
            keywords: manifest.keywords,
            lang: manifest.lang,
            draft: manifest.draft,
            date_modified: manifest.date_modified.unwrap_or_default(),
            date_published: manifest.date_published.unwrap_or_default(),
            category: manifest.category.unwrap_or_default(),
        })
    }

    pub(crate) fn title(&self) -> &str {
        &self.title
    }

    pub(crate) fn description(&self) -> &str {
        &self.description
    }

    pub(crate) fn prefix(&self) -> &Path {
        &self.prefix
    }

    pub(crate) fn posts(&self) -> &[Post] {
        &self.posts
    }

    pub(crate) fn lang(&self) -> &String {
        &self.lang
    }

    pub(crate) fn date_published(&self) -> &String {
        &self.date_published
    }
}

pub fn load(base: &Path, app: &AppConfig) -> eyre::Result<Vec<Article>> {
    let mut articles = Vec::new();
    load_recursive(base, base, &mut articles, app)?;
    Ok(articles)
}

fn load_recursive(
    base: &Path,
    current: &Path,
    articles: &mut Vec<Article>,
    app: &AppConfig,
) -> eyre::Result<()> {
    for entry in std::fs::read_dir(current)? {
        let path = entry?.path();
        let file_type = path.metadata()?.file_type();

        if file_type.is_dir() {
            load_recursive(base, &path, articles, app)?;
        } else if file_type.is_file() {
            let file_name = path.file_name().and_then(|n| n.to_str());
            if let (Some(file_name), Some(parent)) = (file_name, path.parent()) {
                if file_name == MANIFEST_FILE {
                    let raw_prefix = parent
                        .strip_prefix(base)
                        .map_or_else(|_| PathBuf::new(), Path::to_path_buf);

                    let mut prefix = raw_prefix.clone();
                    if let Some(first) = prefix.iter().next() {
                        if let Some(first_str) = first.to_str() {
                            if first_str.len() == 2
                                && first_str.chars().all(|c| c.is_ascii_alphabetic())
                            {
                                let mut new_prefix = PathBuf::new();
                                for (i, comp) in prefix.iter().enumerate() {
                                    if i == 0 {
                                        continue;
                                    }
                                    new_prefix.push(comp);
                                }
                                prefix = new_prefix;
                            }
                        }
                    }

                    articles.push(Article::load(prefix, parent, app)?);
                }
            }
        }
    }
    Ok(())
}

fn process_posts(posts: &mut Vec<Post>) -> Vec<Post> {
    posts.sort_by_key(|post| {
        format!(
            "{}-{:02}-{:02}-{}",
            post.year, post.month, post.day, post.title
        )
    });
    posts.reverse();

    if let Some(first_post) = posts.first_mut() {
        first_post.show_year = true;
    }
    for i in 1..posts.len() {
        posts[i].show_year = posts[i - 1].year != posts[i].year;
    }

    let mut last_matching_updated = 0;
    for i in 1..posts.len() {
        if posts[i].updated == posts[last_matching_updated].updated {
            if let Ok(updated_value) = u32::try_from(i - last_matching_updated) {
                posts[i].set_updated(updated_value);
            }
        } else {
            last_matching_updated = i;
        }
    }

    let (released_posts, unreleased_posts): (Vec<Post>, Vec<Post>) =
        posts.drain(..).partition(|post| !post.draft);

    for post in &unreleased_posts {
        info!("│  📝 Draft: {} ({})\n", post.title, post.url);
    }

    released_posts
}