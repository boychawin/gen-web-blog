use genwebblog::constants::app;
use genwebblog::error::{GenWebBlogError, Result};
use genwebblog::shared;
use genwebblog::shared::utils::is_online;
use log::{error, info, warn};
use reqwest::Client;
use std::env;
use std::fs;
use std::net::TcpListener;
use std::process;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args: Vec<String> = env::args().collect();

    let requires_network = args
        .get(1)
        .is_none_or(|cmd| matches!(cmd.as_str(), "deploy" | "seo" | "update"));

    if requires_network && !is_online().await {
        error!("🛜  No internet connection required for this operation");
        return Err(GenWebBlogError::network(
            "Internet connection required but not available",
        ));
    }

    let command = if args.len() > 1 {
        args[1].as_str()
    } else {
        "start"
    };

    if !matches!(command, "init" | "help" | "--help" | "-h") {
        let config = genwebblog::app::read_config();
        if !config.app_info.app_token.is_empty() {
            if let Err(e) = shared::verify_token::verify_token(&config.app_info.app_token).await {
                warn!("Token verification failed: {e}");
            }
        }
    }

    let result = match command {
        "init" => {
            let is_full = args.get(2).is_some_and(|s| s == "full");
            info!("🚀 Initializing project with full setup: {is_full}");
            generate_project_files(is_full)
        }
        "deploy" => {
            let is_mock = args.get(2).is_some_and(|s| s == "test" || s == "mock");
            info!("🚀 Starting deployment (mock: {is_mock})");
            run_deploy(is_mock).await
        }
        "seo" => {
            if args.get(2).is_some_and(|s| s == "test") {
                run_seo_test()
            } else {
                run_seo()
            }
        }
        "resize" => run_resize(),
        "logo" => run_resize_logo(),
        "build" => run_build(),
        "build-lang" => {
            if let Some(lang_code) = args.get(2) {
                run_build_language(lang_code)
            } else {
                run_build()
            }
        }
        "update" => check_for_update().await,
        "start" => start_server().await,
        "page" => {
            if let Some(page_name) = args.get(2) {
                create_page(page_name)
            } else {
                return Err(GenWebBlogError::config(
                    "Please specify a page name, e.g., ./genwebblog page home",
                ));
            }
        }
        "quick-start" | "qs" => {
            info!("🚀 GenWebBlog Quick Start - Creating demo content");
            run_quick_start()
        }
        "lang" | "language" => handle_language_cmd(&args),
        "new" => {
            if let Some(post_title) = args.get(2) {
                create_new_post(post_title)
            } else {
                return Err(GenWebBlogError::config(
                    "Please specify a post title, e.g., ./genwebblog new 'My New Post'",
                ));
            }
        }
        "help" | "--help" | "-h" => {
            show_help();
            Ok(())
        }
        "version" | "--version" | "-v" => {
            println!("{} v{}", app::NAME, app::VERSION);
            println!("A modern static site generator for blogs");
            Ok(())
        }
        _ => {
            error!("Unknown command: {command}");
            show_help();
            Err(GenWebBlogError::config(format!(
                "Unknown command: {command}"
            )))
        }
    };

    // Handle the result
    if let Err(e) = result {
        error!("{}", e.user_message());
        process::exit(1);
    }

    Ok(())
}

fn handle_language_cmd(args: &[String]) -> Result<()> {
    match args.get(2).map(|s| s.as_str()) {
        Some("list" | "ls") => genwebblog::language::list_languages()
            .map_err(|e| GenWebBlogError::language(e.to_string())),
        Some("install") => {
            if let Some(code) = args.get(3) {
                genwebblog::language::install_language(code)
                    .map_err(|e| GenWebBlogError::language(e.to_string()))
            } else {
                Err(GenWebBlogError::config(
                    "Please specify language code to install",
                ))
            }
        }
        Some("uninstall" | "remove") => {
            if let Some(code) = args.get(3) {
                genwebblog::language::uninstall_language(code)
                    .map_err(|e| GenWebBlogError::language(e.to_string()))
            } else {
                Err(GenWebBlogError::config(
                    "Please specify language code to uninstall",
                ))
            }
        }
        Some("set-default" | "default") => {
            if let Some(code) = args.get(3) {
                genwebblog::language::set_default_language(code)
                    .map_err(|e| GenWebBlogError::language(e.to_string()))
            } else {
                Err(GenWebBlogError::config(
                    "Please specify language code to set as default",
                ))
            }
        }
        Some("info") => {
            if let Some(code) = args.get(3) {
                genwebblog::language::show_language_info(code)
                    .map_err(|e| GenWebBlogError::language(e.to_string()))
            } else {
                Err(GenWebBlogError::config(
                    "Please specify language code for info",
                ))
            }
        }
        Some("help") | None => {
            show_language_help();
            Ok(())
        }
        Some(cmd) => Err(GenWebBlogError::config(format!(
            "Unknown language command: {cmd}"
        ))),
    }
}

