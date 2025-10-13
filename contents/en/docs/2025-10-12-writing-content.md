---
title: "Writing Content (Markdown)"
description: "How to write posts and pages with Markdown"
lang: "en"
image: "/images/2025-02-24/genwebblog.svg"
author: boychawin
keywords:
  - "markdown"
  - "content writing"
  - "frontmatter"
  - "posts"
---


## Post format (Frontmatter)

Each post should be a Markdown file with YAML frontmatter. Filename must follow `YYYY-MM-DD-slug.md`.

Example frontmatter:

```markdown
---
title: "My First Post"
description: "Short description for SEO"
author: "Your Name"
lang: "en"
layout: "post"
tags: ["rust", "tutorial"]
image: "images/posts/first.webp"
---

# My First Post

Post body...
```

### Field reference

- `title` — Page title (recommended ≤ 60 chars)
- `description` — Meta description (recommended ≤ 160 chars)
- `author` / `author_url` / `author_email`
- `lang` — Language code (e.g., `en`, `th`)
- `layout` — Template to render (e.g., `post`, `docs`, `page`)
- `tags` — Array of tags
- `image` — Cover image path (processed automatically)

## Pages and localized content

Create pages under `contents/` or inside locale directories such as `contents/en/`. Use `layout: "docs"` for documentation pages or `layout: "page"` for generic pages.

## Images and processing

When `image` is provided, the build pipeline processes images (resizing, format conversion) to generate responsive assets.

## SEO best practices

- Provide meaningful `title` and `description`.
- Use semantic headings and include internal links.
- Optimize images and add `alt` attributes.
