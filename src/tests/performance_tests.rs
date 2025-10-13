use std::fs;
use std::path::Path;
use std::time::{Duration, Instant};
use tempfile::TempDir;

const PERFORMANCE_THRESHOLD_MS: u128 = 5000; // 5 seconds max for builds
const MEMORY_THRESHOLD_MB: u64 = 500; // 500MB max memory usage

#[test]
#[ignore = "Long-running performance test — ignored by default. Run with `cargo test -- --ignored`."]
fn test_build_performance() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    // Create test content with multiple files
    create_large_test_content(temp_path);

    let start = Instant::now();

    // Run build command
    let output = std::process::Command::new("cargo")
        .args(["run", "--release", "build"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute build command");

    let duration = start.elapsed();

    assert!(output.status.success(), "Build command failed");
    assert!(
        duration.as_millis() < PERFORMANCE_THRESHOLD_MS,
        "Build took too long: {}ms (threshold: {}ms)",
        duration.as_millis(),
        PERFORMANCE_THRESHOLD_MS
    );

    println!("✅ Build performance: {}ms", duration.as_millis());
}

#[test]
#[ignore = "Long-running performance test — ignored by default. Run with `cargo test -- --ignored`."]
fn test_seo_analysis_performance() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    create_large_test_content(temp_path);

    // First build the site
    let _build_output = std::process::Command::new("cargo")
        .args(["run", "--release", "build"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute build command");

    let start = Instant::now();

    // Run SEO analysis
    let output = std::process::Command::new("cargo")
        .args(["run", "--release", "seo"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute seo command");

    let duration = start.elapsed();

    assert!(output.status.success(), "SEO command failed");
    assert!(
        duration.as_millis() < 3000, // 3 seconds for SEO analysis
        "SEO analysis took too long: {}ms",
        duration.as_millis()
    );

    println!("✅ SEO analysis performance: {}ms", duration.as_millis());
}

#[test]
#[ignore = "Memory/IO intensive test — ignored by default. Run with `cargo test -- --ignored`."]
fn test_memory_usage_during_build() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    create_large_test_content(temp_path);

    // Monitor memory usage during build
    let start = Instant::now();

    let child = std::process::Command::new("cargo")
        .args(["run", "--release", "build"])
        .current_dir(temp_path)
        .spawn()
        .expect("Failed to spawn build command");

    let pid = child.id();

    // Monitor memory in separate thread
    let memory_handle = std::thread::spawn(move || {
        let mut peak_memory = 0u64;
        while get_process_memory(pid).is_ok() {
            if let Ok(memory) = get_process_memory(pid) {
                peak_memory = peak_memory.max(memory);
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        peak_memory
    });

    let output = child.wait_with_output().expect("Failed to wait for build");
    let max_memory = memory_handle.join().unwrap_or(0);

    let duration = start.elapsed();

    assert!(output.status.success(), "Build command failed");

    if max_memory > 0 {
        assert!(
            max_memory < MEMORY_THRESHOLD_MB * 1024 * 1024,
            "Memory usage too high: {}MB (threshold: {}MB)",
            max_memory / 1024 / 1024,
            MEMORY_THRESHOLD_MB
        );

        println!(
            "✅ Memory usage: {}MB, Duration: {}ms",
            max_memory / 1024 / 1024,
            duration.as_millis()
        );
    } else {
        println!("⚠️  Could not measure memory usage");
    }
}

#[test]
#[ignore = "Concurrent builds performance test — ignored by default. Run with `cargo test -- --ignored`."]
fn test_concurrent_builds() {
    let num_concurrent = 3;
    let mut handles = Vec::new();

    for i in 0..num_concurrent {
        let handle = std::thread::spawn(move || {
            let temp_dir = TempDir::new().expect("Failed to create temp directory");
            let temp_path = temp_dir.path();

            create_test_content(temp_path, i);

            let start = Instant::now();

            let output = std::process::Command::new("cargo")
                .args(["run", "--release", "build"])
                .current_dir(temp_path)
                .output()
                .expect("Failed to execute build command");

            let duration = start.elapsed();

            assert!(output.status.success(), "Concurrent build {i} failed");
            duration
        });

        handles.push(handle);
    }

    let mut total_duration = Duration::from_millis(0);
    for (i, handle) in handles.into_iter().enumerate() {
        let duration = handle.join().expect("Thread panicked");
        total_duration += duration;
        println!("✅ Concurrent build {}: {}ms", i, duration.as_millis());
    }

    let avg_duration = total_duration.as_millis() / num_concurrent as u128;
    assert!(
        avg_duration < PERFORMANCE_THRESHOLD_MS,
        "Average concurrent build time too slow: {avg_duration}ms"
    );

    println!("✅ Average concurrent build time: {avg_duration}ms");
}

#[test]
#[ignore = "Large-file processing test — ignored by default. Run with `cargo test -- --ignored`."]
fn test_large_file_processing() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    // Create a very large markdown file
    create_large_markdown_file(temp_path);
    create_minimal_config(temp_path);

    let start = Instant::now();

    let output = std::process::Command::new("cargo")
        .args(["run", "--release", "build"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute build command");

    let duration = start.elapsed();

    assert!(output.status.success(), "Large file build failed");
    assert!(
        duration.as_millis() < PERFORMANCE_THRESHOLD_MS * 2, // Allow more time for large files
        "Large file processing took too long: {}ms",
        duration.as_millis()
    );

    println!("✅ Large file processing: {}ms", duration.as_millis());
}

// Helper functions

fn create_large_test_content(temp_path: &Path) {
    create_minimal_config(temp_path);

    // Create multiple markdown files
    for i in 0..20 {
        let content = format!(
            r#"---
title: "Performance Test Article {}"
description: "Article {} for performance testing"
---

# Performance Test Article {}

This is article number {} created for performance testing.

{}

## Section 1
Lorem ipsum dolor sit amet, consectetur adipiscing elit.

## Section 2
Sed do eiusmod tempor incididunt ut labore et dolore magna aliqua.

## Section 3
Ut enim ad minim veniam, quis nostrud exercitation ullamco.
"#,
            i,
            i,
            i,
            i,
            "Content line\n".repeat(100)
        );

        fs::write(
            temp_path.join(format!(
                "contents/2025-01-{:02}-performance-test-{}.md",
                i + 1,
                i
            )),
            content,
        )
        .expect("Failed to write test markdown");
    }

    // Create page YML files
    for page in &["index", "about", "contact", "articles"] {
        let yml_content = format!(
            r#"title: "{page} Page"
description: "Test {page} page for performance testing"
image: ""
"#
        );
        fs::write(temp_path.join(format!("contents/{page}.yml")), yml_content)
            .expect("Failed to write YML file");
    }
}

fn create_test_content(temp_path: &Path, index: i32) {
    create_minimal_config(temp_path);

    let content = format!(
        r#"---
title: "Test Article {index}"
description: "Article {index} for concurrent testing"
image: ""
date: "2025-01-01"
author: "Test Author"
---

# Test Article {index}

This is test article {index} for concurrent testing.
"#
    );

    fs::write(
        temp_path.join(format!("contents/2025-01-01-test-article-{index}.md")),
        content,
    )
    .expect("Failed to write test markdown");

    let index_yml = format!(
        r#"title: "Test Blog {index}"
description: "Test blog {index} for concurrent testing"
image: ""
"#
    );
    fs::write(temp_path.join("contents/index.yml"), index_yml).expect("Failed to write index.yml");
}

fn create_large_markdown_file(temp_path: &Path) {
    fs::create_dir_all(temp_path.join("contents")).expect("Failed to create contents directory");

    let mut content = String::with_capacity(1024 * 1024); // 1MB initial capacity
    content.push_str(
        r#"---
title: "Large Performance Test Article"
description: "A very large article for performance testing"
---

# Large Performance Test Article

This is a very large article created for performance testing.

"#,
    );

    // Add 10,000 lines of content
    for i in 0..10000 {
        content.push_str(&format!(
            "Line {i} of large content for performance testing.\n"
        ));
        if i % 100 == 0 {
            content.push_str(&format!("\n## Section {}\n\n", i / 100));
        }
    }

    fs::write(
        temp_path.join("contents/2025-01-01-large-performance-test.md"),
        content,
    )
    .expect("Failed to write large markdown file");
}

fn create_minimal_config(temp_path: &Path) {
    // Create necessary directories
    fs::create_dir_all(temp_path.join("contents")).expect("Failed to create contents directory");
    fs::create_dir_all(temp_path.join("source/templates"))
        .expect("Failed to create templates directory");
    fs::create_dir_all(temp_path.join("public")).expect("Failed to create public directory");

    // Copy essential files from the manifest directory so tests run reliably
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

    // Create minimal templates
    let template = r"<!DOCTYPE html>
<html>
<head><title>{{title}}</title></head>
<body><h1>{{title}}</h1><p>{{description}}</p></body>
</html>";

    for template_name in &["index", "about", "contact", "articles"] {
        fs::write(
            temp_path.join(format!("source/templates/{template_name}.html")),
            template,
        )
        .expect("Failed to write template");
    }

    // Create index.yml if it doesn't exist
    if !temp_path.join("contents/index.yml").exists() {
        let index_yml = r#"title: "Performance Test Blog"
description: "A blog for performance testing"
image: ""
"#;
        fs::write(temp_path.join("contents/index.yml"), index_yml)
            .expect("Failed to write index.yml");
    }
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

fn get_process_memory(pid: u32) -> Result<u64, std::io::Error> {
    // Cross-platform memory measurement
    #[cfg(target_os = "linux")]
    {
        let status = fs::read_to_string(format!("/proc/{}/status", pid))?;
        for line in status.lines() {
            if line.starts_with("VmRSS:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    return parts[1].parse::<u64>().map(|kb| kb * 1024).map_err(|_| {
                        std::io::Error::new(std::io::ErrorKind::InvalidData, "Parse error")
                    });
                }
            }
        }
        Err(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "Memory info not found",
        ))
    }

    #[cfg(target_os = "macos")]
    {
        use std::process::Command;
        let output = Command::new("ps")
            .args(["-o", "rss=", "-p", &pid.to_string()])
            .output()?;

        let rss_str = String::from_utf8_lossy(&output.stdout);
        rss_str
            .trim()
            .parse::<u64>()
            .map(|kb| kb * 1024)
            .map_err(|_| std::io::Error::new(std::io::ErrorKind::InvalidData, "Parse error"))
    }

    #[cfg(target_os = "windows")]
    {
        // For Windows, we'll skip memory measurement in tests
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Memory measurement not supported on Windows",
        ))
    }

    #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
    {
        Err(std::io::Error::new(
            std::io::ErrorKind::Unsupported,
            "Unsupported platform",
        ))
    }
}
