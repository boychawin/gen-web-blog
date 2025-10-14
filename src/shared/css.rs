use crate::constants::{build, extensions};
use crate::error::{GenWebBlogError, Result};
use grass;
use minifier::css::minify;
use std::fs::{self, File};
use std::io::Write;
use std::path::Path;

pub fn compile_sass(filename: &str) -> Result<()> {
    let scss_file = format!("source/styles/{filename}.{}", extensions::SCSS);
    let css_file = format!("public/_system_/styles/{filename}.{}", extensions::CSS);

    println!("│  🔍 Checking SCSS file: {scss_file}");

    if !Path::new(&scss_file).exists() {
        println!("│  ⚠️ SCSS file not found: {scss_file}");
        if let Some(parent) = Path::new(&css_file).parent() {
            fs::create_dir_all(parent).map_err(|e| {
                GenWebBlogError::file_system(parent, format!("Failed to create directory: {e}"))
            })?;
        }
        crate::shared::fs::write_file(&css_file, b"/* CSS file generated - SCSS source not found */\n").map_err(|e| {
            GenWebBlogError::css_compilation(&css_file, format!("Failed to write CSS file: {e}"))
        })?;
        return Ok(());
    }

    println!("│  🔧 Compiling SCSS: {scss_file} → {css_file}");

    if let Some(parent) = Path::new(&css_file).parent() {
        fs::create_dir_all(parent).map_err(|e| {
            GenWebBlogError::file_system(parent, format!("Failed to create directory: {e}"))
        })?;
    }

    match grass::from_path(&scss_file, &grass::Options::default()) {
        Ok(css_content) => {
            crate::shared::fs::write_file(&css_file, css_content.as_bytes()).map_err(|e| {
                GenWebBlogError::css_compilation(&css_file, format!("Failed to write CSS file: {e}"))
            })?;
            println!("│  ✅ SASS compiled with grass: {} → {} ({} bytes)", scss_file, css_file, css_content.len());
        }
        Err(e) => {
            println!("│  ⚠️ SASS compilation failed ({e}), using fallback");
            match crate::shared::fs::read_file_to_string(&scss_file) {
                Ok(content) => {
                    let css_content = content
                        .lines()
                        .filter(|line| !line.trim_start().starts_with("//"))
                        .collect::<Vec<&str>>()
                        .join("\n");

                    crate::shared::fs::write_file(&css_file, css_content.as_bytes()).map_err(|e| {
                        GenWebBlogError::css_compilation(&css_file, format!("Failed to write CSS file: {e}"))
                    })?;
                    println!("│  ⚡ Fallback CSS generated: {} ({} bytes)", css_file, css_content.len());
                }
                Err(_) => {
                    let mut file = File::create(&css_file).map_err(|e| {
                        GenWebBlogError::css_compilation(&css_file, format!("Failed to create CSS file: {e}"))
                    })?;
                    file.write_all(b"/* CSS file generated - SCSS source read error */\n").map_err(|e| {
                        GenWebBlogError::css_compilation(&css_file, format!("Failed to write CSS file: {e}"))
                    })?;
                    println!("│  ❌ Error reading SCSS, empty CSS created: {css_file}");
                }
            }
        }
    }

    Ok(())
}

pub fn concat_vendor_css(files: Vec<&str>) -> Result<()> {
    let mut concatted = String::with_capacity(1024 * 1024);
    let _ = fs::create_dir_all("public/_system_/styles");

    for filestem in files {
        let vendor_path = format!("public/_system_/styles/{filestem}.{}", extensions::CSS);
        match crate::shared::fs::read_file_to_string(&vendor_path) {
            Ok(contents) => {
                concatted.push_str(&contents);
                println!("│  🔗 Concatenated: {vendor_path}");

                // attempt to remove the intermediate file after successful concatenation
                match fs::remove_file(&vendor_path) {
                    Ok(_) => println!("│  🗑️ Removed intermediate: {vendor_path}"),
                    Err(e) => eprintln!("│  ⚠️ Failed to remove intermediate {vendor_path}: {e}"),
                }
            }
            Err(err) => {
                eprintln!("│  ⚠️ Missing vendor CSS file {vendor_path}: {err}. Skipping.");
                continue;
            }
        }
    }

    let minified_css = match minify(&concatted) {
        Ok(m) => m.to_string(),
        Err(e) => {
            eprintln!("│  ⚠️ CSS minification failed: {e}. Writing unminified content.");
            concatted
        }
    };

    let vendor_css_path = format!("public/_system_/styles/{}", build::VENDOR_CSS);
    if let Some(parent) = Path::new(&vendor_css_path).parent() {
        fs::create_dir_all(parent).map_err(|e| {
            GenWebBlogError::file_system(parent, format!("Failed to create directory: {e}"))
        })?;
    }
    crate::shared::fs::write_file(&vendor_css_path, minified_css.as_bytes()).map_err(|e| {
        GenWebBlogError::file_system(&vendor_css_path, format!("Failed to write vendor CSS: {e}"))
    })?;

    println!("│  ✅ Vendor CSS created: {} ({} bytes)", vendor_css_path, minified_css.len());
    Ok(())
}
