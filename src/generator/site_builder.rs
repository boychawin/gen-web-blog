use chrono::Timelike;
use handlebars::Handlebars;
use serde_derive::Serialize;
use serde_json::Value;
use std::collections::HashMap;
use std::path::{Path, PathBuf};

use crate::shared::handlebars::{create_hbs_options, register_all_templates_and_helpers};
use crate::shared::read_metadata_yml::{self, GenericYmlInfo};
use crate::validation::{FileValidator, ValidationConfig};
use crate::{
    app::{read_config, AppConfig},
    blog::{load, Article},
    posts::Post,
    tailwind,
};

use super::{
    asset_manager::AssetManager, content_parser::ContentParser, output_writer::OutputWriter,
    page_processor::PageProcessor,
};

pub struct Generator<'a> {
    handlebars: Handlebars<'a>,
    articles: Vec<Article>,
    out_directory: PathBuf,
    app: AppConfig,
    all_file_yml: Vec<GenericYmlInfo>,
    validator: FileValidator,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
struct Releases {
    version: String,
    posts: Vec<ReleasePost>,
    feed_updated: String,
}

#[derive(Debug, Serialize)]
#[allow(dead_code)]
pub(crate) struct ReleasePost {
    pub(crate) title: String,
    pub(crate) url: String,
}

#[derive(Serialize)]
pub struct Data<'a> {
    pub lang: String,
    pub locale: Option<String>,
    pub locale_alternate: Option<String>,
    pub link_video: Option<String>,
    pub title: String,
    pub description: String,
    pub path: String,
    pub url: String,
    pub image: String,
    pub keywords: String,
    pub root: String,
    pub site_root: String,
    pub article: &'a Article,
    pub articles: Option<Vec<Value>>,
    pub post: Option<Post>,
    pub posts: Option<Vec<Post>>,
    pub main_posts: Option<Vec<Post>>,
    pub date_modified: Option<String>,
    pub date_published: Option<String>,
    pub category: Option<String>,
    pub translations: Option<HashMap<String, String>>,
    pub app_name: String,
    pub app_author: String,
    pub app_version: String,
    pub app_domain: String,
    pub app_email: String,
    pub app_description: String,
    pub favicon_ico: String,
    pub apple_touch_icon: String,
    pub favicon_ico_16: String,
    pub favicon_ico_32: String,
    pub favicon_svg: String,
    pub mask_icon: String,
    pub web_manifest: String,
    pub type_page: String,
    pub app_facebook_link: String,
    pub app_instagram_link: String,
    pub app_x_link: String,
    pub app_youtube_link: String,
    pub app_line_link: String,
    pub app_github_link: String,
    pub app_phone: String,
    pub twitter_site: String,
    pub twitter_creator: String,
    pub robots_content: String,
    pub og_type: String,
    pub twitter_card: String,
    pub twitter_label1: String,
    pub google_adsense_client: String,
}

#[must_use]
pub fn get_locale_from_lang(lang_code: &str, app: &AppConfig) -> String {
    if let Some(locales) = &app.locales {
        match lang_code {
            code if code == app.languages.default_language => {
                if let Some(th) = &locales.th {
                    return th.clone();
                }
            }
            _ => {
                if let Some(en) = &locales.en {
                    return en.clone();
                }
            }
        }
    }

    "en_US".to_string()
}

