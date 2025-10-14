use image::ImageFormat;
use std::fs::metadata;
use std::path::Path;
use std::process::Command;
use std::fs as stdfs;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;

use crate::shared::command::install_required_tool::install_required_tool;

fn to_public_path(input_image_path: &str) -> Result<String, String> {
    let cleaned_path = input_image_path.trim_start_matches('/');
    if cleaned_path.starts_with("public/") {
        Ok(cleaned_path.to_string())
    } else {
        Path::new("public")
            .join(cleaned_path)
            .to_str()
            .map(|s| s.to_string())
            .ok_or_else(|| "Invalid path encoding".to_string())
    }
}

fn output_base_from_path(path_str: &str) -> Result<String, String> {
    let p = Path::new(path_str);
    let file_name = p
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| "Invalid file name".to_string())?;
    let base = p
        .parent()
        .ok_or_else(|| "Invalid parent directory".to_string())?
        .join(file_name);
    base
        .to_str()
        .map(|s| s.to_string())
        .ok_or_else(|| "Invalid output path encoding".to_string())
}

fn process_image_internal(input_image_path: &str, is_logo: bool) -> Result<String, String> {
    let url_public = to_public_path(input_image_path)?;

    if metadata(&url_public).is_err() {
        println!("│  ⚠️ Image file does not exist: {url_public}");
        println!("│  💡 Creating placeholder path and skipping processing");
        println!("│  📝 Tip: Add image file to continue with optimized images");

        return output_base_from_path(input_image_path);
    }

    let path_metadata =
        metadata(&url_public).map_err(|e| format!("Cannot read file metadata: {e}"))?;

    if !path_metadata.is_file() {
        return Err(format!("Path is not a file: {url_public}"));
    }

    // Check file extension
    let path = Path::new(&url_public);
    let extension = path
        .extension()
        .and_then(|ext| ext.to_str())
        .map(|ext| ext.to_lowercase());

    match extension.as_deref() {
        Some("jpg" | "jpeg" | "png" | "webp" | "gif" | "bmp" | "tiff") => {
            // Valid image extension, continue processing
        }
        Some("svg") => {
            // SVG files don't need resizing, return original path
            println!("│  ⏭️ SVG file detected, skipping resize: {url_public}");
            return output_base_from_path(input_image_path);
        }
        _ => {
            eprintln!("│  ⚠️ File is not a supported image format: {url_public}");
            return Err(format!("Unsupported image format: {url_public}"));
        }
    }

    let output_base_path_str = output_base_from_path(&url_public)?;

    // Determine quality settings. Use the same tuned values as the logo flow
    // so resized JPGs and AVIFs have comparable quality.
    let jpeg_quality: u8 = 85; // same as logo
    let avif_crf: u8 = 30; // same as logo

    // For logos we produce PNG outputs (preserve transparency). For other images use JPG.
    let (output_1024, output_768, output_640) = if is_logo {
        (
            format!("{output_base_path_str}-1024.png"),
            format!("{output_base_path_str}-768.png"),
            format!("{output_base_path_str}-640.png"),
        )
    } else {
        (
            format!("{output_base_path_str}-1024.jpg"),
            format!("{output_base_path_str}-768.jpg"),
            format!("{output_base_path_str}-640.jpg"),
        )
    };

    let output_1024_avif = format!("{output_base_path_str}-1024.avif");
    let output_768_avif = format!("{output_base_path_str}-768.avif");
    let output_640_avif = format!("{output_base_path_str}-640.avif");

    // ขั้นตอนการปรับขนาดภาพ
    if metadata(&output_768).is_err()
        || metadata(&output_640).is_err()
        || metadata(&output_1024).is_err()
    {
        println!("│  🖼️ Resizing images to multiple sizes...");
        println!("│     📏 Creating 1024px version...");
        resize_image(&url_public, &output_1024, 1024, jpeg_quality)?;
        println!("│     📏 Creating 768px version...");
        resize_image(&url_public, &output_768, 768, jpeg_quality)?;
        println!("│     📏 Creating 640px version...");
        resize_image(&url_public, &output_640, 640, jpeg_quality)?;
        println!("│  ✅ Image resizing completed");
    } else {
        // ลด log ซ้ำซาก: print เฉพาะเมื่อ verbose/debug mode หรือ print สรุปท้าย batch แทน
        // println!("│  ⏭️ Resized images already exist, skipping...");
    }

    // ขั้นตอนการแปลงเป็น AVIF
    if metadata(&output_768_avif).is_err()
        || metadata(&output_640_avif).is_err()
        || metadata(&output_1024_avif).is_err()
    {
        println!("│  🔄 Converting to AVIF format...");
        println!("│     🎨 Converting 1024px to AVIF...");
        convert_to_avif(&output_1024, &output_1024_avif, avif_crf)?;
        println!("│     🎨 Converting 768px to AVIF...");
        convert_to_avif(&output_768, &output_768_avif, avif_crf)?;
        println!("│     🎨 Converting 640px to AVIF...");
        convert_to_avif(&output_640, &output_640_avif, avif_crf)?;
        println!("│  ✅ AVIF conversion completed");
    } else {
        // ลด log ซ้ำซาก: print เฉพาะเมื่อ verbose/debug mode หรือ print สรุปท้าย batch แทน
        // println!("│  ⏭️ AVIF images already exist, skipping...");
    }

    let input_path2 = Path::new(&input_image_path);
    let file_name2 = input_path2
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(|| "Invalid file name in input path".to_string())?;
    let output_base_path2 = input_path2
        .parent()
        .ok_or_else(|| "Invalid parent directory in input path".to_string())?
        .join(file_name2);

    let output_base_path_str2 = output_base_path2
        .to_str()
        .ok_or_else(|| "Invalid output path encoding".to_string())?;

    // If this is a logo run, also generate favicons
    if is_logo {
        println!("│  ℹ️ Detected logo file - generating favicons...");
        if let Err(e) = generate_favicons(input_image_path) {
            eprintln!("│  ⚠️ Failed to generate favicons: {e}");
            // do not treat favicon failure as fatal for image processing; continue
        } else {
            println!("│  ✅ Favicons generated");
        }
    }

    Ok(output_base_path_str2.to_owned())
}

