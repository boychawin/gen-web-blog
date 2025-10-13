use std::path::PathBuf;
use thiserror::Error;

/// Main error type for GenWebBlog operations
#[derive(Error, Debug)]
pub enum GenWebBlogError {
    /// Configuration-related errors
    #[error("Configuration error: {message}")]
    Config { message: String },

    /// File validation errors
    #[error("Validation error in {file}: {message}")]
    Validation { file: PathBuf, message: String },

    /// Template rendering errors
    #[error("Template error in {template}: {message}")]
    Template { template: String, message: String },

    /// Markdown processing errors
    #[error("Markdown processing error in {file}: {message}")]
    Markdown { file: PathBuf, message: String },

    /// Language management errors
    #[error("Language error: {message}")]
    Language { message: String },

    /// Deployment errors
    #[error("Deployment error: {message}")]
    Deploy { message: String },

    /// GitHub API errors
    #[error("GitHub API error: {message}")]
    GitHub { message: String },

    /// Cloudflare API errors
    #[error("Cloudflare API error: {message}")]
    Cloudflare { message: String },

    /// File system operation errors
    #[error("File system error at {path}: {message}")]
    FileSystem { path: PathBuf, message: String },

    /// Network-related errors
    #[error("Network error: {message}")]
    Network { message: String },

    /// Image processing errors
    #[error("Image processing error for {file}: {message}")]
    ImageProcessing { file: PathBuf, message: String },

    /// SEO validation errors
    #[error("SEO validation error: {message}")]
    Seo { message: String },

    /// SASS/CSS compilation errors
    #[error("CSS compilation error for {file}: {message}")]
    CssCompilation { file: PathBuf, message: String },

    /// Server errors
    #[error("Server error: {message}")]
    Server { message: String },

