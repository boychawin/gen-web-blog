---
title: "Build & Deploy"
description: "How to build the site and deploy to GitHub/Cloudflare"
lang: "th"
image: "/images/2025-02-24/genwebblog.svg"
author: boychawin
keywords:
  - "build"
  - "deploy"
  - "continuous deployment"
  - "CI/CD"
---

## Build

คำสั่งพื้นฐาน

```bash
# สร้าง static site ลงในโฟลเดอร์ build/
genwebblog build

# สร้างสำหรับภาษาเฉพาะ
genwebblog build-lang en
```

ค่า output จะถูกวางลงใน `build/` (โดย default) และสำหรับภาษาที่ไม่ใช่ดีฟอลต์ จะสร้างโฟลเดอร์เช่น `build/en/`

## Deployment

GenWebBlog รองรับการ deploy ไปยัง GitHub และ Cloudflare (Pages) ผ่านการตั้งค่าใน `app.toml`

ตัวอย่างการตั้งค่า GitHub ใน `app.toml`:

```toml
[deploy_github]
user = "your-github-user"
repo_name = "your-repo"
token = "your-token"
branch = "main"
```

Cloudflare Pages:

```toml
[deploy_cloudflare]
account_id = "..."
api_token = "..."
project_name = "..."
```

Deploy commands

```bash
# Mock/test deploy
genwebblog deploy test

# Deploy to production
genwebblog deploy
```

ข้อแนะนำก่อน deploy

- ตรวจสอบว่า `app.toml` มี `app_domain` และ token ที่ถูกต้อง
- สร้าง branch สำหรับ deploy ถ้าจำเป็น และตั้งค่าใน `app.toml` ให้ชัดเจน