pub fn process_image(input_image_path: &str) -> Result<String, String> {
    process_image_internal(input_image_path, false)
}

pub fn process_logo(input_image_path: &str) -> Result<String, String> {
    // For the `logo` command we only generate favicons (do not run full resize pipeline).
    match generate_favicons(input_image_path) {
        Ok(()) => output_base_from_path(input_image_path),
        Err(e) => Err(e),
    }
}

fn check_command(command: &str) -> bool {
    // เรียกคำสั่งและตรวจสอบเวอร์ชัน
    let output = Command::new(command).arg("--version").output();

    // หากเกิดข้อผิดพลาดหรือไม่พบคำสั่งในระบบให้คืนค่า false
    match output {
        Ok(output) => output.status.success(), // ถ้าคำสั่งสำเร็จ, return true
        Err(_) => false,                       // ถ้าไม่พบคำสั่ง หรือเกิดข้อผิดพลาด, return false
    }
}

// Ensure a required CLI tool is installed (best-effort). Returns Ok(()) if the tool is available or was installed.
fn ensure_tool_installed(tool: &str) -> Result<(), String> {
    if check_command(tool) {
        return Ok(());
    }

    println!("│  ⚠️ {tool} is not installed. Installing...");
    install_required_tool(tool).map_err(|e| format!("Failed to install {tool}: {e}"))?;
    println!("│  ✅ {tool} installed successfully.");
    Ok(())
}

fn resize_image(
    input_path: &str,
    output_path: &str,
    width: u32,
    quality: u8,
) -> Result<(), String> {
    // Ensure output directory exists
    if let Some(parent) = Path::new(output_path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }

    ensure_tool_installed("magick")?;

    // Build ImageMagick command. For PNG outputs (logos) we must preserve alpha and don't set JPEG quality.
    let mut magick_cmd = Command::new("magick");
    magick_cmd.arg(input_path).arg("-resize").arg(format!("{width}x")).arg("-strip");
    if output_path.to_lowercase().ends_with(".png") {
        // Preserve alpha channel: do not set -quality
        magick_cmd.arg(output_path);
    } else {
        // Only flatten onto white if the input is likely to have an alpha
        // channel (common for .png and .webp). This avoids changing the
        // background for inputs that don't have transparency.
        let input_lc = input_path.to_lowercase();
        if input_lc.ends_with(".png") || input_lc.ends_with(".webp") {
            magick_cmd
                .arg("-background")
                .arg("white")
                .arg("-alpha")
                .arg("remove")
                .arg("-flatten");
        }

        magick_cmd
            .arg("-quality")
            .arg(quality.to_string())
            .arg(output_path);
    }

    let output = magick_cmd
        .output()
        .map_err(|e| format!("Error running ImageMagick: {e}"))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        eprintln!("│  ❌ ImageMagick error: {error_msg}");
        return Err(format!("ImageMagick failed: {error_msg}"));
    }

    println!("│    ✅ Created {output_path} ({width}px width)");
    Ok(())
}

