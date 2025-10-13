use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

#[test]
#[ignore = "Long-running integration test — ignored by default. Run explicitly with `cargo test -- --ignored`."]
fn test_build_command_generates_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    copy_test_content(temp_path);

    let output = Command::new("cargo")
        .args(["run", "build"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute build command");

    assert!(
        output.status.success(),
        "Build command failed: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    assert!(
        temp_path.join("build").exists(),
        "Build directory was not created"
    );

    assert!(
        temp_path.join("build/index.html").exists(),
        "index.html was not generated"
    );

    assert!(
        temp_path.join("build/_system_/styles/app.css").exists(),
        "CSS was not generated"
    );
}

#[test]
#[ignore = "Long-running integration test — ignored by default. Run explicitly with `cargo test -- --ignored`."]
fn test_seo_command_analyzes_content() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    copy_test_content(temp_path);

    let build_output = Command::new("cargo")
        .args(["run", "build"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute build command");

    assert!(build_output.status.success());

    let seo_output = Command::new("cargo")
        .args(["run", "seo"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute seo command");

    assert!(
        seo_output.status.success(),
        "SEO command failed: {}",
        String::from_utf8_lossy(&seo_output.stderr)
    );

    let output_str = String::from_utf8_lossy(&seo_output.stdout);
    assert!(
        output_str.contains("SEO Scan Summary"),
        "SEO analysis output missing"
    );
}

#[test]
#[ignore = "Integration test that exercises CLI language commands — ignored by default. Run with `cargo test -- --ignored`."]
fn test_lang_commands() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    copy_test_content(temp_path);

    let list_output = Command::new("cargo")
        .args(["run", "lang", "list"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute lang list command");

    assert!(list_output.status.success());
    let output_str = String::from_utf8_lossy(&list_output.stdout);
    assert!(
        output_str.contains("Available Languages"),
        "Language list output missing"
    );

    let info_output = Command::new("cargo")
        .args(["run", "lang", "info", "th"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute lang info command");

    assert!(info_output.status.success());
    let info_str = String::from_utf8_lossy(&info_output.stdout);
    assert!(
        info_str.contains("Language Information"),
        "Language info output missing"
    );
}

#[test]
#[ignore = "Integration test that creates files on disk — ignored by default. Run with `cargo test -- --ignored`."]
fn test_new_post_creation() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    copy_test_content(temp_path);

    let new_post_output = Command::new("cargo")
        .args(["run", "new", "post", "Test Integration Post"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute new post command");

    assert!(
        new_post_output.status.success(),
        "New post command failed: {}",
        String::from_utf8_lossy(&new_post_output.stderr)
    );

    let contents_dir = temp_path.join("contents");
    let post_files: Vec<_> = fs::read_dir(&contents_dir)
        .expect("Failed to read contents directory")
        .filter_map(|entry| entry.ok())
        .filter(|entry| {
            entry
                .file_name()
                .to_string_lossy()
                .contains("test-integration-post")
        })
        .collect();

    assert!(!post_files.is_empty(), "New post file was not created");
}

#[test]
#[ignore = "Network/IO heavy integration test (deploy flow) — ignored by default. Run with `cargo test -- --ignored`."]
fn test_deploy_mock_mode() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    copy_test_content(temp_path);

    let deploy_output = Command::new("cargo")
        .args(["run", "deploy", "test"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute deploy test command");

    assert!(
        deploy_output.status.success(),
        "Deploy test command failed: {}",
        String::from_utf8_lossy(&deploy_output.stderr)
    );

    let output_str = String::from_utf8_lossy(&deploy_output.stdout);
    assert!(
        output_str.contains("Mock deployment completed successfully"),
        "Deploy mock output missing"
    );
}

#[test]
#[ignore = "Integration test exercising validation CLI flow — ignored by default. Run with `cargo test -- --ignored`."]
fn test_validation_with_invalid_files() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    copy_test_content(temp_path);

    let invalid_file_path = temp_path.join("contents/invalid-filename.md");
    fs::write(
        &invalid_file_path,
        "# Invalid File\nThis file has an invalid name.",
    )
    .expect("Failed to write invalid file");

    let build_output = Command::new("cargo")
        .args(["run", "build"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute build command");

    assert!(build_output.status.success());

    let output_str = String::from_utf8_lossy(&build_output.stdout);
    assert!(
        output_str.contains("Validation"),
        "Validation output missing"
    );
}

#[test]
#[ignore = "Integration test for multi-language build outputs — ignored by default. Run with `cargo test -- --ignored`."]
fn test_build_multi_language() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    copy_test_content(temp_path);

    let build_output = Command::new("cargo")
        .args(["run", "build-lang", "en"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute build-lang command");

    assert!(
        build_output.status.success(),
        "Build-lang command failed: {}",
        String::from_utf8_lossy(&build_output.stderr)
    );

    assert!(
        temp_path.join("build/en").exists(),
        "English build directory not created"
    );
    assert!(
        temp_path.join("build/en/index.html").exists(),
        "English index.html not generated"
    );
}

#[test]
#[ignore = "CLI/help integration test — ignored by default to speed up CI. Run with `cargo test -- --ignored`."]
fn test_help_command() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    copy_test_content(temp_path);

    let help_output = Command::new("cargo")
        .args(["run", "help"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute help command");

    assert!(help_output.status.success());

    let output_str = String::from_utf8_lossy(&help_output.stdout);
    assert!(
        output_str.contains("GenWebBlog Command Reference"),
        "Help output missing"
    );
    assert!(output_str.contains("QUICK START"), "Help sections missing");
    assert!(output_str.contains("DEPLOYMENT"), "Help sections missing");
}


fn copy_test_content(temp_path: &Path) {

    fs::create_dir_all(temp_path.join("contents")).expect("Failed to create contents directory");
    fs::create_dir_all(temp_path.join("source/templates"))
        .expect("Failed to create templates directory");
    fs::create_dir_all(temp_path.join("public")).expect("Failed to create public directory");

    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| ".".to_string());

    let cargo_src = Path::new(&manifest_dir).join("Cargo.toml");
    if cargo_src.exists() {
        fs::copy(&cargo_src, temp_path.join("Cargo.toml")).expect("Failed to copy Cargo.toml");
    }

    let app_src = Path::new(&manifest_dir).join("app.toml");
    if app_src.exists() {
        fs::copy(&app_src, temp_path.join("app.toml")).expect("Failed to copy app.toml");
    }

    let src_src = Path::new(&manifest_dir).join("src");
    if src_src.exists() {
        copy_dir_all(&src_src, temp_path.join("src")).expect("Failed to copy src directory");
    }

    create_minimal_test_content(temp_path);
}

fn copy_dir_all(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir_all(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

fn create_minimal_test_content(temp_path: &Path) {

    let index_yml = r#"title: "Test Blog"
description: "A test blog for integration testing"
image: ""
"#;
    fs::write(temp_path.join("contents/index.yml"), index_yml).expect("Failed to write index.yml");

    let about_yml = r#"title: "About Us"
description: "About our test blog"
image: ""
"#;
    fs::write(temp_path.join("contents/about.yml"), about_yml).expect("Failed to write about.yml");

    let test_md = r#"---
title: "Test Article"
description: "A test article for integration testing"
---

# Test Article

This is a test article for integration testing.
"#;
    fs::write(
        temp_path.join("contents/2025-01-01-test-article.md"),
        test_md,
    )
    .expect("Failed to write test markdown");

    let index_template = r#"<!DOCTYPE html>
<html>
<head>
    <title>{{title}}</title>
    <meta name="description" content="{{description}}">
</head>
<body>
    <h1>{{title}}</h1>
    <p>{{description}}</p>
</body>
</html>"#;

    fs::write(
        temp_path.join("source/templates/index.html"),
        index_template,
    )
    .expect("Failed to write index template");
    fs::write(
        temp_path.join("source/templates/about.html"),
        index_template,
    )
    .expect("Failed to write about template");
}
