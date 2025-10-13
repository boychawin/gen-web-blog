use std::fs;
use std::path::Path;
use serde::Deserialize;
use serde_yaml;

fn default_false() -> Option<bool> {
    Some(false)
}

#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GenericYmlInfo {
    #[serde(skip_deserializing)]
    pub page_name: String,
    #[serde(default)]
    pub title: String,
    #[serde(default)]
    pub description: String,
    #[serde(default = "default_false")]
    pub draft: Option<bool>,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    #[serde(rename = "date_modified")]
    pub date_modified: Option<String>,
    #[serde(default)]
    #[serde(rename = "date_published")]
    pub date_published: Option<String>,
    #[serde(default)]
    pub lang: Option<String>,
    #[serde(default)]
    pub layout: Option<String>,
    #[serde(default)]
    pub category: Option<String>,
    #[serde(default)]
    pub link_text: Option<String>,
    #[serde(default)]
    pub author: Option<String>,
    #[serde(default)]
    pub author_url: Option<String>,
    #[serde(default)]
    pub author_email: Option<String>,
}

pub fn read_yaml_metadata<T: for<'de> Deserialize<'de>>(
    file_name: &str,
) -> Result<T, Box<dyn std::error::Error>> {
    let yaml_content = match crate::shared::fs::read_file_to_string(file_name) {
        Ok(content) => content,
        Err(e) => {
            eprintln!("│  ❌ Unable to read: {file_name}: {e}");
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Unable to read file {file_name}: {e}"),
            )));
        }
    };

    match serde_yaml::from_str(&yaml_content) {
        Ok(val) => Ok(val),
        Err(err) => {
            eprintln!("│  ❌ Failed to parse YAML in {file_name}: {err}");
            Err(Box::new(err))
        }
    }
}

pub fn scan_yml_files_in_directory(directory: &str) -> Vec<GenericYmlInfo> {
    scan_yml_files_recursive(directory, directory)
}

fn scan_yml_files_recursive(base_directory: &str, current_directory: &str) -> Vec<GenericYmlInfo> {
    let dir_path = Path::new(current_directory);
    let mut all_yml_info = Vec::new();

    if let Ok(entries) = fs::read_dir(dir_path) {
        for entry in entries.flatten() {
            let path = entry.path();

            if path.is_dir() {
                // Recursively scan subdirectories
                if let Some(subdir_name) = path.file_name().and_then(|name| name.to_str()) {
                    // Skip hidden directories only (dotfiles); allow language folders like `en`
                    if !subdir_name.starts_with('.') {
                        let subdir_path = path.to_string_lossy();
                        let mut subdir_ymls =
                            scan_yml_files_recursive(base_directory, &subdir_path);
                        all_yml_info.append(&mut subdir_ymls);
                    }
                }
            } else if path.extension().and_then(|ext| ext.to_str()) == Some("yml") {
                if let Some(file_stem) = path.file_stem().and_then(|stem| stem.to_str()) {
                    if let Some(path_str) = path.to_str() {
                        if let Ok(mut yml_info) = read_yaml_metadata::<GenericYmlInfo>(path_str) {
                            // Check if this is a subdirectory by comparing with base directory
                            if current_directory != base_directory {
                                // This is in a subdirectory
                                let relative_path = path_str
                                    .strip_prefix(&format!("{base_directory}/"))
                                    .unwrap_or(path_str);
                                // Handle potential language folder like `en` or `th` at the first segment
                                if let Some(slash_pos) = relative_path.find('/') {
                                    let first_segment = &relative_path[..slash_pos];
                                    // If the first segment looks like a language code (2 letters), strip it
                                    let remainder = &relative_path[slash_pos + 1..];
                                    if first_segment.len() == 2
                                        && first_segment.chars().all(|c| c.is_ascii_alphabetic())
                                    {
                                        // Mark detected language on yml_info
                                        yml_info.lang = Some(first_segment.to_string());
                                        // remainder is now the path after the language folder
                                        if let Some(inner_slash) = remainder.find('/') {
                                            let subdir_name = &remainder[..inner_slash];
                                            if file_stem == "index" {
                                                yml_info.page_name = subdir_name.to_string();
                                            } else {
                                                yml_info.page_name =
                                                    format!("{subdir_name}/{file_stem}");
                                            }
                                        } else {
                                            // remainder has no further slash; it's directly the file
                                            yml_info.page_name = file_stem.to_string();
                                        }
                                    } else {
                                        // Not a language folder, keep original behavior
                                        let subdir_name = &relative_path[..slash_pos];
                                        if file_stem == "index" {
                                            yml_info.page_name = subdir_name.to_string();
                                        } else {
                                            yml_info.page_name =
                                                format!("{subdir_name}/{file_stem}");
                                        }
                                    }
                                } else {
                                    yml_info.page_name = file_stem.to_string();
                                }
                            } else {
                                yml_info.page_name = file_stem.to_string();
                            }
                            all_yml_info.push(yml_info);
                        }
                    }
                }
            }
        }
    } else {
        eprintln!("│  ❌ Unable to read directory: {current_directory}");
    }

    all_yml_info
}
