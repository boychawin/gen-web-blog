use eyre::Result;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LanguageConfig {
    pub code: String,
    pub name: String,
    pub native_name: String,
    pub translations: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageManager {
    pub available_languages: HashMap<String, LanguageConfig>,
    pub installed_languages: Vec<String>,
    pub default_language: String,
}

impl LanguageManager {
    #[must_use]
    pub fn new() -> Self {
        Self {
            available_languages: Self::get_default_languages(),
            installed_languages: vec!["th".to_string()], // Default installed
            default_language: "th".to_string(),
        }
    }

    pub fn load_from_file() -> Result<Self> {
        use crate::app::read_config;

        let app_config = read_config();
        let installed_languages = app_config.languages.installed_languages;
        let default_language = app_config.languages.default_language;

        Ok(Self {
            available_languages: Self::get_default_languages(),
            installed_languages,
            default_language,
        })
    }

    pub fn save_to_file(&self) -> Result<()> {
        use crate::app::read_config;
        use std::fs;

        // Read current app.toml
        let mut app_config = read_config();

        // Update languages section
        app_config
            .languages
            .installed_languages
            .clone_from(&self.installed_languages);
        app_config
            .languages
            .default_language
            .clone_from(&self.default_language);

        // Save back to app.toml
        let content = toml::to_string_pretty(&app_config)?;
        fs::write("app.toml", content)?;
        Ok(())
    }

    #[must_use]
    pub fn list_available_languages(&self) -> Vec<&LanguageConfig> {
        self.available_languages.values().collect()
    }

    #[must_use]
    pub fn list_installed_languages(&self) -> Vec<&LanguageConfig> {
        self.installed_languages
            .iter()
            .filter_map(|code| self.available_languages.get(code))
            .collect()
    }

    #[must_use]
    pub fn is_language_available(&self, code: &str) -> bool {
        self.available_languages.contains_key(code)
    }

    #[must_use]
    pub fn is_language_installed(&self, code: &str) -> bool {
        self.installed_languages.contains(&code.to_string())
    }

    pub fn install_language(&mut self, code: &str) -> Result<()> {
        if !self.is_language_available(code) {
            return Err(eyre::eyre!("Language '{}' is not available", code));
        }

        if self.is_language_installed(code) {
            warn!("│  ⚠️  Language '{code}' is already installed");
            return Ok(());
        }

        // Mark language as installed
        self.installed_languages.push(code.to_string());

        // Create translation files
        self.create_translation_files(code)?;

        // Save updated config
        self.save_to_file()?;

        info!("│  ✅ Language '{code}' installed successfully");
        Ok(())
    }

    pub fn uninstall_language(&mut self, code: &str) -> Result<()> {
        if code == self.default_language {
            return Err(eyre::eyre!("Cannot uninstall default language '{}'", code));
        }

        if !self.is_language_installed(code) {
            return Err(eyre::eyre!("Language '{}' is not installed", code));
        }

        // Remove from installed languages
        self.installed_languages.retain(|lang| lang != code);

        // Remove translation files
        Self::remove_translation_files(code)?;

        // Save updated config
        self.save_to_file()?;

        info!("│  ✅ Language '{code}' uninstalled successfully");
        Ok(())
    }

    pub fn set_default_language(&mut self, code: &str) -> Result<()> {
        if !self.is_language_installed(code) {
            return Err(eyre::eyre!(
                "Language '{}' must be installed before setting as default",
                code
            ));
        }

        self.default_language = code.to_string();
        self.save_to_file()?;

        info!("│  ✅ Default language set to '{code}'");
        Ok(())
    }

    #[must_use]
    pub fn get_default_language(&self) -> Option<String> {
        if self.default_language.is_empty() {
            None
        } else {
            Some(self.default_language.clone())
        }
    }

    fn create_translation_files(&self, code: &str) -> Result<()> {
        let translations_dir = format!("source/translations/{code}");
        fs::create_dir_all(&translations_dir)?;

        if let Some(lang_config) = self.available_languages.get(code) {
            // Create main translation file
            let translations_file = format!("{translations_dir}/main.toml");
            let content = toml::to_string_pretty(&lang_config.translations)?;
            fs::write(translations_file, content)?;

            // Create template-specific translation files
            let template_translations = vec![
                ("layout", "ไฟล์เทมเพลตหลัก"),
                ("navigation", "เมนูนำทาง"),
                ("footer", "ส่วนท้ายเว็บไซต์"),
                ("forms", "ฟอร์มต่างๆ"),
                ("messages", "ข้อความแสดงผล"),
            ];

            for (template, description) in template_translations {
                let template_file = format!("{translations_dir}/{template}.toml");
                let template_content = format!(
                    "# {} สำหรับภาษา {}\n[translations]\nexample = \"ตัวอย่าง\"\n",
                    description, lang_config.native_name
                );
                fs::write(template_file, template_content)?;
            }
        }

        Ok(())
    }

    fn remove_translation_files(code: &str) -> Result<()> {
        let translations_dir = format!("source/translations/{code}");
        if Path::new(&translations_dir).exists() {
            fs::remove_dir_all(translations_dir)?;
        }
        Ok(())
    }

    fn get_default_languages() -> HashMap<String, LanguageConfig> {
        let mut languages = HashMap::new();

        // Thai (default installed)
        languages.insert(
            "th".to_string(),
            LanguageConfig {
                code: "th".to_string(),
                name: "Thai".to_string(),
                native_name: "ไทย".to_string(),
                translations: Self::get_thai_translations(),
            },
        );

        // English
        languages.insert(
            "en".to_string(),
            LanguageConfig {
                code: "en".to_string(),
                name: "English".to_string(),
                native_name: "English".to_string(),
                translations: Self::get_english_translations(),
            },
        );

        languages
    }

    fn get_thai_translations() -> HashMap<String, String> {
        let mut translations = HashMap::new();
        translations.insert("home".to_string(), "หน้าหลัก".to_string());
        translations.insert("about".to_string(), "เกี่ยวกับ".to_string());
        translations.insert("contact".to_string(), "ติดต่อ".to_string());
        translations.insert("articles".to_string(), "บทความ".to_string());
        translations.insert("read_more".to_string(), "อ่านเพิ่มเติม".to_string());
        translations.insert("previous".to_string(), "ก่อนหน้า".to_string());
        translations.insert("next".to_string(), "ถัดไป".to_string());
        translations.insert("search".to_string(), "ค้นหา".to_string());
        translations.insert("published_on".to_string(), "เผยแพร่เมื่อ".to_string());
        translations.insert("updated_on".to_string(), "อัปเดตเมื่อ".to_string());
        translations
    }

    fn get_english_translations() -> HashMap<String, String> {
        let mut translations = HashMap::new();
        translations.insert("home".to_string(), "Home".to_string());
        translations.insert("about".to_string(), "About".to_string());
        translations.insert("contact".to_string(), "Contact".to_string());
        translations.insert("articles".to_string(), "Articles".to_string());
        translations.insert("read_more".to_string(), "Read More".to_string());
        translations.insert("previous".to_string(), "Previous".to_string());
        translations.insert("next".to_string(), "Next".to_string());
        translations.insert("search".to_string(), "Search".to_string());
        translations.insert("published_on".to_string(), "Published on".to_string());
        translations.insert("updated_on".to_string(), "Updated on".to_string());
        translations
    }

    #[must_use]
    pub fn get_translation(&self, lang_code: &str, key: &str) -> Option<String> {
        self.available_languages
            .get(lang_code)
            .and_then(|lang| lang.translations.get(key))
            .cloned()
    }

    #[must_use]
    pub fn load_translations(&self, lang_code: &str) -> HashMap<String, String> {
        if let Some(lang_config) = self.available_languages.get(lang_code) {
            lang_config.translations.clone()
        } else {
            // Fallback to default language
            self.available_languages
                .get(&self.default_language)
                .map(|lang| lang.translations.clone())
                .unwrap_or_default()
        }
    }
}

impl Default for LanguageManager {
    fn default() -> Self {
        Self::new()
    }
}

pub fn list_languages() -> Result<()> {
    let manager = LanguageManager::load_from_file()?;

    println!("│  🌍 Available Languages:");
    println!("│");

    for lang in manager.list_available_languages() {
        let is_installed = manager.is_language_installed(&lang.code);
        let status = if is_installed {
            "✅ Installed"
        } else {
            "⬜ Available"
        };
        let default_marker = if lang.code == manager.default_language {
            " (Default)"
        } else {
            ""
        };

        println!(
            "│  {} {} - {} ({}){}",
            status, lang.code, lang.native_name, lang.name, default_marker
        );
    }

    println!("│");
    println!("│  💡 Use './genwebblog lang install <code>' to install a language");
    println!("│  💡 Use './genwebblog lang set-default <code>' to set default language");

    Ok(())
}

pub fn install_language(code: &str) -> Result<()> {
    let mut manager = LanguageManager::load_from_file()?;

    println!("│  🔽 Installing language '{code}'...");
    manager.install_language(code)?;

    // Show installation success message with next steps
    if let Some(lang_config) = manager.available_languages.get(code) {
        println!("│");
        println!(
            "│  🎉 Language '{}' ({}) installed successfully!",
            lang_config.native_name, code
        );
        println!("│");
        println!("│  📁 Translation files created in: source/translations/{code}/");
        println!("│  🔧 You can now customize translations for your templates");
        println!("│");

        if code != manager.default_language {
            println!("│  💡 To set as default: ./genwebblog lang set-default {code}");
        }

        println!("│  💡 Run './genwebblog build' to regenerate with new language support");
    }

    Ok(())
}

pub fn uninstall_language(code: &str) -> Result<()> {
    let mut manager = LanguageManager::load_from_file()?;

    println!("│  🗑️  Uninstalling language '{code}'...");
    manager.uninstall_language(code)?;

    Ok(())
}

pub fn set_default_language(code: &str) -> Result<()> {
    let mut manager = LanguageManager::load_from_file()?;

    println!("│  🔄 Setting default language to '{code}'...");
    manager.set_default_language(code)?;

    println!("│  💡 Run './genwebblog build' to regenerate with new default language");

    Ok(())
}

pub fn show_language_info(code: &str) -> Result<()> {
    let manager = LanguageManager::load_from_file()?;

    if let Some(lang_config) = manager.available_languages.get(code) {
        println!("│  🌍 Language Information:");
        println!("│");
        println!("│  Code: {}", lang_config.code);
        println!("│  Name: {}", lang_config.name);
        println!("│  Native: {}", lang_config.native_name);
        let is_installed = manager.is_language_installed(&lang_config.code);
        println!("│  Installed: {}", if is_installed { "Yes" } else { "No" });

        if code == manager.default_language {
            println!("│  Default: Yes");
        }

        println!("│");
        println!(
            "│  Available Translations: {}",
            lang_config.translations.len()
        );

        for (key, value) in &lang_config.translations {
            println!("│    {key} = {value}");
        }
    } else {
        return Err(eyre::eyre!("Language '{}' not found", code));
    }

    Ok(())
}
