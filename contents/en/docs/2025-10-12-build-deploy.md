---
title: "Build & Deploy"
description: "How to build the site and deploy to GitHub/Cloudflare"
lang: "en"
image: "/images/2025-02-24/genwebblog.svg"
author: boychawin
keywords:
  - "build"
  - "deploy"
  - "CI/CD"
  - "continuous integration"
---

## Build

Basic build commands:

```bash
# Build entire site into build/
genwebblog build

# Build only a language
genwebblog build-lang en
```

Output will be placed under `build/`. Non-default languages generate under `build/<code>/`.

## Deploy

Example GitHub settings in `app.toml`:

```toml
[deploy_github]
user = "your-github-user"
repo_name = "your-repo"
token = "your-token"
branch = "main"
```

Cloudflare Pages example:

```toml
[deploy_cloudflare]
account_id = "..."
api_token = "..."
project_name = "..."
```

Deploy commands:

```bash
# Mock/test deploy
genwebblog deploy test

# Deploy to production
genwebblog deploy
```

Checklist before deploy:

- Verify `app.toml` contains correct `app_domain` and deploy credentials
- Ensure build output is up-to-date (`genwebblog build`)
- Test deploy in a staging environment or with `deploy test`

