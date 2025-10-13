use crate::shared::validation::*;
use regex::Regex;
use std::path::Path;

#[test]
fn test_valid_markdown_filenames() {
    let config = ValidationConfig::default();
    let validator = FileValidator::new(config);

    // Valid date patterns
    assert!(validator.is_valid_date_format("2025-04-26"));
    assert!(validator.is_valid_date_format("2023-12-31"));
    assert!(validator.is_valid_date_format("2024-01-01"));
    assert!(validator.is_valid_date_format("2025-01-15"));

    // Invalid date patterns
    assert!(!validator.is_valid_date_format("2025-13-26")); // Invalid month
    assert!(!validator.is_valid_date_format("2025-04-32")); // Invalid day
    assert!(!validator.is_valid_date_format("2019-04-26")); // Too old year
    assert!(!validator.is_valid_date_format("2031-04-26")); // Too future year
    assert!(!validator.is_valid_date_format("25-04-26")); // Wrong year format
    assert!(!validator.is_valid_date_format("2025-4-26")); // Wrong month format
    assert!(!validator.is_valid_date_format("2025-04-6")); // Wrong day format
}

#[test]
fn test_markdown_file_detection() {
    let config = ValidationConfig::default();
    let validator = FileValidator::new(config);

    // Valid markdown files
    assert!(validator.is_markdown_file(Path::new("test.md")));
    assert!(validator.is_markdown_file(Path::new("article.md")));
    assert!(validator.is_markdown_file(Path::new("2025-01-01-post.md")));

    // Invalid files (not markdown)
    assert!(!validator.is_markdown_file(Path::new("test.txt")));
    assert!(!validator.is_markdown_file(Path::new("image.jpg")));
    assert!(!validator.is_markdown_file(Path::new("style.css")));
    assert!(!validator.is_markdown_file(Path::new("script.js")));
    assert!(!validator.is_markdown_file(Path::new("config.toml")));
}

#[test]
fn test_filename_format_validation() {
    // Test the regex pattern directly
    let date_kebab_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}-[a-z0-9]+(-[a-z0-9]+)*$").unwrap();

    // Valid patterns - should pass
    assert!(date_kebab_regex.is_match("2025-04-26-what-is-seo-beginners-guide"));
    assert!(date_kebab_regex.is_match("2024-12-01-simple-post"));
    assert!(date_kebab_regex.is_match("2023-01-15-test123"));
    assert!(date_kebab_regex.is_match("2025-06-15-hello-world"));
    assert!(date_kebab_regex.is_match("2024-03-20-markdown-guide"));
    assert!(date_kebab_regex.is_match("2025-11-05-seo-tips"));

    // Invalid patterns - should fail
    assert!(!date_kebab_regex.is_match("2025-04-26-What-Is-SEO")); // Uppercase letters
    assert!(!date_kebab_regex.is_match("2025-04-26-post_title")); // Underscore
    assert!(!date_kebab_regex.is_match("2025-04-26")); // No slug after date
    assert!(!date_kebab_regex.is_match("what-is-seo-guide")); // No date prefix
    assert!(!date_kebab_regex.is_match("25-04-26-post")); // Wrong year format
    assert!(!date_kebab_regex.is_match("2025-4-26-post")); // Wrong month format
    assert!(!date_kebab_regex.is_match("2025-04-6-post")); // Wrong day format
    assert!(!date_kebab_regex.is_match("2025-04-26-Post Title")); // Spaces
    assert!(!date_kebab_regex.is_match("2025-04-26-")); // Empty slug
    assert!(!date_kebab_regex.is_match("2025-04-26--double-dash")); // Double dash (handled separately)
}

#[test]
fn test_forbidden_characters() {
    let config = ValidationConfig::default();
    let forbidden_chars = &config.forbidden_chars;

    // Test that forbidden characters are properly defined
    assert!(forbidden_chars.contains(&' ')); // Space
    assert!(forbidden_chars.contains(&'<')); // Less than
    assert!(forbidden_chars.contains(&'>')); // Greater than
    assert!(forbidden_chars.contains(&':')); // Colon
    assert!(forbidden_chars.contains(&'"')); // Quote
    assert!(forbidden_chars.contains(&'|')); // Pipe
    assert!(forbidden_chars.contains(&'?')); // Question mark
    assert!(forbidden_chars.contains(&'*')); // Asterisk
}

#[test]
fn test_slug_validation_scenarios() {
    // Test various slug scenarios that might occur
    let date_kebab_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}-[a-z0-9]+(-[a-z0-9]+)*$").unwrap();

    // Good slugs
    assert!(date_kebab_regex.is_match("2025-01-01-a")); // Single letter
    assert!(date_kebab_regex.is_match("2025-01-01-123")); // Numbers only
    assert!(date_kebab_regex.is_match("2025-01-01-a1b2c3")); // Mixed
    assert!(date_kebab_regex.is_match("2025-01-01-very-long-article-title-with-many-words"));

    // Bad slugs
    assert!(!date_kebab_regex.is_match("2025-01-01-")); // Empty after dash
    assert!(!date_kebab_regex.is_match("2025-01-01-A")); // Uppercase
    assert!(!date_kebab_regex.is_match("2025-01-01-a_b")); // Underscore
    assert!(!date_kebab_regex.is_match("2025-01-01-a.b")); // Dot
}

#[test]
fn test_validation_result() {
    let mut result = ValidationResult::new();

    // Initially should be valid
    assert!(result.is_valid);
    assert!(!result.has_errors());
    assert!(!result.has_warnings());

    // Add warning - should still be valid
    result.add_warning("Test warning".to_string());
    assert!(result.is_valid);
    assert!(!result.has_errors());
    assert!(result.has_warnings());

    // Add error - should become invalid
    result.add_error("Test error".to_string());
    assert!(!result.is_valid);
    assert!(result.has_errors());
    assert!(result.has_warnings());
}

#[test]
fn test_validation_summary() {
    let mut summary = ValidationSummary::new();

    // Initially empty
    assert_eq!(summary.total_files, 0);
    assert_eq!(summary.valid_files, 0);
    assert_eq!(summary.files_with_errors, 0);
    assert_eq!(summary.files_with_warnings, 0);

    // Add valid file
    let valid_result = ValidationResult::success();
    summary.add_file_result(Path::new("valid.md").to_path_buf(), valid_result);

    assert_eq!(summary.total_files, 1);
    assert_eq!(summary.valid_files, 1);
    assert_eq!(summary.files_with_errors, 0);
    assert_eq!(summary.files_with_warnings, 0);

    // Add file with warning
    let mut warning_result = ValidationResult::new();
    warning_result.add_warning("Test warning".to_string());
    summary.add_file_result(Path::new("warning.md").to_path_buf(), warning_result);

    assert_eq!(summary.total_files, 2);
    assert_eq!(summary.valid_files, 1);
    assert_eq!(summary.files_with_errors, 0);
    assert_eq!(summary.files_with_warnings, 1);

    // Add file with error
    let mut error_result = ValidationResult::new();
    error_result.add_error("Test error".to_string());
    summary.add_file_result(Path::new("error.md").to_path_buf(), error_result);

    assert_eq!(summary.total_files, 3);
    assert_eq!(summary.valid_files, 1);
    assert_eq!(summary.files_with_errors, 1);
    assert_eq!(summary.files_with_warnings, 1);
    assert!(summary.has_errors());
}