impl Generator<'_> {
    /// Creates a new `ReleaseGenerator`
    ///
    /// # Errors
    ///
    /// Returns an error if the directories cannot be accessed or if there are file system issues
    pub fn new(
        out_directory: impl AsRef<Path>,
        posts_directory: impl AsRef<Path>,
    ) -> eyre::Result<Self> {
        let mut handlebars = Handlebars::new();
        // handlebars.set_strict_mode(true); // Disabled due to empty string root variable for default language

        let app = read_config();

        // Get template directories from config
        let source_layouts = app
            .paths
            .as_ref()
            .and_then(|p| p.source_layouts.as_ref())
            .cloned()
            .unwrap_or_else(|| "source/layouts".to_string());
        let source_pages = app
            .paths
            .as_ref()
            .and_then(|p| p.source_pages.as_ref())
            .cloned()
            .unwrap_or_else(|| "source/pages".to_string());
        let source_templates = app
            .paths
            .as_ref()
            .and_then(|p| p.source_templates.as_ref())
            .cloned()
            .unwrap_or_else(|| "source".to_string());

        // Register templates and default helpers
        let _ = register_all_templates_and_helpers(
            &mut handlebars,
            &[
                source_layouts.as_str(),
                source_pages.as_str(),
                source_templates.as_str(),
            ],
            &create_hbs_options(false),
        );

        let app = read_config();
        let contents_dir = app
            .paths
            .as_ref()
            .and_then(|p| p.contents_dir.as_ref())
            .cloned()
            .unwrap_or_else(|| "contents".to_string());
        let all_file_yml = read_metadata_yml::scan_yml_files_in_directory(&contents_dir);

        // Initialize validator with default config
        let validation_config = ValidationConfig::default();
        let validator = FileValidator::new(validation_config);

        Ok(Generator {
            handlebars,
            articles: load(posts_directory.as_ref(), &app)?,
            out_directory: out_directory.as_ref().to_path_buf(),
            app,
            all_file_yml,
            validator,
        })
    }

    /// Generates the release site
    ///
    /// # Errors
    ///
    /// Returns an error if site generation fails, template processing fails, or file operations fail
    pub fn generate(&self) -> eyre::Result<()> {
        println!("│  🏗️ Starting site generation...");

        // Run validation first
        self.run_validation()?;

        let asset_manager = AssetManager::new(&self.out_directory);
        let mut output_writer = OutputWriter::new(&self.out_directory);
        let content_parser = ContentParser::new(&self.app);
        let page_processor = PageProcessor::new(&self.handlebars, &self.app, &self.articles);

        asset_manager.create_build_directories()?;
        asset_manager.copy_static_assets()?;

        // Generate CSS
        let css_bundle = tailwind::process_tailwind_files();
        tailwind::save_css_to_file(&css_bundle, "build/_system_/styles/app.css");

        // Generate pages for each language
        for lang_code in &self.app.languages.installed_languages {
            self.generate_pages_for_language(
                lang_code,
                &page_processor,
                &mut output_writer,
                &content_parser,
            )?;
        }

        // Generate final sitemap combining all languages
        self.generate_final_sitemap(&content_parser, &output_writer)?;

        println!("│  ✅ Site generation completed!");
        Ok(())
    }

    fn generate_pages_for_language(
        &self,
        language_code: &str,
        page_processor: &PageProcessor,
        output_writer: &mut OutputWriter,
        content_parser: &ContentParser,
    ) -> eyre::Result<()> {
        println!("│    🌍 Generating pages for language: {language_code}");

        // Load translations for this language
        let translations = Self::load_translations(language_code);

        // Filter articles for this language using ContentParser
        let language_articles =
            content_parser.filter_articles_by_language(&self.articles, language_code);

        // Generate main pages
        for yml_info in &self.all_file_yml {
            // Skip articles.yml as it's handled separately by articles listing generation
            if yml_info.page_name == "articles" {
                continue;
            }

            // If yml_info has a language set, skip it when generating other languages
            if let Some(yml_lang) = &yml_info.lang {
                if yml_lang != language_code {
                    // Skip this YML for this language
                    continue;
                }
            }

            // Validate YML content using ContentParser
            if let Err(e) = content_parser.validate_yml_content(yml_info) {
                eprintln!(
                    "│    ⚠️ Warning: YML validation failed for {}: {}",
                    yml_info.page_name, e
                );
                continue;
            }

            // Find the article that matches this page_name
            let matching_article = language_articles
                .iter()
                .find(|article| {
                    let article_name = article
                        .prefix()
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");
                    article_name == yml_info.page_name
                })
                .or_else(|| language_articles.first()); // fallback to first article

            if let Some(article) = matching_article {
                // Prepare a mutable translations map so we can inject docs links html for docs page
                let mut translations_for_render = translations.clone();

                // If we're rendering the docs index, build an HTML list of links from contents/<lang>/docs/
                if yml_info.page_name == "docs" {
                    use std::path::Path;
                    let contents_dir = self
                        .app
                        .paths
                        .as_ref()
                        .and_then(|p| p.contents_dir.as_ref())
                        .cloned()
                        .unwrap_or_else(|| "contents".to_string());

                    let default_lang = self.app.languages.default_language.as_str();
                    // For default language, docs are in contents/docs; for others, contents/<lang>/docs
                    let docs_dir = if language_code == default_lang {
                        format!("{contents_dir}/docs")
                    } else {
                        format!("{contents_dir}/{language_code}/docs")
                    };

                    if Path::new(&docs_dir).exists() {
                        let mut items: Vec<String> = Vec::new();
                        for entry in std::fs::read_dir(&docs_dir)? {
                            let path = entry?.path();
                            if path.extension().and_then(|e| e.to_str()) == Some("md") {
                                // derive slug from filename (posts format YYYY-MM-DD-slug.md)
                                let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");
                                let slug = stem.splitn(4, '-').last().unwrap_or(stem);

                                // read title from frontmatter if available
                                let mut title = slug.to_string();
                                if let Ok(content) = crate::shared::fs::read_file_to_string(&path) {
                                    if content.len() > 5 && content.starts_with("---") {
                                        if let Some(pos) = content[3..].find("---") {
                                            let yaml = &content[..pos + 3];
                                            if let Ok(doc) =
                                                serde_yaml::from_str::<serde_yaml::Value>(yaml)
                                            {
                                                if let Some(t) =
                                                    doc.get("title").and_then(|v| v.as_str())
                                                {
                                                    title = t.to_string();
                                                }
                                            }
                                        }
                                    }
                                }

                                let url = if language_code == default_lang {
                                    format!("/docs/{slug}.html")
                                } else {
                                    format!("/{language_code}/docs/{slug}.html")
                                };

                                items.push(format!(
                                    r#"<li><a href="{}">{}</a></li>"#,
                                    url,
                                    html_escape::encode_text(&title)
                                ));
                            }
                        }

                        if !items.is_empty() {
                            let list_html = format!("<ul>{}</ul>", items.join("\n"));
                            translations_for_render
                                .insert("docs_links_html".to_string(), list_html);
                        }
                    }
                }

                let content = if yml_info.page_name == "index" {
                    // Special processing for index page
                    let articles_for_listing =
                        content_parser.parse_articles_for_listing(&self.articles, language_code);
                    page_processor.process_index_page(
                        article,
                        yml_info,
                        language_code,
                        &translations_for_render,
                        articles_for_listing,
                        &language_articles, // Pass all articles
                    )?
                } else {
                    // Regular page processing
                    page_processor.process_article_page(
                        article,
                        yml_info,
                        language_code,
                        &translations_for_render,
                    )?
                };

                let output_path =
                    self.get_output_path_for_language(language_code, &yml_info.page_name);
                output_writer.write_html_file(&output_path, &content)?;
            }
        }

        // Generate individual posts for each article (with deduplication)
        let mut generated_posts = std::collections::HashSet::new();
        for article in &language_articles {
            self.generate_posts_for_article(
                article,
                language_code,
                page_processor,
                output_writer,
                &translations,
                &mut generated_posts,
            )?;
        }

        // Generate articles listing page once per language (not per article)
        if !language_articles.is_empty() {
            self.generate_articles_listing_for_language(
                &language_articles,
                language_code,
                page_processor,
                output_writer,
                &translations,
            )?;
        }

        // Generate sitemap
        self.generate_sitemap(
            &language_articles,
            language_code,
            output_writer,
            content_parser,
        )?;

        // Generate releases.json for this language
        self.generate_releases_json(&language_articles, language_code, output_writer)?;

        // Note: Index page is now handled through regular page processing with special path logic

        Ok(())
    }

    fn generate_sitemap(
        &self,
        articles: &[&Article],
        language_code: &str,
        output_writer: &mut OutputWriter,
        content_parser: &ContentParser,
    ) -> eyre::Result<()> {
        println!("│    🗺️ Generating sitemap for language: {language_code}");

        // Create sitemap entries for this specific language
        let sitemap_entries = content_parser.create_sitemap_entries_for_language(
            articles,
            &self.app.app_info.app_domain,
            language_code,
        );
        let sitemap_json = serde_json::Value::Array(sitemap_entries.clone());

        let sitemap_path = if language_code == self.app.languages.default_language {
            "sitemap.json".to_string()
        } else {
            format!("{language_code}/sitemap.json")
        };

        output_writer.write_json_file(&sitemap_path, &sitemap_json)?;
        println!(
            "│    ✅ Generated sitemap: {} with {} entries",
            sitemap_path,
            sitemap_entries.len()
        );
        Ok(())
    }

    fn generate_releases_json(
        &self,
        articles: &[&Article],
        language_code: &str,
        output_writer: &mut OutputWriter,
    ) -> eyre::Result<()> {
        use serde_derive::Serialize;

        #[derive(Serialize)]
        struct ReleasePost {
            title: String,
            url: String,
        }

        #[derive(Serialize)]
        struct Releases {
            version: String,
            posts: Vec<ReleasePost>,
            feed_updated: String,
        }

        let mut all_releases: Vec<ReleasePost> = Vec::new();

        for article in articles {
            let posts = article.posts();
            let is_released: Vec<&crate::posts::Post> =
                posts.iter().filter(|post| !post.draft).collect();

            let releases: Vec<ReleasePost> = is_released
                .iter()
                .map(|post| ReleasePost {
                    title: post.title.clone(),
                    url: post.url.clone(),
                })
                .collect();

            all_releases.extend(releases);
        }

        let data = Releases {
            version: self.app.app_info.app_version.clone(),
            posts: all_releases,
            feed_updated: chrono::Utc::now()
                .with_nanosecond(0)
                .unwrap_or_else(chrono::Utc::now)
                .to_rfc3339(),
        };

        let default_lang = self.app.languages.default_language.as_str();
        let releases_path = if language_code == default_lang {
            "releases.json".to_string()
        } else {
            format!("{language_code}/releases.json")
        };

        output_writer.write_json_file(&releases_path, &serde_json::to_value(&data)?)?;
        println!("│    ✅ Generated: releases.json");
        Ok(())
    }

    fn load_translations(language_code: &str) -> HashMap<String, String> {
        let translation_path = format!("source/translations/{language_code}.toml");
        if Path::new(&translation_path).exists() {
            match crate::shared::fs::read_file_to_string(&translation_path) {
                Ok(content) => match toml::from_str::<HashMap<String, String>>(&content) {
                    Ok(translations) => {
                        println!("│    ✅ Loaded translations for {language_code}");
                        translations
                    }
                    Err(e) => {
                        eprintln!(
                            "│    ⚠️ Failed to parse translation file {translation_path}: {e}"
                        );
                        HashMap::new()
                    }
                },
                Err(e) => {
                    eprintln!("│    ⚠️ Failed to read translation file {translation_path}: {e}");
                    HashMap::new()
                }
            }
        } else {
            eprintln!("│    ⚠️ Translation file not found: {translation_path}, using defaults");
            HashMap::new()
        }
    }

    fn generate_final_sitemap(
        &self,
        content_parser: &ContentParser,
        output_writer: &OutputWriter,
    ) -> eyre::Result<()> {
        println!("│    🗺️ Generating final sitemap...");

        // Create sitemap entries for all articles and languages
        let mut all_sitemap_entries = Vec::new();

        for language_code in &self.app.languages.installed_languages {
            let language_articles =
                content_parser.filter_articles_by_language(&self.articles, language_code);
            let entries = content_parser.create_sitemap_entries_for_language(
                &language_articles,
                &self.app.app_info.app_domain,
                language_code,
            );
            all_sitemap_entries.extend(entries);
        }

        // Convert to JSON and save
        let sitemap_json = serde_json::Value::Array(all_sitemap_entries);
        output_writer.write_json_file("sitemap.json", &sitemap_json)?;

        println!(
            "│    ✅ Generated final sitemap with {} entries",
            sitemap_json.as_array().map_or(0, std::vec::Vec::len)
        );
        Ok(())
    }

    #[allow(clippy::unused_self)]
    fn generate_posts_for_article(
        &self,
        article: &Article,
        language_code: &str,
        page_processor: &PageProcessor,
        output_writer: &OutputWriter,
        translations: &std::collections::HashMap<String, String>,
        generated_posts: &mut std::collections::HashSet<String>,
    ) -> eyre::Result<()> {
        // Generate individual post pages for this article
        for post in article.posts() {
            if post.draft {
                println!("│    📝 Draft: {} ({})", post.title, post.url);
                continue;
            }

            // Check if this post was already generated
            let post_key = format!("{}:{}", language_code, post.filename);
            if generated_posts.contains(&post_key) {
                continue;
            }

            let post_content =
                page_processor.process_post_page(article, post, language_code, translations)?;

            // Determine output path for post (include article prefix for subdirectory)
            let post_output_path =
                self.get_post_output_path_for_language(language_code, post, article);
            output_writer.write_html_file(&post_output_path, &post_content)?;

            // Mark this post as generated
            generated_posts.insert(post_key);
        }

        Ok(())
    }

    #[allow(clippy::too_many_lines)]
    fn generate_articles_listing_for_language(
        &self,
        articles: &[&Article],
        language_code: &str,
        page_processor: &PageProcessor,
        output_writer: &OutputWriter,
        _translations: &std::collections::HashMap<String, String>,
    ) -> eyre::Result<()> {
        // Read title and description from articles.yml
        let articles_yaml_path = std::path::Path::new("contents").join("articles.yml");
        // Load translations for this language
        let translations = Self::load_translations(language_code);

        let (title, description) = if articles_yaml_path.exists() {
            let yaml_content = crate::shared::fs::read_file_to_string(&articles_yaml_path)?;
            let yaml_data: serde_yaml::Value = serde_yaml::from_str(&yaml_content)?;

            let title = yaml_data
                .get("title")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| {
                    translations
                        .get("articles_title")
                        .map_or("บทความ", |s| s.as_str())
                })
                .to_string();

            let description = yaml_data
                .get("description")
                .and_then(|v| v.as_str())
                .unwrap_or_else(|| {
                    translations
                        .get("articles_subtitle")
                        .map_or("รวมบทความและเนื้อหาที่น่าสนใจ", std::string::String::as_str)
                })
                .to_string();

            (title, description)
        } else {
            let title = translations
                .get("articles_title")
                .cloned()
                .unwrap_or_else(|| {
                    if language_code == "en" {
                        "Articles".to_string()
                    } else {
                        "บทความ".to_string()
                    }
                });
            let description = translations
                .get("articles_subtitle")
                .cloned()
                .unwrap_or_else(|| {
                    if language_code == "en" {
                        "Collection of interesting articles and content".to_string()
                    } else {
                        "รวมบทความและเนื้อหาที่น่าสนใจ".to_string()
                    }
                });
            (title, description)
        };

        // Use the first article for date information (or current date if none)
        let date_published = articles.first().map_or_else(
            || {
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs()
                    .to_string()
            },
            |a| a.date_published().clone(),
        );

        // Create GenericYmlInfo for articles listing page
        let keywords = if language_code == "en" {
            vec![
                "Articles".to_string(),
                "Blog".to_string(),
                "Content".to_string(),
            ]
        } else {
            vec![
                "บทความ".to_string(),
                "บล็อก".to_string(),
                "เนื้อหา".to_string(),
            ]
        };

        let articles_yml = GenericYmlInfo {
            page_name: "articles".to_string(),
            title: title.clone(),
            description: description.clone(),
            keywords,
            image: None,
            draft: None,
            date_published: Some(date_published.clone()),
            date_modified: Some(date_published),
            lang: Some(language_code.to_string()),
            layout: Some("articles".to_string()),
            category: None,
            link_text: None,
            author: None,
            author_url: None,
            author_email: None,
        };

        // Use articles listing processing (should create one from existing article processing)
        let articles_content = if let Some(first_article) = articles.first() {
            page_processor.process_article_page(
                first_article,
                &articles_yml,
                language_code,
                &translations,
            )?
        } else {
            return Ok(()); // Skip if no articles
        };

        // Determine output path for articles listing
        let default_lang = self.app.languages.default_language.as_str();
        let articles_output_path = if language_code == default_lang {
            "articles/index.html".to_string()
        } else {
            format!("{language_code}/articles/index.html")
        };

        output_writer.write_html_file(&articles_output_path, &articles_content)?;

        Ok(())
    }

    fn get_post_output_path_for_language(
        &self,
        language_code: &str,
        post: &Post,
        article: &Article,
    ) -> String {
        // Include article prefix in post path to handle subdirectories
        let mut filename = std::path::PathBuf::from(&post.filename);
        filename.set_extension("html");

        // Get article prefix relative path (e.g., "web-optimization" for subdirectory)
        let article_prefix = article.prefix();

        let default_lang = self.app.languages.default_language.as_str();

        if language_code == default_lang {
            if article_prefix.as_os_str().is_empty() {
                // Root article - place directly in build root
                filename.to_string_lossy().to_string()
            } else {
                // Subdirectory article - place in subdirectory
                format!(
                    "{}/{}",
                    article_prefix.to_string_lossy(),
                    filename.to_string_lossy()
                )
            }
        } else if article_prefix.as_os_str().is_empty() {
            // Root article - place in language directory
            format!("{}/{}", language_code, filename.to_string_lossy())
        } else {
            // If the article prefix equals the language code (e.g., contents/en/...)
            // avoid producing duplicated path like `en/en/...` and place under `en/...` instead.
            let prefix_str = article_prefix.to_string_lossy();
            if prefix_str == language_code {
                format!("{}/{}", language_code, filename.to_string_lossy())
            } else {
                // Subdirectory article - place in language/subdirectory
                format!(
                    "{}/{}/{}",
                    language_code,
                    article_prefix.to_string_lossy(),
                    filename.to_string_lossy()
                )
            }
        }
    }

    fn get_output_path_for_language(&self, language_code: &str, page_name: &str) -> String {
        let use_directory = self
            .app
            .paths
            .as_ref()
            .and_then(|p| p.use_directory_structure)
            .unwrap_or(true);

        // Special handling for index page - always stays as file
        let default_lang = self.app.languages.default_language.as_str();
        if page_name == "index" {
            if language_code == default_lang {
                "index.html".to_string()
            } else {
                format!("{language_code}/index.html")
            }
        } else if use_directory {
            // Directory structure: /about/index.html
            if language_code == default_lang {
                format!("{page_name}/index.html")
            } else {
                format!("{language_code}/{page_name}/index.html")
            }
        } else {
            // Direct file: /about.html
            if language_code == default_lang {
                format!("{page_name}.html")
            } else {
                format!("{language_code}/{page_name}.html")
            }
        }
    }

    /// Run file validation on contents and public directories
    fn run_validation(&self) -> eyre::Result<()> {
        println!("│  🔍 Running file validation...");

        let mut has_errors = false;

        // Validate contents directory
        let contents_dir = self
            .app
            .paths
            .as_ref()
            .and_then(|p| p.contents_dir.as_ref())
            .cloned()
            .unwrap_or_else(|| "contents".to_string());

        if Path::new(&contents_dir).exists() {
            let contents_summary = self
                .validator
                .validate_directory(Path::new(&contents_dir))?;
            if contents_summary.has_errors() {
                has_errors = true;
            }
            contents_summary.print_summary();
        }

        // Validate public directory
        let public_dir = self
            .app
            .paths
            .as_ref()
            .and_then(|p| p.public_dir.as_ref())
            .cloned()
            .unwrap_or_else(|| "public".to_string());

        if Path::new(&public_dir).exists() {
            let public_summary = self.validator.validate_directory(Path::new(&public_dir))?;
            if public_summary.has_errors() {
                has_errors = true;
            }
            // ไม่ต้องแสดง public_summary.print_summary();
        }

        if has_errors {
            println!("│  ⚠️  Validation completed with errors - continuing with build...");
        } else {
            println!("│  ✅ Validation completed successfully!");
        }

        Ok(())
    }
}
