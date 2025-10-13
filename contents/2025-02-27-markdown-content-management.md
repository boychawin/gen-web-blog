---
title: "การใช้ Markdown ในการจัดการเนื้อหา"
author: boychawin
description: "ในบทความนี้จะพูดถึงการใช้ Markdown เพื่อจัดการและจัดรูปแบบเนื้อหาบนเว็บไซต์ได้ง่ายและรวดเร็ว พร้อมตัวอย่างการใช้งานฟีเจอร์ต่างๆ ของ Markdown เช่น BlockQuote, List, Table, CodeBlock และอื่นๆ"
image: "/images/2025-02-27/markdown.jpg"
keywords:
- markdown
- content management
- example
---

# 1. Document (Heading)

ตัวอย่างการใช้งาน:
```md
# หัวข้อหลัก (H1)
เนื้อหาภายใต้หัวข้อหลัก

## หัวข้อรอง (H2)
เนื้อหาภายใต้หัวข้อรอง

### หัวข้อย่อย (H3)
เนื้อหาภายใต้หัวข้อย่อย

#### หัวข้อเล็กลง (H4)
เนื้อหาภายใต้หัวข้อเล็กลง

##### หัวข้อที่เล็กลงอีก (H5)
เนื้อหาภายใต้หัวข้อที่เล็กลงอีก

###### หัวข้อที่เล็กที่สุด (H6)
เนื้อหาภายใต้หัวข้อที่เล็กที่สุด
```

---

## 2. FrontMatter (metadata)
**เนื้อหา FrontMatter อยู่ในส่วน `---` ด้านบน**

ตัวอย่างการใช้งาน:
```yaml
---
layout: post
title: "การใช้ Markdown"
author: boychawin
description: "ตัวอย่างการใช้ FrontMatter"
---
```

> ### 3. BlockQuote
> นี่คือข้อความใน Block Quote
> ตัวอย่างการอ้างอิงจากข้อความสำคัญหรือการคัดลอกข้อความจากแหล่งข้อมูลภายนอก


---

ตัวอย่างการใช้งาน:
```md
> ### 3. BlockQuote
> นี่คือข้อความใน Block Quote
> ตัวอย่างการอ้างอิงจากข้อความสำคัญหรือการคัดลอกข้อความจากแหล่งข้อมูลภายนอก
```


---

### 4. List Item
- Item 1
- Item 2
  - Nested Item

1. Ordered Item 1
2. Ordered Item 2

ตัวอย่างการใช้งาน:
```md
- Item 1
- Item 2
  - Nested Item

1. Ordered Item 1
2. Ordered Item 2
```

---

### 5. DescriptionList
**Term**
: คำอธิบายของ Term

ตัวอย่างการใช้งาน:
```md
**Term**
: คำอธิบายของ Term
```

---

### 6. CodeBlock
```rust
fn main() {
    println!("Hello, Rust!");
}
```

ตัวอย่างการใช้งาน:
```md
```rust
fn main() {
    println!("Hello, Rust!");
}

```
---

### 7. Code
นี่คือตัวอย่างโค้ด `console.log("Hello, world!");` ที่แทรกอยู่ในบรรทัด

ตัวอย่างการใช้งาน:
```md
นี่คือตัวอย่างโค้ด `console.log("Hello, world!");` ที่แทรกอยู่ในบรรทัด
```
---

### 8. HtmlInline
<a href="https://genwebblog.com">GenWebBlog</a>
ตัวอย่างการใช้ HTML Inline Link

ตัวอย่างการใช้งาน:
```md
<a href="https://genwebblog.com">GenWebBlog</a>
```

---

### 9. Raw
<details>
  <summary>กดเพื่อดูเพิ่มเติม</summary>
  นี่คือเนื้อหาที่ซ่อนอยู่!
</details>

ตัวอย่างการใช้งาน:
```md
<details>
  <summary>กดเพื่อดูเพิ่มเติม</summary>
  นี่คือเนื้อหาที่ซ่อนอยู่!
</details>

