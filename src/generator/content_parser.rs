use serde_json::{json, Value};
use std::collections::HashSet;
use log::warn;

use crate::app::AppConfig;
use crate::blog::Article;
use crate::posts::Post;
use crate::shared::read_metadata_yml::GenericYmlInfo;

pub struct ContentParser<'a> {
    app: &'a AppConfig,
}

impl<'a> ContentParser<'a> {
    #[must_use]
    pub fn new(app: &'a AppConfig) -> Self {
        Self { app }
    }

    #[must_use]
    pub fn parse_articles_for_listing(
        &self,
        articles: &[Article],
        language_code: &str,
    ) -> Vec<Value> {
        articles
            .iter()
            .filter(|article| {
                (article.lang() == language_code)
                    || (article.lang().is_empty()
                        && language_code == self.app.languages.default_language)
            })
            .map(|article| {
                let url = self.generate_article_url(article, language_code);
                json!({
                    "title": article.title(),
                    "description": article.description(),
                    "url": url,
                    "path": article.prefix().display().to_string(),
                    "link_text": article.title(),
                    "post_count": article.posts().len(),
                    "date": article.date_published(),
                    "lang": article.lang(),
                })
            })
            .collect()
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn parse_posts_for_category(&self, posts: &[Post], category: &str) -> Vec<Post> {
        posts
            .iter()
            .filter(|post| post.category.as_deref() == Some(category))
            .cloned()
            .collect()
    }

    #[allow(dead_code)]
    #[must_use]
    pub fn extract_keywords_from_yml(&self, yml_info: &GenericYmlInfo) -> String {
        if yml_info.keywords.is_empty() {
            let mut keywords = Vec::new();

            if !yml_info.title.is_empty() {
                keywords.push(yml_info.title.clone());
            }

            if !yml_info.description.is_empty() {
                keywords.push(yml_info.description.clone());
            }

            keywords.join(", ")
        } else {
            yml_info.keywords.join(", ")
        }
    }

    pub fn validate_yml_content(&self, yml_info: &GenericYmlInfo) -> Result<(), String> {
        if yml_info.title.is_empty() {
            return Err("Title is required".to_string());
        }

        if yml_info.description.is_empty() {
            return Err("Description is required".to_string());
        }

        if yml_info.page_name.is_empty() {
            return Err("Page name is required".to_string());
        }

        if yml_info.title.len() > 60 {
            warn!("⚠️ Warning: Title is longer than 60 characters ({}), may be truncated in search results", yml_info.title.len());
        }

        if yml_info.description.len() > 160 {
            warn!("⚠️ Warning: Description is longer than 160 characters ({}), may be truncated in search results", yml_info.description.len());
        }

        if yml_info.page_name.contains(' ') || yml_info.page_name.contains('_') {
            warn!("⚠️ Warning: Page name '{}' contains spaces or underscores, consider using hyphens instead", yml_info.page_name);
        }

        Ok(())
    }

    #[must_use]
    pub fn filter_articles_by_language<'b>(
        &self,
        articles: &'b [Article],
        language_code: &str,
    ) -> Vec<&'b Article> {
        articles
            .iter()
            .filter(|article| {
                article.lang() == language_code
                    || (article.lang().is_empty()
                        && language_code == self.app.languages.default_language)
            })
            .collect()
    }

    #[must_use]
    pub fn generate_article_url(&self, article: &Article, language_code: &str) -> String {
        if language_code == self.app.languages.default_language {
            if article.prefix().as_os_str().is_empty() {
                "/".to_string()
            } else {
                format!("/{}", article.prefix().display())
            }
        } else {
            let prefix_str = article.prefix().to_string_lossy();
            if prefix_str.is_empty() {
                format!("/{language_code}")
            } else {
                format!("/{language_code}/{prefix_str}")
            }
        }
    }

    pub fn validate_article_urls(&self, articles: &[Article]) -> Result<(), String> {
        let mut url_set = HashSet::new();
        let mut duplicate_urls = Vec::new();

        for language_code in &self.app.languages.installed_languages {
            let filtered_articles = self.filter_articles_by_language(articles, language_code);

            for article in filtered_articles {
                let url = self.generate_article_url(article, language_code);
                if !url_set.insert(url.clone()) {
                    duplicate_urls.push(url);
                }
            }
        }

        if !duplicate_urls.is_empty() {
            return Err(format!(
                "Duplicate URLs found: {}",
                duplicate_urls.join(", ")
            ));
        }

        Ok(())
    }

    #[must_use]
    pub fn create_sitemap_entries(&self, articles: &[Article], base_url: &str) -> Vec<Value> {
        let mut entries = Vec::new();

        for lang_code in &self.app.languages.installed_languages {
            let filtered_articles = self.filter_articles_by_language(articles, lang_code);

            for article in filtered_articles {
                let relative_url = self.generate_article_url(article, lang_code);
                let full_url = if relative_url == "/" {
                    base_url.to_string()
                } else {
                    format!("{base_url}{relative_url}")
                };

                entries.push(json!({
                    "url": full_url,
                    "lastmod": article.date_published(),
                    "changefreq": "weekly",
                    "priority": "0.8",
                    "lang": lang_code
                }));
            }
        }

        entries
    }

    #[must_use]
    pub fn create_sitemap_entries_for_language(
        &self,
        articles: &[&Article],
        base_url: &str,
        language_code: &str,
    ) -> Vec<Value> {
        let mut entries = Vec::new();

        for article in articles {
            let relative_url = self.generate_article_url(article, language_code);
            let full_url = if relative_url == "/" {
                base_url.to_string()
            } else {
                format!("{base_url}{relative_url}")
            };

            entries.push(json!({
                "url": full_url,
                "lastmod": article.date_published(),
                "changefreq": "weekly",
                "priority": "0.8",
                "lang": language_code
            }));
        }

        entries
    }

    #[must_use]
    pub fn generate_robots_txt(&self, base_url: &str) -> String {
        use std::fmt::Write;

        let mut robots_content = String::new();
        robots_content.push_str("User-agent: *\n");
        robots_content.push_str("Allow: /\n");
        robots_content.push('\n');

        for lang_code in &self.app.languages.installed_languages {
            if lang_code == &self.app.languages.default_language {
                let _ = writeln!(robots_content, "Sitemap: {base_url}/sitemap.xml");
            } else {
                let _ = writeln!(
                    robots_content,
                    "Sitemap: {base_url}/{lang_code}/sitemap.xml"
                );
            }
        }

        robots_content
    }
}