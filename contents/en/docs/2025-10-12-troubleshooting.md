---
title: "Troubleshooting"
description: "Common issues and how to fix them"
lang: "en"
image: "/images/2025-02-24/genwebblog.svg"
author: boychawin
keywords:
  - "troubleshooting"
  - "errors"
  - "debugging"
  - "template not found"
---


### Template not found

Cause:

- The layout name in frontmatter (`layout: "docs"`) does not have a matching template file under `source/pages/` or `source/layouts/`.

Fix:

1. Ensure the template exists (e.g., `source/pages/docs.html`).
2. Check template path and name — the generator registers templates by relative path.
3. Ensure the template file has a `.html` extension.

### Markdown filename warnings

Cause:

- Filenames not following `YYYY-MM-DD-slug.md` or dates incorrectly formatted.

Fix:

- Rename files to `2025-10-12-my-post.md` (4-digit year, 2-digit month and day).

### Images not processed

Cause:

- Image path incorrect or unsupported format.

Fix:

- Place images in `public/images/` and reference them in frontmatter: `image: "images/xxx.webp"`.

