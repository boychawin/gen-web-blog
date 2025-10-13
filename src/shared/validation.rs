use eyre::{Result, WrapErr};
use regex::Regex;
use std::collections::HashSet;
use std::fs;
use std::path::{Path, PathBuf};
use log::{info, warn};

/// File validation configuration focused on markdown files
#[derive(Debug, Clone)]
pub struct ValidationConfig {
    pub enabled: bool,
    pub check_markdown_filename_format: bool,
    pub allowed_image_formats: HashSet<String>,
    pub forbidden_chars: Vec<char>,
}

impl Default for ValidationConfig {
    fn default() -> Self {
        let mut allowed_image_formats = HashSet::new();
        allowed_image_formats.insert("webp".to_string());
        allowed_image_formats.insert("avif".to_string());
        allowed_image_formats.insert("jpg".to_string());
        allowed_image_formats.insert("jpeg".to_string());
        allowed_image_formats.insert("png".to_string());
        allowed_image_formats.insert("svg".to_string());

        Self {
            enabled: true,
            check_markdown_filename_format: true,
            allowed_image_formats,
            forbidden_chars: vec!['<', '>', ':', '"', '|', '?', '*', ' '],
        }
    }
}

/// File validator focused on markdown filename format
pub struct FileValidator {
    config: ValidationConfig,
}

impl FileValidator {
    pub fn new(config: ValidationConfig) -> Self {
        Self { config }
    }

    /// Validate a single file - focused on markdown filename format
    pub fn validate_file(&self, file_path: &Path) -> Result<ValidationResult> {
        if !self.config.enabled {
            return Ok(ValidationResult::success());
        }

        let mut result = ValidationResult::new();

        // Check if file exists
        if !file_path.exists() {
            result.add_error(format!("File does not exist: {}", file_path.display()));
            return Ok(result);
        }

        // Validate markdown filename format
        if self.config.check_markdown_filename_format && self.is_markdown_file(file_path) {
            self.validate_markdown_filename_format(file_path, &mut result)?;
        }

        Ok(result)
    }

    /// Check if file is a markdown file
    pub fn is_markdown_file(&self, file_path: &Path) -> bool {
        file_path
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|s| s.to_lowercase() == "md")
            .unwrap_or(false)
    }

    /// Validate markdown filename format (YYYY-MM-DD-kebab-case)
    fn validate_markdown_filename_format(
        &self,
        file_path: &Path,
        result: &mut ValidationResult,
    ) -> Result<()> {
        let filename = file_path
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| eyre::eyre!("Invalid filename"))?;

        // Remove .md extension for pattern checking
        let name_without_ext = filename.strip_suffix(".md").unwrap_or(filename);

        // Check for date prefix pattern (YYYY-MM-DD-) followed by kebab-case
        let date_kebab_regex = Regex::new(r"^\d{4}-\d{2}-\d{2}-[a-z0-9]+(-[a-z0-9]+)*$")
            .wrap_err("Failed to compile markdown filename regex")?;

        if !date_kebab_regex.is_match(name_without_ext) {
            result.add_error(format!(
                "Invalid markdown filename format: '{filename}'. Expected format: 'YYYY-MM-DD-kebab-case.md' (e.g., '2025-04-26-what-is-seo-beginners-guide.md')"
            ));
        } else {
            // Additional validation: check date validity
            // Use safe slicing to avoid panics on unexpected short names
            let date_part = name_without_ext.get(..10).unwrap_or(""); // YYYY-MM-DD
            if !self.is_valid_date_format(date_part) {
                result.add_warning(format!(
                    "Date in filename may be invalid: '{date_part}' in file: {filename}"
                ));
            }

            // Check for forbidden characters in the slug part
            let slug_part = name_without_ext.get(11..).unwrap_or(""); // everything after YYYY-MM-DD-
            for &forbidden_char in &self.config.forbidden_chars {
                if slug_part.contains(forbidden_char) {
                    result.add_error(format!(
                        "Filename contains forbidden character '{forbidden_char}' in slug part: {filename}"
                    ));
                }
            }

            // Check slug length (not too long)
            if slug_part.len() > 80 {
                result.add_warning(format!(
                    "Filename slug is quite long ({} chars): {}. Consider shortening for better readability.",
                    slug_part.len(), filename
                ));
            }

            // Check for consecutive hyphens
            if slug_part.contains("--") {
                result.add_warning(format!(
                    "Filename contains consecutive hyphens: {filename}. Use single hyphens in kebab-case"
                ));
            }
        }

        Ok(())
    }

    /// Validate date format (basic check)
    pub fn is_valid_date_format(&self, date_str: &str) -> bool {
        let parts: Vec<&str> = date_str.split('-').collect();
        if parts.len() != 3 {
            return false;
        }

        // Check year format (4 digits) and reasonable range
        if parts[0].len() != 4 {
            return false;
        }
        if let Ok(year) = parts[0].parse::<u32>() {
            if !(2020..=2030).contains(&year) {
                return false;
            }
        } else {
            return false;
        }

        // Check month format (2 digits) and range (1-12)
        if parts[1].len() != 2 {
            return false;
        }
        if let Ok(month) = parts[1].parse::<u32>() {
            if !(1..=12).contains(&month) {
                return false;
            }
        } else {
            return false;
        }

        // Check day format (2 digits) and range (1-31, basic check)
        if parts[2].len() != 2 {
            return false;
        }
        if let Ok(day) = parts[2].parse::<u32>() {
            if !(1..=31).contains(&day) {
                return false;
            }
        } else {
            return false;
        }

        true
    }

    /// Validate directory of files
    pub fn validate_directory(&self, dir_path: &Path) -> Result<ValidationSummary> {
        let mut summary = ValidationSummary::new();

        if !dir_path.exists() || !dir_path.is_dir() {
            return Err(eyre::eyre!(
                "Directory does not exist: {}",
                dir_path.display()
            ));
        }

        self.validate_directory_recursive(dir_path, &mut summary)?;

        Ok(summary)
    }

    fn validate_directory_recursive(
        &self,
        dir_path: &Path,
        summary: &mut ValidationSummary,
    ) -> Result<()> {
        let entries = fs::read_dir(dir_path)
            .wrap_err_with(|| format!("Failed to read directory: {}", dir_path.display()))?;

        for entry in entries {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                self.validate_directory_recursive(&path, summary)?;
            } else if self.is_markdown_file(&path) {
                // Only validate markdown files
                let result = self.validate_file(&path)?;
                summary.add_file_result(path, result);
            }
        }

        Ok(())
    }
}

