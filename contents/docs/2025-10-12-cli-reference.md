---
title: "CLI Reference"
description: "คำสั่ง CLI ทั้งหมดและอธิบายการใช้งาน"
lang: "th"
image: "/images/2025-02-24/genwebblog.svg"
author: boychawin
keywords:
  - "CLI"
  - "commands"
  - "genwebblog"
  - "usage"
---

คำสั่งหลัก (ตัวอย่าง)

`genwebblog init [full]`

- สร้างโครงสร้างโปรเจกต์เริ่มต้น หากระบุ `full` จะเพิ่มไฟล์ตัวอย่างเพิ่มเติม

`genwebblog start`

- เริ่ม development server (ที่อยู่: `http://127.0.0.1:3000` โดยปกติ)

`genwebblog build`

- สร้าง static site ลงใน `build/`

`genwebblog build-lang <code>`

- สร้างเฉพาะภาษาที่ระบุ เช่น `genwebblog build-lang en`

`genwebblog new "Post Title"`

- สร้างไฟล์โพสต์ใหม่ (จะสร้างไฟล์ด้วยวันที่และ slug ให้โดยอัตโนมัติ)

`genwebblog page <name>`

- สร้างเพจ codeless เช่น `genwebblog page about`

`genwebblog lang install <code>` / `genwebblog lang set-default <code>`

- จัดการภาษาที่ติดตั้งและตั้ง default language

`genwebblog seo` / `genwebblog seo test`

- รันการตรวจสอบ SEO และการทดสอบที่เกี่ยวข้อง

`genwebblog deploy` / `genwebblog deploy test`

- คำสั่งสำหรับ deploy ตามการตั้งค่าใน `app.toml`

ดู `src/main.rs` เพื่อรายละเอียดการทำงานภายในและคำสั่งเพิ่มเติม