fn convert_to_avif(input_path: &str, output_path: &str, crf: u8) -> Result<(), String> {
    // Ensure output directory exists
    if let Some(parent) = Path::new(output_path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }

    ensure_tool_installed("ffmpeg")?;

    // Build ffmpeg command. If input is PNG, request an AVIF pixel format that supports alpha.
    let mut ffmpeg_cmd = Command::new("ffmpeg");
    ffmpeg_cmd
        .arg("-y")
        .arg("-i")
        .arg(input_path)
        .arg("-c:v")
        .arg("libaom-av1")
        .arg("-crf")
        .arg(crf.to_string())
        .arg("-b:v")
        .arg("0")
        .arg("-movflags")
        .arg("+faststart");

    if input_path.to_lowercase().ends_with(".png") || input_path.to_lowercase().ends_with(".webp") {
        // webp/png may have alpha; use yuva420p pixel format to preserve alpha in AVIF
        ffmpeg_cmd.arg("-pix_fmt").arg("yuva420p");
    }

    ffmpeg_cmd.arg(output_path);

    let output = ffmpeg_cmd
        .output()
        .map_err(|e| format!("Error running FFmpeg: {e}"))?;

    if !output.status.success() {
        let error_msg = String::from_utf8_lossy(&output.stderr);
        eprintln!("│  ❌ FFmpeg error: {error_msg}");
        return Err(format!("FFmpeg failed: {error_msg}"));
    }

    println!("│    ✅ Created AVIF: {output_path}");
    Ok(())
}

