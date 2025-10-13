---
title: "Troubleshooting"
description: "Common issues and how to fix them"
lang: "th"
image: "/images/2025-02-24/genwebblog.svg"
author: boychawin
keywords:
  - "troubleshooting"
  - "error"
  - "template not found"
  - "debug"
---


### Template not found

สาเหตุ:

- ชื่อ layout ที่ระบุอยู่ใน frontmatter (`image: "/images/2025-02-24/genwebblog.svg"`) ไม่มีไฟล์เทมเพลตที่ตรงกันใน `source/pages/` หรือ `source/layouts/`

การแก้ไข:

1. ตรวจสอบว่าไฟล์เทมเพลตมีอยู่จริง เช่น `source/pages/docs.html` หรือ `source/layouts/layout.html`
2. เช็คชื่อไฟล์และตำแหน่ง — generator ลงทะเบียนเทมเพลตโดยใช้ path แบบ relative
3. ถ้าสร้างเทมเพลตใหม่ให้แน่ใจว่าไฟล์ลงท้ายด้วย `.html`

### Markdown filename warnings

สาเหตุ:

- ชื่อไฟล์ไม่ตรงตามรูปแบบ `YYYY-MM-DD-slug.md` หรือมีรูปแบบวันที่ผิดพลาด

การแก้ไข:

- เปลี่ยนชื่อไฟล์ให้เป็น `2025-10-12-my-post.md` (ใช้ 4-digit year, 2-digit month/day)

### รูปภาพไม่ถูกประมวลผล

สาเหตุที่เป็นไปได้:

- รูปภาพมีนามสกุลที่ไม่รองรับหรือพาธไม่ถูกต้อง

การแก้ไข:

- วางไฟล์รูปภาพใน `public/images/` หรือ `public/` และระบุพาธใน frontmatter เช่น `image: "images/xxx.webp"`