/// Validation result for a single file
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
    pub is_valid: bool,
}

impl ValidationResult {
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            is_valid: true,
        }
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationResult {
    pub fn success() -> Self {
        Self::new()
    }

    pub fn add_error(&mut self, error: String) {
        self.errors.push(error);
        self.is_valid = false;
    }

    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }
}

/// Summary of validation results for multiple files
#[derive(Debug)]
pub struct ValidationSummary {
    pub total_files: usize,
    pub valid_files: usize,
    pub files_with_errors: usize,
    pub files_with_warnings: usize,
    pub file_results: Vec<(PathBuf, ValidationResult)>,
}

impl Default for ValidationSummary {
    fn default() -> Self {
        Self::new()
    }
}

impl ValidationSummary {
    pub fn new() -> Self {
        Self {
            total_files: 0,
            valid_files: 0,
            files_with_errors: 0,
            files_with_warnings: 0,
            file_results: Vec::new(),
        }
    }

    pub fn add_file_result(&mut self, path: PathBuf, result: ValidationResult) {
        self.total_files += 1;

        if result.is_valid && !result.has_warnings() {
            self.valid_files += 1;
        }

        if result.has_errors() {
            self.files_with_errors += 1;
        }

        if result.has_warnings() {
            self.files_with_warnings += 1;
        }

        self.file_results.push((path, result));
    }

    pub fn print_summary(&self) {
        info!("│  📋 Markdown Filename Validation Summary:");
        info!("│    📁 Total markdown files: {}", self.total_files);
        info!("│    ✅ Valid filenames: {}", self.valid_files);
        info!("│    ⚠️  Files with warnings: {}", self.files_with_warnings);
        info!("│    ❌ Files with errors: {}", self.files_with_errors);

        // Print errors
        for (path, result) in &self.file_results {
            if result.has_errors() {
                warn!("│    🔥 {}", path.display());
                for error in &result.errors {
                    warn!("│      ❌ {error}");
                }
            }
        }

        // Print warnings
        for (path, result) in &self.file_results {
            if result.has_warnings() && !result.has_errors() {
                info!("│    ⚠️  {}", path.display());
                for warning in &result.warnings {
                    info!("│      ⚠️  {warning}");
                }
            }
        }
    }

    pub fn has_errors(&self) -> bool {
        self.files_with_errors > 0
    }
}
