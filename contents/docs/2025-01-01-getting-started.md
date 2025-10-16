---
title: "เริ่มต้นกับ GenWebBlog"
description: "วิธีการติดตั้ง และเริ่มใช้งาน GenWebBlog"
lang: "th"
image: "/images/2025-02-24/genwebblog.svg"
author: boychawin
keywords:
  - "เริ่มต้น"
  - "ติดตั้ง"
  - "การใช้งาน"
  - "GenWebBlog"
---

เอกสารนี้เป็นคู่มือเริ่มต้นใช้งาน GenWebBlog — สำหรับผู้ที่ต้องการติดตั้ง สร้างโปรเจกต์แรก และเข้าใจโครงสร้างพื้นฐานของระบบ

## ภาพรวมสั้น ๆ

GenWebBlog เป็น Static Site Generator เขียนด้วย Rust ออกแบบมาสำหรับการเขียนบล็อกและเว็บไซต์คอนเทนต์ โดยมีคุณสมบัติสำคัญเช่น:

- รองรับหลายภาษา (i18n)
- เขียนเนื้อหาเป็น Markdown พร้อม YAML frontmatter
- เทมเพลตด้วย Handlebars
- การจัดการ assets, รูปภาพ และ sitemap อัตโนมัติ

## ความต้องการเบื้องต้น

- Rust toolchain (สำหรับ build จาก source) — ติดตั้งผ่าน rustup
- curl หรือเครื่องมือดาวน์โหลดสำหรับสคริปต์ติดตั้ง

## ติดตั้ง (วิธีที่แนะนำ)

1. ติดตั้งแบบรวดเร็ว (ติดตั้ง binary / สคริปต์):

```bash
curl -fsSL https://raw.githubusercontent.com/boychawin/gen-web-blog/main/install.sh | bash
```

2. ติดตั้งจาก source (สำหรับนักพัฒนา):

```bash
git clone https://github.com/boychawin/gen-web-blog.git
cd gen-web-blog
cargo build --release
# binary จะอยู่ที่ target/release/genwebblog
```

## สร้างโปรเจกต์ใหม่ และรันเซิร์ฟเวอร์พัฒนา

```bash
# สร้างโครงสร้างโปรเจกต์ตัวอย่าง
genwebblog init my-blog
cd my-blog

# เริ่ม server สำหรับพัฒนา (hot reload แบบพื้นฐาน)
genwebblog start
```

เมื่อ server ทำงานแล้ว ให้เปิดเบราว์เซอร์ที่ `http://127.0.0.1:3000` (ค่าเริ่มต้น)

## โครงสร้างโปรเจกต์ที่สำคัญ

- `app.toml` — การตั้งค่าหลักของเว็บไซต์ (domain, languages, deployment)
- `contents/` — Markdown posts และไฟล์ YML ของเพจต่าง ๆ
- `source/` — เทมเพลต, layouts และ styles
- `public/` — static assets ที่จะคัดลอกไปยัง `build/`
- `build/` — โฟลเดอร์เอาต์พุตที่ generator สร้างขึ้น

อ่านส่วนอื่น ๆ ของเอกสารนี้เพื่อเรียนรู้วิธีเขียนโพสต์, สร้างเทมเพลต, และตั้งค่าการ deploy
