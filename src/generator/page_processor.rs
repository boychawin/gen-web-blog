use handlebars::Handlebars;
use serde_json::Value;
use std::collections::HashMap;

use crate::app::AppConfig;
use crate::blog::Article;
use crate::posts::Post;
use crate::shared::read_metadata_yml::GenericYmlInfo;
use crate::shared::utils::{
    get_string_or_default, get_string_ref_or_default, join_keywords, DEFAULT_APPLE_TOUCH_ICON,
    DEFAULT_FAVICON_16, DEFAULT_FAVICON_32, DEFAULT_FAVICON_ICO, DEFAULT_FAVICON_SVG,
    DEFAULT_MASK_ICON, DEFAULT_OG_TYPE, DEFAULT_ROBOTS_CONTENT, DEFAULT_TWITTER_CARD,
    DEFAULT_WEB_MANIFEST,
};

use super::site_builder::{get_locale_from_lang, Data};

pub struct PageProcessor<'a> {
    handlebars: &'a Handlebars<'a>,
    app: &'a AppConfig,
    articles: &'a [Article],
}

impl<'a> PageProcessor<'a> {
    #[must_use]
    pub fn new(
        handlebars: &'a Handlebars<'a>,
        app: &'a AppConfig,
        articles: &'a [Article],
    ) -> Self {
        Self {
            handlebars,
            app,
            articles,
        }
    }

    pub fn process_article_page(
        &self,
        article: &Article,
        yml_info: &GenericYmlInfo,
        language_code: &str,
        translations: &HashMap<String, String>,
    ) -> eyre::Result<String> {
        let mut data = self.create_page_data(article, yml_info, language_code, translations);

        let template_name = yml_info.layout.as_ref().unwrap_or(&yml_info.page_name);
        if template_name == "articles" {
            let content_parser = crate::generator::content_parser::ContentParser::new(self.app);
            let articles_data =
                content_parser.parse_articles_for_listing(self.articles, language_code);
            data.articles = Some(articles_data);
        }

        let mut main_posts = Vec::new();
        for article in self.articles {
            if (article.lang() == language_code
                || (article.lang().is_empty()
                    && language_code == self.app.languages.default_language))
                && article.prefix().as_os_str().is_empty()
            {
                main_posts.extend(article.posts().to_vec());
            }
        }
        main_posts.sort_by(|a, b| b.date_published.cmp(&a.date_published));
        data.main_posts = Some(main_posts);

        data.type_page = self.get_type_page_for_template(&yml_info.page_name);

        let content = self.handlebars.render(template_name, &data)?;
        Ok(content)
    }

    pub fn process_index_page(
        &self,
        article: &Article,
        yml_info: &GenericYmlInfo,
        language_code: &str,
        translations: &HashMap<String, String>,
        articles: Vec<Value>,
        all_articles: &[&Article],
    ) -> eyre::Result<String> {
        let mut data = self.create_page_data(article, yml_info, language_code, translations);
        data.articles = Some(articles);

        let mut all_posts = Vec::new();
        for article in all_articles {
            all_posts.extend(article.posts().to_vec());
        }
        all_posts.sort_by(|a, b| b.date_published.cmp(&a.date_published));
        data.posts = Some(all_posts);

        data.type_page = self.get_type_page_for_template("index");

        let content = self.handlebars.render("index", &data)?;
        Ok(content)
    }

    fn get_type_page_for_template(&self, template_name: &str) -> String {
        if let Some(seo) = &self.app.seo {
            match template_name {
                "index" => get_string_or_default(&seo.type_page_home, "WebSite").to_string(),
                "about" => get_string_or_default(&seo.type_page_about, "AboutPage").to_string(),
                "contact" => {
                    get_string_or_default(&seo.type_page_contact, "ContactPage").to_string()
                }
                "articles" | "blog" => {
                    get_string_or_default(&seo.type_page_blog, "BlogPosting").to_string()
                }
                _ => get_string_or_default(&seo.type_page_default, "WebPage").to_string(),
            }
        } else {
            "WebPage".to_string()
        }
    }

