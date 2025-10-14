<div align="center">

# 🚀 GenWebBlog

<img src="https://img.shields.io/badge/Rust-1.70+-orange?style=for-the-badge&logo=rust" alt="Rust">
<img src="https://img.shields.io/badge/License-MIT-blue?style=for-the-badge" alt="License">
<img src="https://img.shields.io/badge/Version-1.0.0-green?style=for-the-badge" alt="Version">

**A modern, blazingly fast static site generator built with Rust**

*Perfect for blogs, portfolios, and content-driven websites*

</div>

---

## ✨ **Features**

🌍 **Multilingual Support** - Built-in i18n with easy language management  
⚡ **Lightning Fast** - Rust-powered performance with optimized builds  
🎨 **Modern CSS** - SCSS compilation with TailwindCSS integration  
📱 **SEO Optimized** - Built-in SEO analysis and optimization tools  
🚀 **One-Click Deploy** - Automated GitHub + Cloudflare deployment  
🖼️ **Image Processing** - Automatic image optimization and resizing  
📝 **Markdown First** - Write in Markdown with YAML frontmatter  
🔧 **Developer Friendly** - Hot reloading, validation, and comprehensive CLI  

---

## 📦 **Installation**

### **🍎 macOS (Recommended - Native Support)**

#### **One-Command Installation**
```bash
curl -fsSL https://raw.githubusercontent.com/boychawin/gen-web-blog/main/install.sh | bash
```

✅ **Native Apple Silicon (M1/M2/M3) support**  
✅ **Intel Mac compatibility**  
✅ **Optimized performance**