fn run_quick_start() -> Result<()> {
    println!("🚀 GenWebBlog Quick Start");
    println!("Creating demo content...");
    run_build()?;
    println!("✅ Quick start complete! Run './genwebblog start' to view your site.");
    Ok(())
}

fn run_build() -> Result<()> {
    info!("🔨 Building static site...");
    if let Err(e) = genwebblog::main() {
        return Err(GenWebBlogError::config(format!("Build failed: {e}")));
    }
    info!("✅ Build completed successfully");
    Ok(())
}

fn run_build_language(lang_code: &str) -> Result<()> {
    info!("🔨 Building site for language: {lang_code}");
    run_build()
}

async fn start_server() -> Result<()> {
    info!("🌐 Starting development server...");

    run_build()?;

    let config = genwebblog::app::read_config();
    let configured_port = config.app_info.app_port;

    let port = if TcpListener::bind(format!("127.0.0.1:{configured_port}")).is_ok() {
        configured_port
    } else {
        info!("Port {configured_port} is busy, searching for available port...");
        (configured_port..configured_port + 100)
            .find(|&port| TcpListener::bind(format!("127.0.0.1:{port}")).is_ok())
            .unwrap_or_else(|| {
                (3000..4000)
                    .find(|&port| TcpListener::bind(format!("127.0.0.1:{port}")).is_ok())
                    .unwrap_or(3000)
            })
    };

    println!("│  🚀 Server running at http://127.0.0.1:{port}");
    println!("│  📁 Serving files from: build/");
    println!("│  🔄 Press Ctrl+C to stop");

    let routes = warp::fs::dir("build");
    warp::serve(routes).run(([127, 0, 0, 1], port)).await;

    Ok(())
}

async fn run_deploy(is_mock: bool) -> Result<()> {
    info!("🚀 Starting deployment (mock: {is_mock})");

    if is_mock {
        println!("│  🧪 Mock deployment - no actual deployment performed");
        return Ok(());
    }

    run_build()?;

    let config = genwebblog::app::read_config();
    let client = Client::new();

    let deploy_config = genwebblog::deploy::DeployConfig {
        user: &config.deploy_github.user,
        repo_name: &config.deploy_github.repo_name,
        branch: &config.deploy_github.branch,
        build_dir: "build",
        github_token: &config.deploy_github.token,
        client: &client,
        cloudflare_api_token: &config.deploy_cloudflare.api_token,
        cloudflare_account_id: &config.deploy_cloudflare.account_id,
        project_name: &config.deploy_cloudflare.project_name,
    };

    genwebblog::deploy::check_repo_exists(
        &client,
        &config.deploy_github.token,
        &config.deploy_github.user,
        &config.deploy_github.repo_name,
        &config.deploy_github.private,
    )
    .await?;

    genwebblog::deploy::push_to_github(&deploy_config).await?;

    info!("✅ Deployment completed successfully");
    Ok(())
}

fn run_seo() -> Result<()> {
    info!("🔍 Running SEO analysis...");

    run_build()?;

    println!("│  🚀 Starting SEO scanning...");
    shared::seo::scan_html_files_in_directory("build");
    Ok(())
}

fn run_seo_test() -> Result<()> {
    println!("│  🧪 Running SEO tests...");
    if shared::seo::run_seo_tests() {
        println!("│  ✅ All SEO tests passed!");
    } else {
        println!("│  ❌ Some SEO tests failed!");
    }
    Ok(())
}

async fn check_for_update() -> Result<()> {
    let config = genwebblog::app::read_config();
    let version = config.app_info.app_version;
    shared::update_version::check_for_update(&version).await?;
    Ok(())
}

pub fn run_resize() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Some(input_path) = args.get(2) {
        println!("🔹 Input path: {input_path}");
        println!("│  🔄 Resizing image... ");

        match shared::process_image::process_image(input_path) {
            Ok(_) => {
                println!("│  🎉 Resized image saved");
            }
            Err(e) => {
                return Err(GenWebBlogError::image_processing(
                    input_path,
                    format!("Failed to resize image: {e}"),
                ));
            }
        }
    } else {
        println!("│  Usage: ./genwebblog resize <image_path>");
    }

    Ok(())
}

pub fn run_resize_logo() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    if let Some(input_path) = args.get(2) {
        println!("🔹 Input path: {input_path}");
        println!("│  🔄 Resizing logo... ");

        match shared::process_image::process_image(input_path) {
            Ok(_) => {
                println!("│  🎉 Resized logo saved");
            }
            Err(e) => {
                return Err(GenWebBlogError::image_processing(
                    input_path,
                    format!("Failed to resize logo: {e}"),
                ));
            }
        }
    } else {
        println!("│  Usage: ./genwebblog logo <image_path>");
    }

    Ok(())
}