    /// Generic I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// TOML parsing errors
    #[error("TOML parsing error: {0}")]
    TomlParsing(#[from] toml::de::Error),

    /// TOML serialization errors
    #[error("TOML serialization error: {0}")]
    TomlSerialization(#[from] toml::ser::Error),

    /// YAML parsing errors
    #[error("YAML parsing error: {0}")]
    YamlParsing(#[from] serde_yaml::Error),

    /// JSON parsing errors
    #[error("JSON parsing error: {0}")]
    JsonParsing(#[from] serde_json::Error),

    /// HTTP request errors
    #[error("HTTP request error: {0}")]
    Http(#[from] reqwest::Error),

    /// Git repository errors
    #[error("Git repository error: {0}")]
    Git(#[from] git2::Error),

    /// Handlebars template errors
    #[error("Handlebars template error: {0}")]
    Handlebars(#[from] handlebars::RenderError),

    /// Generic eyre errors for backward compatibility
    #[error("Generic error: {0}")]
    Generic(#[from] eyre::Error),
}

/// Result type alias for GenWebBlog operations
pub type Result<T> = std::result::Result<T, GenWebBlogError>;

impl GenWebBlogError {
    /// Create a configuration error
    pub fn config<S: Into<String>>(message: S) -> Self {
        Self::Config {
            message: message.into(),
        }
    }

    /// Create a validation error
    pub fn validation<P: Into<PathBuf>, S: Into<String>>(file: P, message: S) -> Self {
        Self::Validation {
            file: file.into(),
            message: message.into(),
        }
    }

    /// Create a template error
    pub fn template<S: Into<String>>(template: S, message: S) -> Self {
        Self::Template {
            template: template.into(),
            message: message.into(),
        }
    }

    /// Create a markdown processing error
    pub fn markdown<P: Into<PathBuf>, S: Into<String>>(file: P, message: S) -> Self {
        Self::Markdown {
            file: file.into(),
            message: message.into(),
        }
    }

    /// Create a language error
    pub fn language<S: Into<String>>(message: S) -> Self {
        Self::Language {
            message: message.into(),
        }
    }

    /// Create a deployment error
    pub fn deploy<S: Into<String>>(message: S) -> Self {
        Self::Deploy {
            message: message.into(),
        }
    }

    /// Create a GitHub API error
    pub fn github<S: Into<String>>(message: S) -> Self {
        Self::GitHub {
            message: message.into(),
        }
    }

    /// Create a Cloudflare API error
    pub fn cloudflare<S: Into<String>>(message: S) -> Self {
        Self::Cloudflare {
            message: message.into(),
        }
    }

    /// Create a file system error
    pub fn file_system<P: Into<PathBuf>, S: Into<String>>(path: P, message: S) -> Self {
        Self::FileSystem {
            path: path.into(),
            message: message.into(),
        }
    }

    /// Create a network error
    pub fn network<S: Into<String>>(message: S) -> Self {
        Self::Network {
            message: message.into(),
        }
    }

    /// Create an image processing error
    pub fn image_processing<P: Into<PathBuf>, S: Into<String>>(file: P, message: S) -> Self {
        Self::ImageProcessing {
            file: file.into(),
            message: message.into(),
        }
    }

    /// Create an SEO validation error
    pub fn seo<S: Into<String>>(message: S) -> Self {
        Self::Seo {
            message: message.into(),
        }
    }

    /// Create a CSS compilation error
    pub fn css_compilation<P: Into<PathBuf>, S: Into<String>>(file: P, message: S) -> Self {
        Self::CssCompilation {
            file: file.into(),
            message: message.into(),
        }
    }

    /// Create a server error
    pub fn server<S: Into<String>>(message: S) -> Self {
        Self::Server {
            message: message.into(),
        }
    }

    /// Check if this error is retryable (for network operations)
    pub fn is_retryable(&self) -> bool {
        matches!(
            self,
            GenWebBlogError::Network { .. }
                | GenWebBlogError::Http(_)
                | GenWebBlogError::GitHub { .. }
                | GenWebBlogError::Cloudflare { .. }
        )
    }

    /// Get the error category for logging purposes
    pub fn category(&self) -> &'static str {
        match self {
            GenWebBlogError::Config { .. } => "config",
            GenWebBlogError::Validation { .. } => "validation",
            GenWebBlogError::Template { .. } => "template",
            GenWebBlogError::Markdown { .. } => "markdown",
            GenWebBlogError::Language { .. } => "language",
            GenWebBlogError::Deploy { .. } => "deploy",
            GenWebBlogError::GitHub { .. } => "github",
            GenWebBlogError::Cloudflare { .. } => "cloudflare",
            GenWebBlogError::FileSystem { .. } => "filesystem",
            GenWebBlogError::Network { .. } => "network",
            GenWebBlogError::ImageProcessing { .. } => "image",
            GenWebBlogError::Seo { .. } => "seo",
            GenWebBlogError::CssCompilation { .. } => "css",
            GenWebBlogError::Server { .. } => "server",
            GenWebBlogError::Io(_) => "io",
            GenWebBlogError::TomlParsing(_) => "toml_parsing",
            GenWebBlogError::TomlSerialization(_) => "toml_serialization",
            GenWebBlogError::YamlParsing(_) => "yaml_parsing",
            GenWebBlogError::JsonParsing(_) => "json_parsing",
            GenWebBlogError::Http(_) => "http",
            GenWebBlogError::Git(_) => "git",
            GenWebBlogError::Handlebars(_) => "handlebars",
            GenWebBlogError::Generic(_) => "generic",
        }
    }

    /// Get a user-friendly error message for display
    pub fn user_message(&self) -> String {
        match self {
            GenWebBlogError::Config { message } => {
                format!("⚙️  Configuration issue: {message}")
            }
            GenWebBlogError::Validation { file, message } => {
                format!("📋 Validation failed for {}: {message}", file.display())
            }
            GenWebBlogError::Template { template, message } => {
                format!("🎨 Template error in '{template}': {message}")
            }
            GenWebBlogError::Markdown { file, message } => {
                format!(
                    "📝 Markdown processing failed for {}: {message}",
                    file.display()
                )
            }
            GenWebBlogError::Language { message } => {
                format!("🌍 Language issue: {message}")
            }
            GenWebBlogError::Deploy { message } => {
                format!("🚀 Deployment failed: {message}")
            }
            GenWebBlogError::GitHub { message } => {
                format!("🐙 GitHub API error: {message}")
            }
            GenWebBlogError::Cloudflare { message } => {
                format!("☁️  Cloudflare API error: {message}")
            }
            GenWebBlogError::FileSystem { path, message } => {
                format!("📁 File system error at {}: {message}", path.display())
            }
            GenWebBlogError::Network { message } => {
                format!("🌐 Network error: {message}")
            }
            GenWebBlogError::ImageProcessing { file, message } => {
                format!(
                    "🖼️  Image processing failed for {}: {message}",
                    file.display()
                )
            }
            GenWebBlogError::Seo { message } => {
                format!("🔍 SEO validation issue: {message}")
            }
            GenWebBlogError::CssCompilation { file, message } => {
                format!(
                    "🎨 CSS compilation failed for {}: {message}",
                    file.display()
                )
            }
            GenWebBlogError::Server { message } => {
                format!("🖥️  Server error: {message}")
            }
            _ => format!("❌ Error: {self}"),
        }
    }
}

/// Convenience macro for creating configuration errors
#[macro_export]
macro_rules! config_error {
    ($msg:expr) => {
        $crate::error::GenWebBlogError::config($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::GenWebBlogError::config(format!($fmt, $($arg)*))
    };
}

/// Convenience macro for creating validation errors
#[macro_export]
macro_rules! validation_error {
    ($file:expr, $msg:expr) => {
        $crate::error::GenWebBlogError::validation($file, $msg)
    };
    ($file:expr, $fmt:expr, $($arg:tt)*) => {
        $crate::error::GenWebBlogError::validation($file, format!($fmt, $($arg)*))
    };
}

/// Convenience macro for creating deployment errors
#[macro_export]
macro_rules! deploy_error {
    ($msg:expr) => {
        $crate::error::GenWebBlogError::deploy($msg)
    };
    ($fmt:expr, $($arg:tt)*) => {
        $crate::error::GenWebBlogError::deploy(format!($fmt, $($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_error_creation() {
        let config_err = GenWebBlogError::config("Invalid configuration");
        assert_eq!(config_err.category(), "config");
        assert!(config_err.user_message().contains("Configuration issue"));

        let validation_err =
            GenWebBlogError::validation(PathBuf::from("test.md"), "Invalid filename format");
        assert_eq!(validation_err.category(), "validation");
        assert!(validation_err.user_message().contains("Validation failed"));
    }

    #[test]
    fn test_retryable_errors() {
        let network_err = GenWebBlogError::network("Connection timeout");
        assert!(network_err.is_retryable());

        let config_err = GenWebBlogError::config("Invalid setting");
        assert!(!config_err.is_retryable());
    }

    #[test]
    fn test_error_macros() {
        let err = config_error!("Test error with value: {}", 42);
        assert_eq!(err.category(), "config");

        let err = validation_error!(PathBuf::from("test.md"), "Invalid format");
        assert_eq!(err.category(), "validation");

        let err = deploy_error!("Deployment failed with code: {}", 500);
        assert_eq!(err.category(), "deploy");
    }
}