#### **Download Precompiled Binaries**
1. Go to [Releases](https://github.com/boychawin/gen-web-blog/releases)
2. Download for your Mac:
   - `genwebblog-macos-aarch64.tar.gz` - Apple Silicon (M1/M2/M3)
   - `genwebblog-macos-x86_64.tar.gz` - Intel Macs
3. Extract and add to your PATH

### **🐧 Linux & 🪟 Windows**

Currently, we recommend installing via Cargo for the best experience:

```bash
# Install Rust first: https://rustup.rs/
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Then install GenWebBlog
cargo install --git https://github.com/boychawin/gen-web-blog
```

**Note:** Native Linux and Windows binaries coming soon! We're focusing on delivering the best macOS experience first.

### **🛠️ Build from Source**
```bash
# Prerequisites: Rust 1.75+
git clone https://github.com/boychawin/gen-web-blog.git
cd gen-web-blog
cargo build --release

# Binary will be at: target/release/genwebblog
```

### **✅ Verify Installation**
```bash
genwebblog --help
```

---

## 🎯 **Quick Start**

### 1. **Create Your First Blog**

```bash
# Initialize a new blog
genwebblog init my-awesome-blog
cd my-awesome-blog

# Start development server
genwebblog start
```

### 2. **Create Your First Post**

```bash
# Create a new blog post
genwebblog new "My Amazing First Post"

# Start development server
genwebblog start
```

Your site will be available at `http://127.0.0.1:3000` 🎉

**🍎 macOS users get the best experience with native performance optimization!**

---

## 📖 **Command Reference**

### **Content Management**
```bash
./genwebblog new "Post Title"     # Create new blog post
./genwebblog page about           # Create new page
```

### **Development**
```bash
./genwebblog start               # Start development server
./genwebblog build               # Build static site
./genwebblog build-lang en       # Build for specific language
```

### **Language Management**
```bash
./genwebblog lang list           # List available languages
./genwebblog lang install en     # Install English language pack
./genwebblog lang set-default th # Set Thai as default
./genwebblog lang info en        # Show language details
```

### **SEO & Optimization**
```bash
./genwebblog seo                 # Run SEO analysis
./genwebblog seo test            # Run SEO tests
./genwebblog resize image.jpg    # Optimize images
./genwebblog logo brand.png      # Process logo/favicon
```

### **Deployment**
```bash
./genwebblog deploy              # Deploy to production
./genwebblog deploy test         # Test deployment (mock)
```

---

## 🛠️ **Configuration**

GenWebBlog uses `app.toml` for configuration:

```toml
[app_info]
app_name = "My Awesome Blog"
app_domain = "https://myblog.com"
app_author = "Your Name"
app_description = "My personal blog about technology"

[languages]
installed_languages = ["th", "en"]
default_language = "th"

[deploy_github]
user = "yourusername"
repo_name = "your-blog"
token = "your-github-token"
branch = "main"

[deploy_cloudflare]
account_id = "your-account-id"
api_token = "your-api-token"
project_name = "your-project"
```

---

## 📁 **Project Structure**

```
your-blog/
├── app.toml                    # Main configuration
├── contents/                   # Your blog posts and articles
│   ├── 2024-01-15-hello.md   # Blog posts (YYYY-MM-DD-slug.md)
│   └── index.yml              # Article metadata
├── source/                     # Templates and assets
│   ├── layouts/               # Page layouts
│   ├── templates/             # Page templates
│   ├── styles/                # SCSS files
│   └── translations/          # Language files
├── public/                     # Static assets
│   ├── images/                # Images and media
│   ├── favicon/               # Favicon files
│   └── _system_/              # Generated CSS/JS
└── build/                      # Generated site
```

---

## 🌍 **Multilingual Support**

GenWebBlog has built-in support for multiple languages:

### **Supported Languages**
- 🇹🇭 **Thai (th)** - ไทย
- 🇺🇸 **English (en)** - English  
- 🇯🇵 **Japanese (ja)** - 日本語
- 🇰🇷 **Korean (ko)** - 한국어
- 🇨🇳 **Chinese (zh)** - 中文

### **Adding New Languages**
```bash
# Install a language pack
./genwebblog lang install en

# Customize translations
# Edit: source/translations/en/main.toml

# Set as default (optional)
./genwebblog lang set-default en
```

---

## 🚀 **Deployment Setup**

### **1. GitHub Setup**
1. Create a repository on GitHub
2. Generate a [Personal Access Token](https://github.com/settings/tokens)
3. Add token to `app.toml` configuration

### **2. Cloudflare Pages Setup**
1. Create account at [Cloudflare](https://dash.cloudflare.com/)
2. Go to **Pages** → **Create Project**
3. Connect your GitHub repository
4. Get your **Account ID** and create an **API Token**
5. Add credentials to `app.toml`

### **3. Deploy**
```bash
# Test deployment configuration
./genwebblog deploy test

# Deploy to production
./genwebblog deploy
```

---

## 📝 **Writing Content**

### **Blog Post Format**
```markdown
---
title: "Your Post Title"
description: "Post description for SEO"
author: "Your Name"
date: 2024-01-15
image: "/images/posts/your-post.webp"
tags: ["rust", "blog", "web"]
lang: "en"
---

# Your Post Title

Write your amazing content here using **Markdown**!

## Features

- ✅ Full Markdown support
- ✅ Syntax highlighting
- ✅ Image optimization
- ✅ SEO optimization

## Conclusion

Happy blogging! 🎉
```

### **Filename Convention**
Blog posts must follow the format: `YYYY-MM-DD-slug.md`

**Examples:**
- ✅ `2024-01-15-my-first-post.md`
- ✅ `2024-03-22-rust-performance-tips.md`
- ❌ `my-post.md` (missing date)
- ❌ `2024-1-5-post.md` (wrong date format)

---

## 🔧 **Development**

### **Building from Source**
```bash
# Clone repository
git clone https://github.com/boychawin/gen-web-blog.git
cd gen-web-blog

# Install dependencies (Rust 1.75+)
cargo build

# Run tests
cargo test

# Run with logging
RUST_LOG=debug cargo run start
```

### **macOS-Optimized Builds**
```bash
# Apple Silicon (M1/M2/M3)
cargo build --release --target aarch64-apple-darwin

# Intel Macs
cargo build --release --target x86_64-apple-darwin

# Universal binary (both architectures)
lipo -create \
  target/aarch64-apple-darwin/release/genwebblog \
  target/x86_64-apple-darwin/release/genwebblog \
  -output genwebblog-universal
```

### **Other Platforms**
```bash
# Linux (experimental)
cargo build --release --target x86_64-unknown-linux-gnu

# Windows (experimental)
cargo build --release --target x86_64-pc-windows-msvc
```

**Note:** macOS builds are prioritized and fully tested. Other platforms are community-supported.

---

## 🎨 **Customization**

### **Styling**
- Edit SCSS files in `source/styles/`
- Supports TailwindCSS out of the box
- Automatic CSS compilation and minification

### **Templates**
- Handlebars templating engine
- Layouts in `source/layouts/`
- Templates in `source/templates/`

### **SEO Optimization**
- Automatic sitemap generation
- Meta tag optimization
- Open Graph and Twitter Card support
- Structured data (JSON-LD)

---

## 🔍 **SEO Features**

GenWebBlog includes comprehensive SEO tools:

✅ **Automatic Sitemap** - XML sitemap generation  
✅ **Meta Tags** - Open Graph, Twitter Cards  
✅ **Structured Data** - JSON-LD schema markup  
✅ **Image Optimization** - WebP conversion, lazy loading  
✅ **Performance** - Minified CSS/JS, optimized HTML  
✅ **Validation** - Built-in SEO analysis and testing  

```bash
# Run SEO analysis
./genwebblog seo

# Run SEO tests
./genwebblog seo test
```

---

## 🤝 **Contributing**

We welcome contributions! Please see our [Contributing Guide](CONTRIBUTING.md) for details.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

---

## 📄 **License**

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 **Acknowledgments**

- Built with [Rust](https://www.rust-lang.org/) 🦀
- Powered by [Handlebars](https://handlebarsjs.com/) templating
- Styled with [TailwindCSS](https://tailwindcss.com/)
- Deployed on [Cloudflare Pages](https://pages.cloudflare.com/)

---

## 📞 **Support**

- 📧 **Email**: support@genwebblog.com
- 🐛 **Issues**: [GitHub Issues](https://github.com/boychawin/gen-web-blog/issues)
- 💬 **Discussions**: [GitHub Discussions](https://github.com/boychawin/gen-web-blog/discussions)
- 📖 **Documentation**: [Wiki](https://github.com/boychawin/gen-web-blog/wiki)

---

<div align="center">

**⭐ Star this project if you find it useful!**

🍎 **Optimized for macOS** | 🚀 **Blazingly Fast** | 🌍 **Multilingual**

Made with ❤️ by [Boy Chawin](https://github.com/boychawin)

</div>
