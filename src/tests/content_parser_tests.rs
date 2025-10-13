use crate::app::AppConfig;
use crate::blog::Article;
use crate::generator::content_parser::ContentParser;
use std::path::PathBuf;

fn make_minimal_app() -> AppConfig {
    let mut cfg = AppConfig::default();
    cfg.languages.installed_languages = vec!["en".to_string(), "th".to_string()];
    cfg.languages.default_language = "en".to_string();
    cfg
}

fn make_article(prefix: &str, lang: &str) -> Article {
    let prefix_pb = PathBuf::from(prefix);
    Article {
        layout: "articles".to_string(),
        title: "t".to_string(),
        description: "d".to_string(),
        image: "".to_string(),
        prefix: prefix_pb,
        app_domain: "http://example.com".to_string(),
        posts: Vec::new(),
        link_text: "t".to_string(),
        keywords: Vec::new(),
        author: "".to_string(),
        author_url: "".to_string(),
        author_email: "".to_string(),
        lang: lang.to_string(),
        draft: false,
        date_modified: "".to_string(),
        date_published: "".to_string(),
        category: "".to_string(),
    }
}

#[test]
fn parse_articles_includes_root_and_subdirs() {
    let app = make_minimal_app();
    let parser = ContentParser::new(&app);

    let a1 = make_article("", ""); 
    let a2 = make_article("web-optimization", "en");

    let articles = vec![a1, a2];
    let listing = parser.parse_articles_for_listing(&articles, "en");

    assert!(listing.iter().any(|v| v.get("url").and_then(|u| u.as_str()) == Some("/")));
    assert!(listing.iter().any(|v| v.get("url").and_then(|u| u.as_str()) == Some("/web-optimization")));
}
