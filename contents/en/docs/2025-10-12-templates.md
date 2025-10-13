---
title: "Templates & Layouts"
description: "How templates, layouts and components are organized"
lang: "en"
image: "/images/2025-02-24/genwebblog.svg"
author: boychawin
keywords:
  - "templates"
  - "layouts"
  - "handlebars"
  - "components"
---


## Templates, layouts and components

Templates are stored under `source/` and split into several folders:

- `source/layouts/` — base layouts (e.g., `layout.html`, `head.html`)
- `source/pages/` — page-specific templates (e.g., `index.html`, `post.html`, `docs.html`)
- `source/components/` — partials and reusable components (header, footer, cards)

### How templates are registered

The generator registers `.html` files found in `source/layouts`, `source/pages`, and `source` as Handlebars templates. Template keys are derived from relative paths; for example, `source/pages/docs.html` may register as `pages/docs` or `docs` depending on the directory structure.

### Creating a new page template

1. Create `source/pages/my-page.html` (or place under `source/layouts` if it's a generic layout)
2. Use Handlebars expressions to render data, e.g., `{{ title }}` and `{{{ contents }}}`
3. If you add a component, place it under `source/components/` and call it via `{{> components/header }}`

### Partial example

```hbs
{{> components/header}}
<main>
  <h1>{{ title }}</h1>
  {{{ contents }}}
</main>
{{> components/footer}}
```

### Common issues

- `Template not found <name>`: check filename and registration (missing `.html` or wrong folder)