```

---

### 10. ThematicBreak
---
ตัวอย่างการใช้งาน:
```md
---
```

---

### 11. LineBreak และ SoftBreak
บรรทัดนี้ไม่มี `br`
บรรทัดนี้ใช้ `br`
<br>

ตัวอย่างการใช้งาน:
```md
บรรทัดนี้ไม่มี `br`
บรรทัดนี้ใช้ `br`
<br>
```

---

### 12. Strong
- **ตัวหนา**

ตัวอย่างการใช้ตัวหนา:
```md
**ตัวหนา**
```

---

### 13. Emph
- *ตัวเอียง*

ตัวอย่างการใช้ตัวเอียง:
```md
*ตัวเอียง*
```

---

### 14. Strikethrough
- ~~ขีดฆ่า~~

ตัวอย่างการใช้ขีดฆ่า:
```md
~~ขีดฆ่า~~
```

---

### 15. Superscript
- x^2^ (ตัวยก)

ตัวอย่างการใช้ตัวยก:
```md
- x^2^ (ตัวยก)
```

---

### 16. Link
[ลิงก์ไปยัง GitHub](https://github.com)

ตัวอย่างการใช้ลิงก์:
```md
[ลิงก์ไปยัง GitHub](https://github.com)
```

---

### 17. Image
![รูปภาพตัวอย่าง](https://boychawin.com/images/apple-touch-icon.png)

ตัวอย่างการแทรกรูปภาพ:
```md
![รูปภาพตัวอย่าง](https://boychawin.com/images/apple-touch-icon.png)
```

---
<!--
### 18. ShortCode (ตัวอย่างจาก Hugo/Zola)
```html
{{< youtube dQw4w9WgXcQ >}}
```

ตัวอย่างการใช้งาน:
```md
{{< youtube dQw4w9WgXcQ >}}
``` -->

---

### 18. Table
| คอลัมน์ 1 | คอลัมน์ 2 |
|-----------|-----------|
| ค่า 1     | ค่า 2     |

ตัวอย่างการใช้งาน:
```md
| คอลัมน์ 1 | คอลัมน์ 2 |
|-----------|-----------|
| ค่า 1     | ค่า 2     |
```

---

### 19. Footnote (Definition, Reference)
ข้อความอ้างอิง[^1]

[^1]: นี่คือ Footnote

ตัวอย่างการใช้ Footnote:
```md
ข้อความอ้างอิง[^1]

[^1]: นี่คือ Footnote
```

---

### 20. TaskItem
- [x] งานที่ทำเสร็จ
- [ ] งานที่ต้องทำ

ตัวอย่างการใช้งาน:
```md
- [x] งานที่ทำเสร็จ
- [ ] งานที่ต้องทำ
```

---

### 21. Escaped
\*ไม่เป็นตัวเอียง\*
\# ไม่เป็น Heading

ตัวอย่างการใช้งาน:
```md
\*ไม่เป็นตัวเอียง\*
\# ไม่เป็น Heading
```

---

### 22. Math
$$
E = mc^2
$$

ตัวอย่างการใช้สมการ:
```md
$$
E = mc^2
$$
```

---

### 23. WikiLink
[[หน้าเกี่ยวข้อง]]

ตัวอย่างการใช้งาน:
```md
[[หน้าเกี่ยวข้อง]]
```

---

### 24. Underline
<u>ข้อความขีดเส้นใต้</u>

ตัวอย่างการใช้งาน:
```md
<u>ข้อความขีดเส้นใต้</u>
```

---

### 25. Subscript
H~2~O (ตัวห้อย)

ตัวอย่างการใช้งาน:
```md
H~2~O (ตัวห้อย)
```

---

### 26. SpoileredText
> ||นี่คือข้อความที่ซ่อนอยู่||

ตัวอย่างการใช้งาน:
```md
> ||นี่คือข้อความที่ซ่อนอยู่||
```

---

### 27. EscapedTag
\&lt;div class="container"&gt;

ตัวอย่างการใช้งาน:
```md
\&lt;div class="container"&gt;
```

---

### 28. Alert
> **Warning**: นี่คือตัวอย่าง Alert

ตัวอย่างการใช้งาน:
```md
> **Warning**: นี่คือตัวอย่าง Alert
```

---

#### **ข้อมูลเพิ่มเติม**
- ดูรายการภาษาทั้งหมดที่รองรับได้ที่ [SUPPORTED_LANGUAGES.md](https://github.com/highlightjs/highlight.js/blob/main/SUPPORTED_LANGUAGES.md)
