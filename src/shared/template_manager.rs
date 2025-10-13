use eyre::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use log::info;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeConfig {
    pub name: String,
    pub description: String,
    pub version: String,
    pub author: String,
    pub colors: ThemeColors,
    pub typography: ThemeTypography,
    pub layout: ThemeLayout,
    pub features: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeColors {
    pub primary: String,
    pub secondary: String,
    pub accent: String,
    pub background: String,
    pub text: String,
    pub text_secondary: String,
    pub border: String,
    pub link: String,
    pub link_hover: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeTypography {
    pub font_family_heading: String,
    pub font_family_body: String,
    pub font_size_base: String,
    pub line_height: String,
    pub heading_scale: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeLayout {
    pub max_width: String,
    pub sidebar_width: String,
    pub header_height: String,
    pub footer_height: String,
    pub spacing_unit: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TemplateFile {
    pub name: String,
    pub path: String,
    pub content: String,
    pub template_type: TemplateType,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TemplateType {
    Layout,
    Component,
    Page,
    Partial,
    Style,
}

pub struct TemplateManager {
    pub themes: HashMap<String, ThemeConfig>,
    pub current_theme: Option<String>,
}

impl Default for ThemeColors {
    fn default() -> Self {
        Self {
            primary: "#3B82F6".to_string(),
            secondary: "#64748B".to_string(),
            accent: "#F59E0B".to_string(),
            background: "#FFFFFF".to_string(),
            text: "#1F2937".to_string(),
            text_secondary: "#6B7280".to_string(),
            border: "#E5E7EB".to_string(),
            link: "#3B82F6".to_string(),
            link_hover: "#2563EB".to_string(),
        }
    }
}

impl Default for ThemeTypography {
    fn default() -> Self {
        Self {
            font_family_heading: "'Inter', -apple-system, BlinkMacSystemFont, sans-serif"
                .to_string(),
            font_family_body: "'Inter', -apple-system, BlinkMacSystemFont, sans-serif".to_string(),
            font_size_base: "16px".to_string(),
            line_height: "1.6".to_string(),
            heading_scale: 1.25,
        }
    }
}

impl Default for ThemeLayout {
    fn default() -> Self {
        Self {
            max_width: "1200px".to_string(),
            sidebar_width: "280px".to_string(),
            header_height: "64px".to_string(),
            footer_height: "auto".to_string(),
            spacing_unit: "1rem".to_string(),
        }
    }
}

impl Default for TemplateManager {
    fn default() -> Self {
        Self::new()
    }
}

impl TemplateManager {
    pub fn new() -> Self {
        Self {
            themes: HashMap::new(),
            current_theme: None,
        }
    }

    pub fn load_themes(&mut self) -> Result<()> {
        // Load built-in themes
        self.load_builtin_themes()?;

        // Load custom themes from templates/themes directory
        let themes_dir = "templates/themes";
        if Path::new(themes_dir).exists() {
            for entry in fs::read_dir(themes_dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    if let Some(theme_name) = path.file_name().and_then(|n| n.to_str()) {
                        if let Ok(theme) = Self::load_theme_config(&path) {
                            self.themes.insert(theme_name.to_string(), theme);
                        }
                    }
                }
            }
        }

        Ok(())
    }

    fn load_builtin_themes(&mut self) -> Result<()> {
        // Default theme
        let default_theme = ThemeConfig {
            name: "Default".to_string(),
            description: "Clean and minimal design perfect for any content".to_string(),
            version: "1.0.0".to_string(),
            author: "GenWebBlog Team".to_string(),
            colors: ThemeColors::default(),
            typography: ThemeTypography::default(),
            layout: ThemeLayout::default(),
            features: vec![
                "Responsive design".to_string(),
                "SEO optimized".to_string(),
                "Fast loading".to_string(),
                "Accessibility ready".to_string(),
            ],
        };
        self.themes.insert("default".to_string(), default_theme);

        // Modern theme
        let modern_theme = ThemeConfig {
            name: "Modern".to_string(),
            description: "Contemporary design with gradients and smooth animations".to_string(),
            version: "1.0.0".to_string(),
            author: "GenWebBlog Team".to_string(),
            colors: ThemeColors {
                primary: "#6366F1".to_string(),
                secondary: "#8B5CF6".to_string(),
                accent: "#06B6D4".to_string(),
                background: "linear-gradient(135deg, #667eea 0%, #764ba2 100%)".to_string(),
                text: "#1F2937".to_string(),
                text_secondary: "#6B7280".to_string(),
                border: "#E5E7EB".to_string(),
                link: "#6366F1".to_string(),
                link_hover: "#4F46E5".to_string(),
            },
            typography: ThemeTypography {
                font_family_heading: "'Poppins', sans-serif".to_string(),
                font_family_body: "'Inter', sans-serif".to_string(),
                font_size_base: "16px".to_string(),
                line_height: "1.7".to_string(),
                heading_scale: 1.3,
            },
            layout: ThemeLayout {
                max_width: "1400px".to_string(),
                sidebar_width: "320px".to_string(),
                header_height: "80px".to_string(),
                footer_height: "auto".to_string(),
                spacing_unit: "1.5rem".to_string(),
            },
            features: vec![
                "Gradient backgrounds".to_string(),
                "Smooth animations".to_string(),
                "Modern typography".to_string(),
                "Card-based layout".to_string(),
                "Dark mode toggle".to_string(),
            ],
        };
        self.themes.insert("modern".to_string(), modern_theme);

        // Dark theme
        let dark_theme = ThemeConfig {
            name: "Dark".to_string(),
            description: "Dark mode optimized for comfortable night reading".to_string(),
            version: "1.0.0".to_string(),
            author: "GenWebBlog Team".to_string(),
            colors: ThemeColors {
                primary: "#60A5FA".to_string(),
                secondary: "#A78BFA".to_string(),
                accent: "#34D399".to_string(),
                background: "#111827".to_string(),
                text: "#F9FAFB".to_string(),
                text_secondary: "#D1D5DB".to_string(),
                border: "#374151".to_string(),
                link: "#60A5FA".to_string(),
                link_hover: "#93C5FD".to_string(),
            },
            typography: ThemeTypography::default(),
            layout: ThemeLayout::default(),
            features: vec![
                "Dark mode design".to_string(),
                "Eye-friendly colors".to_string(),
                "High contrast".to_string(),
                "Night reading optimized".to_string(),
            ],
        };
        self.themes.insert("dark".to_string(), dark_theme);

        // Blog theme
        let blog_theme = ThemeConfig {
            name: "Blog".to_string(),
            description: "Blog-focused layout optimized for reading experience".to_string(),
            version: "1.0.0".to_string(),
            author: "GenWebBlog Team".to_string(),
            colors: ThemeColors {
                primary: "#059669".to_string(),
                secondary: "#7C3AED".to_string(),
                accent: "#DC2626".to_string(),
                background: "#FEFEFE".to_string(),
                text: "#111827".to_string(),
                text_secondary: "#4B5563".to_string(),
                border: "#D1D5DB".to_string(),
                link: "#059669".to_string(),
                link_hover: "#047857".to_string(),
            },
            typography: ThemeTypography {
                font_family_heading: "'Merriweather', serif".to_string(),
                font_family_body: "'Source Sans Pro', sans-serif".to_string(),
                font_size_base: "18px".to_string(),
                line_height: "1.8".to_string(),
                heading_scale: 1.2,
            },
            layout: ThemeLayout {
                max_width: "800px".to_string(),
                sidebar_width: "300px".to_string(),
                header_height: "60px".to_string(),
                footer_height: "auto".to_string(),
                spacing_unit: "1.25rem".to_string(),
            },
            features: vec![
                "Reading-optimized typography".to_string(),
                "Serif headings".to_string(),
                "Comfortable line spacing".to_string(),
                "Distraction-free layout".to_string(),
            ],
        };
        self.themes.insert("blog".to_string(), blog_theme);

        // Business theme
        let business_theme = ThemeConfig {
            name: "Business".to_string(),
            description: "Professional corporate design for business websites".to_string(),
            version: "1.0.0".to_string(),
            author: "GenWebBlog Team".to_string(),
            colors: ThemeColors {
                primary: "#1F2937".to_string(),
                secondary: "#374151".to_string(),
                accent: "#F59E0B".to_string(),
                background: "#FFFFFF".to_string(),
                text: "#111827".to_string(),
                text_secondary: "#6B7280".to_string(),
                border: "#E5E7EB".to_string(),
                link: "#1F2937".to_string(),
                link_hover: "#374151".to_string(),
            },
            typography: ThemeTypography {
                font_family_heading: "'Roboto', sans-serif".to_string(),
                font_family_body: "'Open Sans', sans-serif".to_string(),
                font_size_base: "16px".to_string(),
                line_height: "1.6".to_string(),
                heading_scale: 1.25,
            },
            layout: ThemeLayout {
                max_width: "1200px".to_string(),
                sidebar_width: "250px".to_string(),
                header_height: "70px".to_string(),
                footer_height: "auto".to_string(),
                spacing_unit: "1rem".to_string(),
            },
            features: vec![
                "Corporate design".to_string(),
                "Professional typography".to_string(),
                "Clean layout".to_string(),
                "Business-focused".to_string(),
            ],
        };
        self.themes.insert("business".to_string(), business_theme);

        // Creative theme
        let creative_theme = ThemeConfig {
            name: "Creative".to_string(),
            description: "Artistic and vibrant design for creative portfolios".to_string(),
            version: "1.0.0".to_string(),
            author: "GenWebBlog Team".to_string(),
            colors: ThemeColors {
                primary: "#EC4899".to_string(),
                secondary: "#8B5CF6".to_string(),
                accent: "#F97316".to_string(),
                background: "#FEFEFE".to_string(),
                text: "#1F2937".to_string(),
                text_secondary: "#6B7280".to_string(),
                border: "#E5E7EB".to_string(),
                link: "#EC4899".to_string(),
                link_hover: "#DB2777".to_string(),
            },
            typography: ThemeTypography {
                font_family_heading: "'Playfair Display', serif".to_string(),
                font_family_body: "'Nunito', sans-serif".to_string(),
                font_size_base: "17px".to_string(),
                line_height: "1.7".to_string(),
                heading_scale: 1.4,
            },
            layout: ThemeLayout {
                max_width: "1300px".to_string(),
                sidebar_width: "350px".to_string(),
                header_height: "90px".to_string(),
                footer_height: "auto".to_string(),
                spacing_unit: "1.75rem".to_string(),
            },
            features: vec![
                "Vibrant colors".to_string(),
                "Creative typography".to_string(),
                "Artistic layout".to_string(),
                "Portfolio ready".to_string(),
            ],
        };
        self.themes.insert("creative".to_string(), creative_theme);

        Ok(())
    }

    fn load_theme_config(theme_dir: &Path) -> Result<ThemeConfig> {
        let config_path = theme_dir.join("theme.toml");
        let content = crate::shared::fs::read_file_to_string(config_path)?;
        let config: ThemeConfig = toml::from_str(&content)?;
        Ok(config)
    }

    pub fn get_theme(&self, name: &str) -> Option<&ThemeConfig> {
        self.themes.get(name)
    }

    pub fn list_themes(&self) -> Vec<(&String, &ThemeConfig)> {
        self.themes.iter().collect()
    }

    pub fn apply_theme(&mut self, theme_name: &str) -> Result<()> {
        if let Some(theme) = self.themes.get(theme_name) {
            self.current_theme = Some(theme_name.to_string());

            // Generate CSS variables from theme
            Self::generate_theme_css(theme)?;

            // Update configuration
            Self::update_theme_config(theme_name)?;

            info!("│  ✅ Theme '{}' applied successfully", theme.name);
            Ok(())
        } else {
            Err(eyre::eyre!("Theme '{}' not found", theme_name))
        }
    }

    fn generate_theme_css(theme: &ThemeConfig) -> Result<()> {
        let css_content = format!(
            r"/* GenWebBlog Theme: {} */
:root {{
  /* Colors */
  --color-primary: {};
  --color-secondary: {};
  --color-accent: {};
  --color-background: {};
  --color-text: {};
  --color-text-secondary: {};
  --color-border: {};
  --color-link: {};
  --color-link-hover: {};

  /* Typography */
  --font-heading: {};
  --font-body: {};
  --font-size-base: {};
  --line-height: {};
  --heading-scale: {};

  /* Layout */
  --max-width: {};
  --sidebar-width: {};
}}

/* Base styles using theme variables */
body {{
  font-family: var(--font-body);
  font-size: var(--font-size-base);
  line-height: var(--line-height);
  color: var(--color-text);
  background-color: var(--color-background);
}}

h1, h2, h3, h4, h5, h6 {{
  font-family: var(--font-heading);
  color: var(--color-text);
}}

a {{
  color: var(--color-link);
}}

a:hover {{
  color: var(--color-link-hover);
}}

.container {{
  max-width: var(--max-width);
  margin: 0 auto;
  padding: 0 1rem;
}}

.sidebar {{
  width: var(--sidebar-width);
}}

/* Theme: {} specific enhancements */
",
            theme.name,
            theme.colors.primary,
            theme.colors.secondary,
            theme.colors.accent,
            theme.colors.background,
            theme.colors.text,
            theme.colors.text_secondary,
            theme.colors.border,
            theme.colors.link,
            theme.colors.link_hover,
            theme.typography.font_family_heading,
            theme.typography.font_family_body,
            theme.typography.font_size_base,
            theme.typography.line_height,
            theme.typography.heading_scale,
            theme.layout.max_width,
            theme.layout.sidebar_width,
            theme.name
        );

        // Write theme CSS to public directory
        let out_path = Path::new("public/_system_/styles/theme.css");
        if let Some(parent) = out_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        crate::shared::fs::write_file(out_path, css_content.as_bytes())?;

        Ok(())
    }

    fn update_theme_config(theme_name: &str) -> Result<()> {
        // This would update app.toml with the selected theme
        // For now, we'll just create a theme file
        let _ = std::fs::create_dir_all("config");
        crate::shared::fs::write_file("config/current_theme.txt", theme_name.as_bytes())?;
        Ok(())
    }

    pub fn create_custom_theme(&mut self, name: &str, base_theme: &str) -> Result<()> {
        if let Some(base) = self.themes.get(base_theme).cloned() {
            let theme_dir = format!("templates/themes/{name}");
            let _ = std::fs::create_dir_all(&theme_dir);

            // Create theme configuration
            let mut custom_theme = base;
            custom_theme.name = name.to_string();
            custom_theme.description = format!("Custom theme based on {base_theme}");
            custom_theme.version = "1.0.0".to_string();
            custom_theme.author = "User".to_string();

            let config_content = toml::to_string_pretty(&custom_theme)?;
            crate::shared::fs::write_file(format!("{theme_dir}/theme.toml"), config_content.as_bytes())?;

            // Add to themes collection
            self.themes.insert(name.to_string(), custom_theme);

            info!("│  ✅ Custom theme '{name}' created successfully");
            Ok(())
        } else {
            Err(eyre::eyre!("Base theme '{}' not found", base_theme))
        }
    }
}
