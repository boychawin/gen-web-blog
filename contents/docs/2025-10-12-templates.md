---
title: "Templates & Layouts"
description: "How templates, layouts and components are organized"
lang: "th"
image: "/images/2025-02-24/genwebblog.svg"
author: boychawin
keywords:
  - "templates"
  - "layouts"
  - "handlebars"
  - "components"
---


## โครงสร้างเทมเพลต และการสร้าง Layout

เทมเพลตของ GenWebBlog อยู่ภายใต้โฟลเดอร์ `source/` โดยมีโครงสร้างหลักดังนี้:

- `source/layouts/` — containing base layout files (e.g., `layout.html`, `head.html`) ซึ่งส่วนใหญ่จะเป็น layout หลักของเว็บไซต์
- `source/pages/` — page templates ที่ใช้ render เพจเฉพาะ เช่น `index.html`, `post.html`, `docs.html`
- `source/components/` — reusable partials/components (เช่น header, footer, card) ที่เรียกใช้จาก layouts และ pages

การลงทะเบียนเทมเพลต

ตัว generator จะ scan โฟลเดอร์ `source/layouts`, `source/pages`, และ `source` แล้วลงทะเบียนไฟล์ `.html` เป็น template name โดยจะใช้ path แบบ relative เป็น key ตัวอย่าง:

- `source/pages/docs.html` จะลงทะเบียนเป็น `docs` หรือ `pages/docs` ขึ้นกับการตั้งค่าและ path key

การสร้าง layout ใหม่

1. สร้างไฟล์ใน `source/layouts/` หรือ `source/pages/` ขึ้นอยู่กับว่าเป็น layout หรือหน้าเฉพาะ
2. ใช้ Handlebars syntax ในไฟล์ เช่น `{{ title }}`, `{{{ contents }}}` และ partials เช่น `{{> components/header }}`
3. ใน Markdown frontmatter ให้ตั้ง `layout: "your-template-name"` เพื่อให้ generator ใช้ template นั้นเมื่อ render

ตัวอย่าง partial การเรียกใช้ header และ footer:

```hbs
{{> components/header}}
{{{ contents }}}
{{> components/footer}}
```

ข้อควรระวัง

- หากเห็นข้อผิดพลาด `Template not found <name>` ให้ตรวจสอบว่าชื่อไฟล์เทมเพลตตรงกับคีย์ที่ generator ลงทะเบียน (เช่น ชื่อไฟล์และตำแหน่ง)
- หากต้องการใช้ component ใหม่ ให้สร้างไฟล์ใน `source/components/` และเรียกใช้งานเป็น partial

