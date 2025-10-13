---
title: "CLI Reference"
description: "Complete CLI command reference"
lang: "en"
image: "/images/2025-02-24/genwebblog.svg"
author: boychawin
keywords:
  - "CLI"
  - "commands"
  - "genwebblog"
  - "usage"
---


### Key commands

`genwebblog init [full]`

- Initialize a new project. If `full` is provided, additional sample files will be created.

`genwebblog start`

- Start the development server (default host/port shown in the startup messages).

`genwebblog build`

- Build static site into `build/`.

`genwebblog build-lang <code>`

- Build only the specified language.

`genwebblog new "Post Title"`

- Create a new post skeleton (with date + slug in filename).

`genwebblog page <name>`

- Create a new page skeleton under `contents/`.

`genwebblog lang install <code>` / `genwebblog lang set-default <code>`

- Manage language packs and default language.

`genwebblog seo` / `genwebblog seo test`

- Run SEO analysis and tests.

`genwebblog deploy` / `genwebblog deploy test`

- Deploy site according to `app.toml` settings or run a test deploy.

For implementation details and more commands see `src/main.rs`.