fn execute_command(command: &str, args: &[&str], error_msg: &str) -> Result<(), String> {
    let status = Command::new(command)
        .args(args)
        .status()
        .map_err(|e| format!("❌ Error running {command}: {e}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("❌ Failed to {error_msg}: {command}"))
    }
}

pub fn generate_favicons(input_path: &str) -> Result<(), String> {
    let url_public = to_public_path(input_path)?;

    if metadata(&url_public).is_err() {
        return Err(format!("│  ⚠️ Image file does not exist: {url_public}"));
    }

    // Place favicons in the same directory as the source image (original behavior)
    let input_path_fs = Path::new(&url_public);
    let output_dir = input_path_fs
        .parent()
        .ok_or_else(|| format!("Invalid parent directory for input path: {url_public}"))?;

    // Ensure output directory exists
    std::fs::create_dir_all(output_dir)
        .map_err(|e| format!("Failed to create favicon directory: {e}"))?;

    // Convert webp to png if needed
    let working_image_path = if url_public.to_lowercase().ends_with(".webp") {
        let temp_png_path = format!("{}/temp_favicon_input.png", output_dir.display());
        convert_webp_to_png(&url_public, &temp_png_path)?;
        temp_png_path
    } else {
        url_public.clone()
    };

    let sizes = [
        (192, "android-chrome-192x192.png"),
        (512, "android-chrome-512x512.png"),
        (180, "apple-touch-icon.png"),
        (16, "favicon-16x16.png"),
        (32, "favicon-32x32.png"),
    ];

    // Ensure ImageMagick is available before creating PNGs and ICO
    ensure_tool_installed("magick")?;

    // Generate PNG files using ImageMagick
    for (size, filename) in &sizes {
        let output_path = output_dir.join(filename);
        execute_command(
            "magick",
            &[
                &working_image_path,
                "-resize",
                &format!("{size}x{size}"),
                output_path
                    .to_str()
                    .ok_or_else(|| format!("Invalid path for {filename}"))?,
            ],
            &format!("create {filename}"),
        )?;
        println!("│  ✅ Created {filename} successfully");
    }

    // Generate favicon.ico
    let favicon_ico_path = output_dir.join("favicon.ico");
    execute_command(
        "magick",
        &[
            &working_image_path,
            "-resize",
            "256x256",
            "-background",
            "none",
            "-gravity",
            "center",
            "-extent",
            "256x256",
            "-colors",
            "256",
            favicon_ico_path
                .to_str()
                .ok_or_else(|| "Invalid favicon.ico path".to_string())?,
        ],
        "create favicon.ico",
    )?;
    println!("│  ✅ Created favicon.ico successfully");

    // Generate SVG files using Inkscape
    let svg_files = [
        ("favicon.svg", "create favicon.svg"),
        ("mask-icon.svg", "create mask-icon.svg"),
    ];
    // Generate SVG files using Inkscape only if available; do not attempt to auto-install.
    if check_command("inkscape") {
        for (filename, error_msg) in svg_files.iter() {
            let svg_path = output_dir.join(filename);
            match Command::new("inkscape")
                .args([
                    &working_image_path,
                    "--export-type=svg",
                    "--export-filename",
                    svg_path
                        .to_str()
                        .ok_or_else(|| format!("Invalid SVG path for {filename}"))?,
                    "--export-width=64",
                    "--export-height=64",
                ])
                .status()
            {
                Ok(status) => {
                    if !status.success() {
                        println!("│  ⚠️ Warning: Failed to {error_msg} for {filename}");
                    } else {
                        println!("│  ✅ Created {filename} successfully (64x64)");
                    }
                }
                Err(e) => {
                    println!("│  ⚠️ Error running inkscape for {filename}: {e}");
                }
            }
        }
    } else {
        println!("│  ⚠️ Inkscape not found — falling back to embedded PNG SVGs");

        // Create a temporary 64x64 PNG using ImageMagick and embed as base64 in SVG files
        let tmp_png_path = output_dir.join("favicon-64-temp.png");
        let tmp_png_str = tmp_png_path
            .to_str()
            .ok_or_else(|| "Invalid temp png path encoding".to_string())?;

        // Create 64x64 PNG
        execute_command(
            "magick",
            &[
                &working_image_path,
                "-resize",
                "64x64",
                tmp_png_str,
            ],
            "create temporary 64x64 png",
        )?;

        // Read and base64 encode
        let png_bytes = stdfs::read(tmp_png_str)
            .map_err(|e| format!("Failed to read temporary PNG for SVG embedding: {e}"))?;
        let b64 = STANDARD.encode(&png_bytes);

        let svg_content = format!(
            "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"64\" height=\"64\" viewBox=\"0 0 64 64\">\n  <image href=\"data:image/png;base64,{b64}\" width=\"64\" height=\"64\"/>\n</svg>"
        );

        let favicon_svg_path = output_dir.join("favicon.svg");
        stdfs::write(&favicon_svg_path, svg_content.as_bytes())
            .map_err(|e| format!("Failed to write favicon.svg: {e}"))?;
        println!("│  ✅ Created favicon.svg (embedded PNG)");

        // mask-icon.svg - use same embedded PNG (consumers expect mask-icon.svg presence)
        let mask_svg_path = output_dir.join("mask-icon.svg");
        stdfs::write(&mask_svg_path, stdfs::read(&favicon_svg_path).map_err(|e| format!("Failed to read generated favicon.svg for mask-icon: {e}"))?)
            .map_err(|e| format!("Failed to write mask-icon.svg: {e}"))?;
        println!("│  ✅ Created mask-icon.svg (embedded PNG)");

        // Clean up temporary PNG
        let _ = stdfs::remove_file(tmp_png_str);
    }

    // Clean up temporary file if created
    if working_image_path != url_public {
        let _ = std::fs::remove_file(&working_image_path);
    }

    Ok(())
}

fn convert_webp_to_png(webp_path: &str, png_path: &str) -> Result<(), String> {
    // Create output directory if it doesn't exist
    if let Some(parent) = Path::new(png_path).parent() {
        std::fs::create_dir_all(parent)
            .map_err(|e| format!("Failed to create output directory: {e}"))?;
    }

    // Load webp image
    let img = image::open(webp_path).map_err(|e| format!("Failed to open webp image: {e}"))?;

    // Save as png
    img.save_with_format(png_path, ImageFormat::Png)
        .map_err(|e| format!("Failed to save as png: {e}"))?;

    println!("│  🔄 Converted webp to png for favicon generation");
    Ok(())
}
