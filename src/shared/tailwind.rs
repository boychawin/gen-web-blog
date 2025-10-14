use encre_css::{generate, Config};
use encre_css_typography::register;
use glob::glob;
use std::path::Path;

use log::{error, info};
use toml::Value as TomlValue;

pub fn process_tailwind_files() -> String {
    let config_content = match crate::shared::fs::read_file_to_string("source/tailwind.config.toml")
    {
        Ok(content) => content,
        Err(e) => {
            error!("|  ❌ Failed to read tailwind.config.toml: {e}");
            return String::new();
        }
    };

    let mut config_value: TomlValue = match config_content.parse::<TomlValue>() {
        Ok(value) => value,
        Err(e) => {
            error!("|  ❌ Invalid TOML format: {e}");
            return String::new();
        }
    };

    if let Some(primary_map) = config_value
        .get_mut("theme")
        .and_then(|t| t.get_mut("colors"))
        .and_then(|c| c.get_mut("primary"))
    {
        if let TomlValue::Table(ref primary_table) = primary_map {
            if let Ok(json_str) = serde_json::to_string(&primary_table) {
                *primary_map = TomlValue::String(json_str);
            }
        }
    }

    let patterns = config_value
        .get("theme")
        .and_then(|theme| theme.get("content"))
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect::<Vec<String>>()
        })
        .unwrap_or_default();

    if patterns.is_empty() {
        error!("│  ❌ No patterns found in content");
    }

    let mut config: Config = config_value
        .try_into()
        .expect("❌ Failed to convert to Config");

    register(&mut config);

    let mut classes = Vec::new();

    for pattern in &patterns {
        for entry in glob(pattern).expect("❌ Failed to read glob pattern") {
            match entry {
                Ok(path) => {
                    if let Ok(contents) = crate::shared::fs::read_file_to_string(&path) {
                        classes.push(contents);
                    }
                }
                Err(e) => error!("│  ❌ Error reading file: {e}"),
            }
        }
    }

    let class_refs: Vec<&str> = classes.iter().map(|s| s.as_str()).collect();

    generate(class_refs, &config)
}

pub fn save_css_to_file(bundle: &str, path: &str) {
    if bundle.is_empty() {
        info!("◇  🎨 Empty CSS bundle, skipping write to: {path}");
        return;
    }

    let path_obj = Path::new(path);

    if let Some(parent) = path_obj.parent() {
        if !parent.exists() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                error!("│  ⚠️ Failed to create directory {parent:?}: {e}");
                return;
            }
        }
    }

    if let Err(e) = crate::shared::fs::write_file(path_obj, bundle.as_bytes()) {
        error!("│  ❌ Failed to save CSS to {path}: {e}");
        return;
    }
    info!("◇  🎨 CSS bundle to : {path}");
}
