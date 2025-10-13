---
title: "วิธีแก้ Properly Size Images และทำให้รองรับ AVIF + WebP"
author: boychawin
description: "ในบทความนี้เราจะมาดูวิธีการปรับขนาดภาพให้เหมาะสม (Properly Size Images) และการแปลงไฟล์ภาพให้รองรับรูปแบบ AVIF และ WebP ซึ่งเป็นรูปแบบที่ช่วยเพิ่มความเร็วในการโหลดเว็บไซต์และมีการบีบอัดที่มีประสิทธิภาพสูง ช่วยปรับปรุงประสิทธิภาพของเว็บไซต์และ SEO ได้อย่างมีประสิทธิภาพ."
image: "/images/2025-02-24/genwebblog.svg"
keywords:
  - properly-size-images
---

## 🔥 Why Image Optimization Matters?

เมื่อพัฒนาเว็บไซต์ หนึ่งในปัจจัยที่มีผลต่อ Performance และ SEO คือ ขนาดไฟล์รูปภาพ ถ้าภาพมีขนาดใหญ่เกินไปจะทำให้ (GenWebBlog เราจัดการให้นะ)

- 🚀 PageSpeed Insights แจ้งเตือน "Properly Size Images"
- ⏳ โหลดช้า ทำให้ LCP (Largest Contentful Paint) สูงขึ้น
- ❌ SEO คะแนนตก เนื่องจาก Google ให้ความสำคัญกับความเร็วเว็บไซต์

### 🎯 เป้าหมาย

- ปรับขนาดรูปภาพให้เหมาะสมกับทุกอุปกรณ์ (Responsive Images)
- รองรับ AVIF และ WebP เพื่อลดขนาดไฟล์
- ทำให้ PageSpeed Insights ✅ ผ่าน

***

### 🛠️ Step 1: Resize & Compress Images

เนื่องจาก PageSpeed Insights แนะนำว่าภาพใหญ่เกินไป จึงควร สร้างหลายขนาด เช่น

- 📏 `1200px` (Desktop)
- 📏 `1024px` (Tablet)
- 📏 `768px` (Mobile)
- 📏 `640px` (Small Devices)

📌 ใช้ ImageMagick & FFmpeg แปลงภาพให้มีขนาดเหมาะสม พร้อมลดขนาดไฟล์

### Convert JPEG → WebP & AVIF

```bash
# Resize และลดขนาด JPEG
magick input.jpg -resize 1200x -quality 50 output-1200.jpg
magick input.jpg -resize 1024x -quality 50 output-1024.jpg
magick input.jpg -resize 768x -quality 50 output-768.jpg
magick input.jpg -resize 640x -quality 50 output-640.jpg

# แปลงเป็น WebP
cwebp -q 50 output-1200.jpg -o output-1200.webp
cwebp -q 50 output-1024.jpg -o output-1024.webp
cwebp -q 50 output-768.jpg -o output-768.webp
cwebp -q 50 output-640.jpg -o output-640.webp

# แปลงเป็น AVIF (ต้องมี ffmpeg หรือ libavif)
ffmpeg -i output-1200.jpg -c:v libaom-av1 -crf 40 -b:v 0 output-1200.avif
ffmpeg -i output-1024.jpg -c:v libaom-av1 -crf 40 -b:v 0 output-1024.avif
ffmpeg -i output-768.jpg -c:v libaom-av1 -crf 40 -b:v 0 output-768.avif
ffmpeg -i output-640.jpg -c:v libaom-av1 -crf 40 -b:v 0 output-640.avif
```

💡 Tips:

- ใช้ -resize WIDTHx เพื่อกำหนดความกว้างที่ต้องการ (อัตราส่วนจะถูกคงไว้)
- WebP รองรับทุกเบราว์เซอร์หลัก
- AVIF บีบอัดได้ดีกว่า WebP แต่ยังมีเบราว์เซอร์บางตัวที่ไม่รองรับ

***

### Step 2: Use `<picture>` for Responsive Images

เพื่อให้เว็บเลือกใช้ไฟล์ภาพที่เหมาะสมกับแต่ละอุปกรณ์ ควรใช้ `<picture>` และ srcset ดังนี้

```html
<picture>
  <source srcset="{{post.image}}-1200.avif" type="image/avif">
  <source srcset="{{post.image}}-1200.webp" type="image/webp">
  <img src="{{post.image}}-1200.jpg"
       alt="{{post.title}} cover image"
       width="1200" height="800"
       class="w-full h-auto sm:h-80 md:h-96 lg:h-128 object-cover rounded-lg bg-white"
       itemprop="image"
       sizes="(max-width: 640px) 640px,
              (max-width: 768px) 768px,
              (max-width: 1024px) 1024px,
              1200px"
       loading="lazy">
</picture>
```

เบราว์เซอร์เลือกโหลดรูปที่เหมาะสมที่สุด:

- ถ้ารองรับ AVIF → โหลด output-1200.avif
- ถ้ารองรับ WebP → โหลด output-1200.webp
- ถ้าไม่รองรับ WebP/AVIF → โหลด output-1200.jpg

***

### 🎯 Step 3: Test with PageSpeed Insights

หลังจากอัปโหลดภาพไปยังเซิร์ฟเวอร์แล้ว ทดสอบผลลัพธ์ ที่ PageSpeed Insights
- ควรได้ผลลัพธ์ดังนี้:

✔ ขนาดรูปภาพเหมาะสม (Properly Size Images)
✔ WebP & AVIF โหลดก่อน (ลด Bandwidth Usage)
✔ CLS ลดลง → SEO ดีขึ้น
✔ โหลดเร็วขึ้น ~30-50%

***

🔥 Final Thoughts

Image Optimization เป็นสิ่งสำคัญ ที่ช่วยให้เว็บไซต์โหลดเร็วขึ้น ลด Bandwidth และเพิ่มคะแนน SEO

### สรุป

- Resize รูปภาพ ตามอุปกรณ์ที่รองรับ
- Convert เป็น WebP & AVIF เพื่อลดขนาดไฟล์
- ใช้ `<picture>` + srcset ให้เว็บเลือกไฟล์ที่เหมาะสม
- Test บน PageSpeed Insights เพื่อปรับปรุง Performance
