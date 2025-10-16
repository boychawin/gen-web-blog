---
title: "การเขียนเนื้อหา (Markdown)"
description: "คำแนะนำการเขียนโพสต์ และหน้าด้วย Markdown"
lang: "th"
image: "/images/2025-02-24/genwebblog.svg"
author: boychawin
keywords:
  - "markdown"
  - "เขียนเนื้อหา"
  - "frontmatter"
  - "โพสต์"
---

## รูปแบบไฟล์โพสต์ (Frontmatter และการตั้งค่า)

ทุกโพสต์ต้องเป็นไฟล์ Markdown ที่มี YAML frontmatter ด้านบนสุด ตัวอย่างที่แนะนำ:

```markdown
---
title: "วิธีการใช้งาน GenWebBlog"
description: "คู่มือสั้น ๆ สำหรับการใช้งาน GenWebBlog"
author: "ทีมพัฒนา"
lang: "th"
layout: "post"
tags: ["guide", "setup"]
image: "images/posts/setup.webp"
---

# วิธีการใช้งาน GenWebBlog

เนื้อหาของโพสต์...
```


คำอธิบายของฟิลด์สำคัญ:

- title: ชื่อบทความ (แนะนำไม่เกิน 60 ตัวอักษร)
- description: คำอธิบายสั้น ๆ สำหรับ meta description (แนะนำไม่เกิน 160 ตัวอักษร)
- author / author_url / author_email: ข้อมูลผู้เขียน
- lang: โค้ดภาษา เช่น `th` หรือ `en`
- layout: เลย์เอาต์ที่ใช้ในการ render (`post`, `page`, `docs`, `articles`)
- tags: แท็กเพื่อกรอง/ค้นหาโพสต์
- image: รูปปก (จะถูกประมวลผลอัตโนมัติ)

## ชื่อไฟล์และวันที่

ชื่อไฟล์ต้องมีรูปแบบ `YYYY-MM-DD-slug.md` เช่น `2025-10-12-how-to-use-genwebblog.md`
ระบบจะอ่านวันที่จากชื่อไฟล์และสร้าง URL ตามโครงสร้างที่กำหนด

## หน้าคงที่ (Pages)

สำหรับเพจคงที่ (เช่น About, Contact, Docs) ให้สร้างไฟล์ Markdown ใน `contents/` หรือโฟลเดอร์ภาษาย่อยเช่น `contents/en/` และใช้ `layout: "page"` หรือ `image: "/images/2025-02-24/genwebblog.svg"` ตามที่ต้องการ ตัวอย่าง:

```markdown
---
title: "About"
description: "About this site"
lang: "en"
layout: "page"
---

# About

รายละเอียดเกี่ยวกับเว็บไซต์...
```

## รูปภาพและการประมวลผล

ถ้าระบุฟิลด์ `image` ระบบจะประมวลผลรูป (resize, convert) อัตโนมัติและสร้างหลายขนาดสำหรับ responsive images

## คำแนะนำการเขียนเพื่อ SEO

- เขียน `title` และ `description` ให้ชัดเจนและไม่ยาวเกินไป
- ใส่ `alt` ในรูปภาพภายใน Markdown (ใช้ HTML img tag หรือ syntax ปกติ)
- ใช้ heading (H1/H2/H3) อย่างมีโครงสร้าง