fn create_page(name: &str) -> Result<()> {
    let yml_path = format!("contents/{name}.yml");
    let html_path = format!("source/templates/{name}.html");

    let yml_content = r#"title: ""
description: ""
image: ""
"#
    .to_string();

    let html_content = format!(
        r"
<h1>{}</h1>
<p>{}</p>
",
        name.to_uppercase(),
        "This page's content will be added later..."
    );

    fs::create_dir_all("contents").map_err(|e| {
        GenWebBlogError::file_system("contents", format!("Failed to create directory: {e}"))
    })?;
    fs::create_dir_all("source/templates").map_err(|e| {
        GenWebBlogError::file_system(
            "source/templates",
            format!("Failed to create directory: {e}"),
        )
    })?;

    fs::write(&yml_path, yml_content).map_err(|e| {
        GenWebBlogError::file_system(&yml_path, format!("Failed to write YML file: {e}"))
    })?;
    fs::write(&html_path, html_content).map_err(|e| {
        GenWebBlogError::file_system(&html_path, format!("Failed to write HTML file: {e}"))
    })?;

    println!("✅ Successfully created the page: {name}");
    Ok(())
}

pub fn generate_project_files(is_full: bool) -> Result<()> {
    println!("│  📝 Generating project files...");

    shared::generate_files::create_files::create_files(is_full)?;

    println!("│  🎉 Project files generated successfully.");
    Ok(())
}

fn create_new_post(title: &str) -> Result<()> {
    let date = chrono::Utc::now().format("%Y-%m-%d").to_string();
    let slug = title
        .to_lowercase()
        .replace(' ', "-")
        .replace(['\'', '"', '?', '!'], "");
    let filename = format!("contents/{date}-{slug}.md");

    let content = format!(
        r#"---
title: "{title}"
description: "Enter your post description here"
author: "Your Name"
date: {date}
image: "/images/posts/{slug}.webp"
tags: ["blog", "new"]
lang: "en"
---

# {title}

Write your amazing content here using markdown!

## Introduction

Start with an engaging introduction that hooks your readers.

## Main Content

Add your main points, insights, or story here.

## Conclusion

Wrap up with a compelling conclusion.

---

*Happy blogging with GenWebBlog! 🚀*
"#
    );

    fs::write(&filename, content).map_err(|e| {
        GenWebBlogError::file_system(&filename, format!("Failed to write post file: {e}"))
    })?;
    println!("✅ Created new post: {filename}");
    println!("📝 Edit the file to add your content");

    Ok(())
}

fn show_help() {
    println!("GenWebBlog Command Reference");
    println!("============================");
    println!();
    println!("🚀 QUICK START:");
    println!("  quick-start, qs     Interactive setup wizard");
    println!("  demo                Generate demo content");
    println!();
    println!("📝 CONTENT MANAGEMENT:");
    println!("  new <title>         Create a new blog post");
    println!("  page <name>         Create a simple page");
    println!();
    println!("🌍 LANGUAGE MANAGEMENT:");
    println!("  lang list           List all available languages");
    println!("  lang install <code> Install a language pack");
    println!("  lang set-default <code> Set default language");
    println!("  lang info <code>    Show language information");
    println!("  lang help           Show language commands help");
    println!();
    println!("🔨 BUILD & DEPLOY:");
    println!("  build               Build the static site");
    println!("  build-lang <code>   Build for specific language");
    println!("  deploy              Deploy to production");
    println!("  deploy test         Test deployment (mock)");
    println!();
    println!("🖥️  DEVELOPMENT:");
    println!("  start               Start development server");
    println!("  dev                 Alias for start");
    println!();
    println!("🔍 SEO & OPTIMIZATION:");
    println!("  seo                 Run SEO analysis");
    println!("  seo test            Run SEO tests");
    println!();
    println!("🖼️  IMAGE PROCESSING:");
    println!("  resize <image>      Resize image for web");
    println!("  logo <image>        Process logo image");
    println!();
    println!("🔧 UTILITIES:");
    println!("  init                Initialize new project");
    println!("  init full           Initialize with full template");
    println!("  update              Check for updates");
    println!("  version             Show version information");
    println!("  help                Show this help message");
    println!();
    println!("📖 EXAMPLES:");
    println!("  ./genwebblog new \"My First Post\"");
    println!("  ./genwebblog lang install en");
    println!("  ./genwebblog build");
    println!("  ./genwebblog deploy");
    println!();
    println!("🌐 More info: https://github.com/boychawin/gen-web-blog");
}

fn show_language_help() {
    println!("GenWebBlog Language Management");
    println!("==============================");
    println!();
    println!("Available commands:");
    println!("  list, ls            List all available languages");
    println!("  install <code>      Install a language pack");
    println!("  uninstall <code>    Remove a language pack");
    println!("  set-default <code>  Set the default language");
    println!("  info <code>         Show detailed language info");
    println!();
    println!("Examples:");
    println!("  ./genwebblog lang list");
    println!("  ./genwebblog lang install en");
    println!("  ./genwebblog lang set-default th");
    println!("  ./genwebblog lang info en");
    println!();
    println!("Supported languages:");
    println!("  th - Thai (ไทย)");
    println!("  en - English");
    println!("  ja - Japanese (日本語)");
    println!("  zh - Chinese (中文)");
    println!("  ko - Korean (한국어)");
    println!();
    println!("💡 Language files are stored in: source/translations/<code>/");
    println!("💡 You can customize translations by editing the TOML files");
}