    #[allow(clippy::too_many_lines)]
    fn create_page_data<'b>(
        &self,
        article: &'b Article,
        yml_info: &GenericYmlInfo,
        language_code: &str,
        translations: &HashMap<String, String>,
    ) -> Data<'b> {
        let locale = get_locale_from_lang(language_code, self.app);
        let root = if language_code == self.app.languages.default_language {
            ""
        } else {
            &format!("/{language_code}")
        };
        let path = if language_code == self.app.languages.default_language {
            if yml_info.page_name == "index" {
                "/".to_string()
            } else {
                format!("/{}", yml_info.page_name)
            }
        } else if yml_info.page_name == "index" {
            format!("/{language_code}")
        } else {
            format!("/{}/{}", language_code, yml_info.page_name)
        };
        let url = format!("{}{}", self.app.app_info.app_domain, &path);

        let (title, description, keywords) = if language_code == self.app.languages.default_language
        {
            (
                yml_info.title.clone(),
                yml_info.description.clone(),
                join_keywords(&yml_info.keywords),
            )
        } else {
            let translated_title = translations
                .get("title")
                .cloned()
                .unwrap_or_else(|| yml_info.title.clone());
            let translated_description = translations
                .get("description")
                .cloned()
                .unwrap_or_else(|| yml_info.description.clone());
            let translated_keywords = if yml_info.page_name == "index" {
                translations
                    .get("keywords")
                    .cloned()
                    .unwrap_or_else(|| join_keywords(&yml_info.keywords))
            } else {
                join_keywords(&yml_info.keywords)
            };
            (
                translated_title,
                translated_description,
                translated_keywords,
            )
        };

        Data {
            lang: language_code.to_string(),
            locale: Some(locale.clone()),
            locale_alternate: Some(if language_code == self.app.languages.default_language {
                get_string_ref_or_default(
                    &self.app.locales.as_ref().and_then(|l| l.en.as_ref()),
                    "en_US",
                )
                .to_string()
            } else {
                get_string_ref_or_default(
                    &self.app.locales.as_ref().and_then(|l| l.th.as_ref()),
                    "th_TH",
                )
                .to_string()
            }),
            link_video: None,
            title,
            description,
            path,
            url,
            image: get_string_or_default(&yml_info.image, "").to_string(),
            keywords,
            root: root.to_string(),
            site_root: self.app.app_info.app_domain.clone(),
            article,
            articles: None,
            post: None,
            posts: Some(article.posts().to_vec()),
            main_posts: None,
            date_modified: None,
            date_published: None,
            category: None,
            translations: Some(translations.clone()),
            app_name: self.app.app_info.app_name.clone(),
            app_author: get_string_or_default(&self.app.app_info.app_author, "").to_string(),
            app_version: self.app.app_info.app_version.clone(),
            app_domain: self.app.app_info.app_domain.clone(),
            app_email: get_string_or_default(&self.app.app_info.app_email, "").to_string(),
            app_description: get_string_or_default(&self.app.app_info.app_description, "")
                .to_string(),
            favicon_ico: get_string_ref_or_default(
                &self.app.favicon.as_ref().and_then(|f| f.ico.as_ref()),
                DEFAULT_FAVICON_ICO,
            )
            .to_string(),
            apple_touch_icon: get_string_ref_or_default(
                &self
                    .app
                    .favicon
                    .as_ref()
                    .and_then(|f| f.apple_touch_icon.as_ref()),
                DEFAULT_APPLE_TOUCH_ICON,
            )
            .to_string(),
            favicon_ico_16: get_string_ref_or_default(
                &self.app.favicon.as_ref().and_then(|f| f.ico_16.as_ref()),
                DEFAULT_FAVICON_16,
            )
            .to_string(),
            favicon_ico_32: get_string_ref_or_default(
                &self.app.favicon.as_ref().and_then(|f| f.ico_32.as_ref()),
                DEFAULT_FAVICON_32,
            )
            .to_string(),
            favicon_svg: get_string_ref_or_default(
                &self.app.favicon.as_ref().and_then(|f| f.svg.as_ref()),
                DEFAULT_FAVICON_SVG,
            )
            .to_string(),
            mask_icon: get_string_ref_or_default(
                &self.app.favicon.as_ref().and_then(|f| f.mask_icon.as_ref()),
                DEFAULT_MASK_ICON,
            )
            .to_string(),
            web_manifest: get_string_ref_or_default(
                &self
                    .app
                    .favicon
                    .as_ref()
                    .and_then(|f| f.web_manifest.as_ref()),
                DEFAULT_WEB_MANIFEST,
            )
            .to_string(),
            type_page: self.get_type_page_for_template(&yml_info.page_name),
            app_facebook_link: get_string_or_default(&self.app.app_info.app_facebook_link, "")
                .to_string(),
            app_instagram_link: get_string_or_default(&self.app.app_info.app_instagram_link, "")
                .to_string(),
            app_x_link: get_string_or_default(&self.app.app_info.app_x_link, "").to_string(),
            app_youtube_link: get_string_or_default(&self.app.app_info.app_youtube_link, "")
                .to_string(),
            app_line_link: get_string_or_default(&self.app.app_info.app_line_link, "").to_string(),
            app_github_link: get_string_or_default(&self.app.app_info.app_github_link, "")
                .to_string(),
            app_phone: get_string_or_default(&self.app.app_info.app_phone, "").to_string(),
            twitter_site: get_string_or_default(&self.app.twitter.twitter_site, "").to_string(),
            twitter_creator: get_string_or_default(&self.app.twitter.twitter_creator, "")
                .to_string(),
            robots_content: get_string_ref_or_default(
                &self
                    .app
                    .seo
                    .as_ref()
                    .and_then(|s| s.robots_content.as_ref()),
                DEFAULT_ROBOTS_CONTENT,
            )
            .to_string(),
            og_type: get_string_ref_or_default(
                &self
                    .app
                    .social_meta
                    .as_ref()
                    .and_then(|s| s.og_type.as_ref()),
                DEFAULT_OG_TYPE,
            )
            .to_string(),
            twitter_card: get_string_ref_or_default(
                &self
                    .app
                    .social_meta
                    .as_ref()
                    .and_then(|s| s.twitter_card.as_ref()),
                DEFAULT_TWITTER_CARD,
            )
            .to_string(),
            twitter_label1: get_string_ref_or_default(
                &self
                    .app
                    .social_meta
                    .as_ref()
                    .and_then(|s| s.twitter_label1.as_ref()),
                "Written by",
            )
            .to_string(),
            google_adsense_client: get_string_ref_or_default(
                &self
                    .app
                    .seo
                    .as_ref()
                    .and_then(|s| s.google_adsense_client.as_ref()),
                "",
            )
            .to_string(),
        }
    }

    pub fn process_post_page(
        &self,
        article: &Article,
        post: &Post,
        language_code: &str,
        translations: &HashMap<String, String>,
    ) -> eyre::Result<String> {
        let post_yml = GenericYmlInfo {
            page_name: post.filename.clone(),
            title: post.title.clone(),
            description: post.description.clone(),
            keywords: vec![post.keywords.clone()],
            image: Some(post.image.clone()),
            draft: Some(post.draft),
            date_published: Some(post.published.clone()),
            date_modified: Some(post.updated.clone()),
            lang: Some(language_code.to_string()),
            layout: Some(post.layout.clone()),
            category: post.category.clone(),
            link_text: Some(post.title.clone()),
            author: Some(post.author.clone()),
            author_url: Some(post.author_url.clone()),
            author_email: Some(post.author_email.clone()),
        };

        let mut data = self.create_page_data(article, &post_yml, language_code, translations);

        data.post = Some(post.clone());
        data.type_page = "BlogPosting".to_string();

        let post_path = if language_code == self.app.languages.default_language {
            post.url.clone()
        } else {
            format!(
                "/{language_code}{post_url}",
                language_code = language_code,
                post_url = &post.url
            )
        };

        data.path.clone_from(&post_path);
        data.url = format!(
            "{app_domain}{post_path}",
            app_domain = self.app.app_info.app_domain,
            post_path = &post_path
        );

        let content = self.handlebars.render(&post.layout, &data)?;
        Ok(content)
    }
}
