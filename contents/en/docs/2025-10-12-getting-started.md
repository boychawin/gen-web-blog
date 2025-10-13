---
title: "Getting Started with GenWebBlog"
description: "Installation and getting started guide"
lang: "en"
image: "/images/2025-02-24/genwebblog.svg"
author: boychawin
keywords:
  - "getting started"
  - "install"
  - "quickstart"
  - "genwebblog"
---

This guide is the official getting-started manual for GenWebBlog. It covers installation, creating a new project, and a quick overview of the project layout.

## Overview

GenWebBlog is a fast, Rust-based static site generator optimized for blogs and content sites. It uses Markdown for content, Handlebars for templates, and built-in tools for images, SEO, and deployment.

## Requirements

- Recommended: Rust toolchain if you plan to build from source
- curl (for quick install script)

## Installation (Quick)

```bash
curl -fsSL https://raw.githubusercontent.com/boychawin/gen-web-blog/main/install.sh | bash
```

## Install from source (Developer)

```bash
git clone https://github.com/boychawin/gen-web-blog.git
cd gen-web-blog
cargo build --release
# binary will be at: target/release/genwebblog
```

## Create a new project and start dev server

```bash
genwebblog init my-blog
cd my-blog
genwebblog start
```

Open the dev server (default) at [http://127.0.0.1:3000](http://127.0.0.1:3000)

## Project structure (quick)

- `app.toml` – main configuration
- `contents/` – Markdown posts and YML manifests
- `source/` – templates, layouts, styles
- `public/` – static assets
- `build/` – generated output

Read other docs for writing content, templates, i18n, and deployment.
