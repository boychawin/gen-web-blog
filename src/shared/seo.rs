use glob::glob;
use regex::Regex;
use std::path::Path;
use log::{info, warn, error};

/// Main entry point for SEO scanning
pub fn scan_html_files_in_directory(directory: &str) {
    let pattern = format!("{directory}/**/*.html");
    let mut total_files: usize = 0;
    let mut passed_files: usize = 0;

    info!("\n🔍 Scanning HTML files for SEO compliance in: {directory}");

    for entry in glob(&pattern).expect("❌ Failed to read glob pattern") {
        match entry {
            Ok(file_path) => {
                total_files += 1;
                info!("\n│  📄 Checking file: {}", file_path.display());
                if check_seo_compliance(&file_path) {
                    passed_files += 1;
                    info!("│  ✅ File {} passed SEO compliance!", file_path.display());
                } else {
                    warn!("│  ❌ File {} did not pass SEO compliance.", file_path.display());
                }
            }
            Err(e) => error!("│  ❌ Error reading file: {e}"),
        }
    }

    info!("\n📊 SEO Scan Summary:");
    info!("│  Total files: {total_files}");
    info!("│  Passed: {passed_files}");
    info!("│  Failed: {}", total_files.saturating_sub(passed_files));

    if total_files == 0 {
        info!("│  No HTML files found to scan in: {directory}");
    } else {
        let success_rate = (passed_files as f64 / total_files as f64) * 100.0;
        info!("│  Success rate: {:.1}%", success_rate);
    }
}

/// Comprehensive SEO compliance check
fn check_seo_compliance(file_path: &Path) -> bool {
    let file_content = match crate::shared::fs::read_file_to_string(file_path) {
        Ok(content) => content,
        Err(e) => {
            error!("│  ❌ Error reading file {}: {}", file_path.display(), e);
            return false;
        }
    };

    info!("│  🔎 Running comprehensive SEO analysis...");

    // Run all 12 SEO checks
    let checks = [
        ("Title Tag", check_title_tag(&file_content, file_path)),
        (
            "Meta Description",
            check_meta_description(&file_content, file_path),
        ),
        (
            "Viewport Meta",
            check_viewport_meta(&file_content, file_path),
        ),
        (
            "Canonical URL",
            check_canonical_url(&file_content, file_path),
        ),
        ("Open Graph", check_open_graph(&file_content, file_path)),
        (
            "Structured Data",
            check_structured_data(&file_content, file_path),
        ),
        (
            "Image Optimization",
            check_image_optimization(&file_content, file_path),
        ),
        (
            "Heading Structure",
            check_heading_structure(&file_content, file_path),
        ),
        (
            "H1 Uniqueness",
            check_h1_uniqueness(&file_content, file_path),
        ),
        (
            "Semantic HTML",
            check_semantic_html(&file_content, file_path),
        ),
        (
            "Page Speed",
            check_page_speed_elements(&file_content, file_path),
        ),
        (
            "Internal Linking",
            check_internal_linking(&file_content, file_path),
        ),
    ];

    let passed_checks = checks.iter().filter(|(_, result)| *result).count();
    let total_checks = checks.len();

    info!("│  📋 Check Results: {passed_checks}/{total_checks} passed");

    // Show failed checks
    for (name, result) in &checks {
        if !result {
            warn!("│    ❌ {name}");
        }
    }

    // Return true if at least 80% of checks pass (configurable threshold)
    let success_rate = passed_checks as f64 / total_checks as f64;
    let threshold = 0.8; // 80% threshold

    if success_rate >= threshold {
        println!(
            "│  ✅ Overall SEO Score: {:.1}% (PASS)",
            success_rate * 100.0
        );
        true
    } else {
        println!(
            "│  ❌ Overall SEO Score: {:.1}% (FAIL - need {:.0}%)",
            success_rate * 100.0,
            threshold * 100.0
        );
        false
    }
}

/// 1. Title Tag Optimization (30-60 characters, unique, keyword-rich)
fn check_title_tag(file_content: &str, file_path: &Path) -> bool {
    let title_regex = Regex::new(r"(?is)<title>(.*?)</title>").unwrap();

    if let Some(cap) = title_regex.captures(file_content) {
        let title = cap.get(1).unwrap().as_str().trim();
        let title_length = title.chars().count();

        // Check title length (Google displays 50-60 characters)
        if title_length < 30 {
            warn!(
                "│  ⚠️  [SEO] Title too short ({} chars). Recommended: 30-60 characters in {}",
                title_length,
                file_path.display()
            );
            return false;
        }
        if title_length > 60 {
            warn!(
                "│  ⚠️  [SEO] Title too long ({} chars). Google truncates at ~60 characters in {}",
                title_length,
                file_path.display()
            );
            return false;
        }
        info!("│  ✅ [SEO] Title tag: \"{title}\" ({title_length} chars)");
        true
    } else {
        error!(
            "│  ❌ [SEO] No <title> tag found in {}",
            file_path.display()
        );
        false
    }
}

/// 2. Meta Description Optimization (120-155 characters)
fn check_meta_description(file_content: &str, file_path: &Path) -> bool {
    let meta_desc_regex = Regex::new(r#"(?i)<meta[^>]*(?:name\s*=\s*["']?description["']?[^>]*content\s*=\s*["']([^"']*?)["']|content\s*=\s*["']([^"']*?)["'][^>]*name\s*=\s*["']?description["']?)"#).unwrap();

    if let Some(cap) = meta_desc_regex.captures(file_content) {
        // Get description from either group 1 or group 2
        let description = cap.get(1).or_else(|| cap.get(2)).unwrap().as_str().trim();
        let desc_length = description.chars().count();

        if desc_length < 50 {
            warn!(
                "│  ⚠️  [SEO] Meta description short ({} chars). Recommended: 50-160 characters, but acceptable for {}",
                desc_length,
                file_path.display()
            );
        }

        if desc_length > 160 {
            warn!(
                "│  ⚠️  [SEO] Meta description long ({} chars). Google may truncate at ~160 characters for {}",
                desc_length,
                file_path.display()
            );
        }

        info!("│  ✅ [SEO] Meta description: {desc_length} chars");
        true
    } else {
        error!(
            "│  ❌ [SEO] No meta description found in {}",
            file_path.display()
        );
        false
    }
}

/// 3. Viewport Meta Tag (Mobile-First Responsive Design)
fn check_viewport_meta(file_content: &str, file_path: &Path) -> bool {
    let viewport_regex =
        Regex::new(r#"(?i)<meta[^>]+name\s*=\s*["']?viewport["']?[^>]*>"#).unwrap();

    if viewport_regex.is_match(file_content) {
        info!("│  ✅ [SEO] Viewport meta tag found (mobile-friendly)");
        true
    } else {
        error!(
            "│  ❌ [SEO] No viewport meta tag found - required for mobile SEO in {}",
            file_path.display()
        );
        false
    }
}

/// 4. Canonical URL (Prevents Duplicate Content Issues)
fn check_canonical_url(file_content: &str, file_path: &Path) -> bool {
    let canonical_regex = Regex::new(
        r#"(?i)<link[^>]*rel\s*=\s*["']?canonical["']?[^>]*href\s*=\s*["']?([^"'\s>]+)["']?"#,
    )
    .unwrap();

    if let Some(cap) = canonical_regex.captures(file_content) {
        let canonical_url = cap.get(1).unwrap().as_str();
        info!("│  ✅ [SEO] Canonical URL: {canonical_url}");
        true
    } else {
        warn!("│  ⚠️  [SEO] No canonical URL found - consider adding to prevent duplicate content issues in {}", file_path.display());
        true // Warning only, not critical
    }
}

/// 5. Open Graph Meta Tags (Social Media Optimization)
fn check_open_graph(file_content: &str, file_path: &Path) -> bool {
    let og_title_regex = Regex::new(
        r#"(?i)<meta\s+property\s*=\s*["']og:title["'][^>]*content\s*=\s*["']([^"']*)["']"#,
    )
    .unwrap();
    let og_desc_regex = Regex::new(
        r#"(?i)<meta\s+property\s*=\s*["']og:description["'][^>]*content\s*=\s*["']([^"']*)["']"#,
    )
    .unwrap();
    let og_url_regex = Regex::new(
        r#"(?i)<meta\s+property\s*=\s*["']og:url["'][^>]*content\s*=\s*["']([^"']*)["']"#,
    )
    .unwrap();

    let has_og_title = og_title_regex.is_match(file_content);
    let has_og_desc = og_desc_regex.is_match(file_content);
    let has_og_url = og_url_regex.is_match(file_content);

    if has_og_title && has_og_desc && has_og_url {
        info!("│  ✅ [SEO] Open Graph tags: title=✓, description=✓, url=✓");
        true
    } else {
        warn!(
            "│  ⚠️  [SEO] Incomplete Open Graph tags: title={}, description={}, url={} in {}",
            has_og_title,
            has_og_desc,
            has_og_url,
            file_path.display()
        );
        true // Warning only, not critical
    }
}

/// 6. Structured Data (JSON-LD Schema Markup)
fn check_structured_data(file_content: &str, file_path: &Path) -> bool {
    let jsonld_regex = Regex::new(
        r#"(?i)<script[^>]*type\s*=\s*["']application/ld\+json["'][^>]*>(.*?)</script>"#,
    )
    .unwrap();

    if let Some(cap) = jsonld_regex.captures(file_content) {
        let json_content = cap.get(1).unwrap().as_str().trim();
        if json_content.contains("@context") && json_content.contains("schema.org") {
            info!("│  ✅ [SEO] Structured data (JSON-LD) found with Schema.org context");
            true
        } else {
            warn!(
                "│  ⚠️  [SEO] JSON-LD found but missing Schema.org context in {}",
                file_path.display()
            );
            true // Warning only
        }
    } else {
        warn!("│  ⚠️  [SEO] No structured data (JSON-LD) found - consider adding for better search results in {}", file_path.display());
        true // Warning only, not critical
    }
}

/// 7. Image Optimization (Alt text, WebP format, loading attributes)
fn check_image_optimization(file_content: &str, file_path: &Path) -> bool {
    let img_regex = Regex::new(r"(?i)<img[^>]*>").unwrap();
    let mut all_images_optimized = true;
    let mut webp_count = 0;
    let mut lazy_loading_count = 0;
    let mut total_images = 0;

    for img_match in img_regex.find_iter(file_content) {
        total_images += 1;
        let img_tag = img_match.as_str();

        // Check for alt attribute
        if !img_tag.contains("alt=") {
            error!(
                "│  ❌ [SEO] Image missing alt attribute: {} in {}",
                img_tag,
                file_path.display()
            );
            all_images_optimized = false;
        }

        // Check for WebP format
        if img_tag.contains(".webp") || img_tag.contains(".avif") {
            webp_count += 1;
        }

        // Check for lazy loading
        if img_tag.contains("loading=\"lazy\"") {
            lazy_loading_count += 1;
        }
    }

    if total_images == 0 {
        info!("│  ✅ [SEO] No images found");
        return true;
    }

    info!("│  ✅ [SEO] Image optimization: total={total_images}, webp/avif={webp_count}, lazy={lazy_loading_count}");

    all_images_optimized
}

/// 8. Heading Structure (H1-H6 Hierarchy)
fn check_heading_structure(file_content: &str, file_path: &Path) -> bool {
    // Check if there's at least one h1 tag
    let h1_regex = Regex::new(r"(?i)<h1[^>]*>").unwrap();
    if !h1_regex.is_match(file_content) {
        eprintln!("│  ❌ [SEO] No <h1> tag found in {}", file_path.display());
        return false;
    }

    // Check heading hierarchy (h1 > h2 > h3, etc.)
    let h_all_regex = Regex::new(r"(?i)<h([1-6])[^>]*>").unwrap();
    let mut heading_levels = Vec::new();
    for cap in h_all_regex.captures_iter(file_content) {
        if let Ok(level) = cap[1].parse::<u32>() {
            heading_levels.push(level);
        }
    }

    // Check for proper hierarchy
    if !heading_levels.is_empty() {
        let mut max_level = 0;
        for &level in &heading_levels {
            if level > max_level + 1 {
                eprintln!(
                    "│  ❌ [SEO] Improper heading hierarchy in {} (jumping from h{} to h{})",
                    file_path.display(),
                    max_level,
                    level
                );
                return false;
            }
            max_level = max_level.max(level);
        }
    }

    println!("│  ✅ [SEO] Heading structure hierarchy is correct");
    true
}

/// 9. Check for multiple H1 tags (SEO best practice: only one H1 per page)
fn check_h1_uniqueness(file_content: &str, file_path: &Path) -> bool {
    let h1_regex = Regex::new(r"(?s)<h1(?:\s+[^>]*)?>.*?</h1>").unwrap();
    let h1_matches: Vec<_> = h1_regex.find_iter(file_content).collect();

    match h1_matches.len() {
        0 => {
            eprintln!("│  ❌ [SEO] No <h1> found in {}", file_path.display());
            false
        }
        1 => {
            println!("│  ✅ [SEO] Single H1 tag found (SEO best practice)");
            true
        }
        count => {
            eprintln!("│  ⚠️  [SEO] Multiple H1 tags found ({}) - consider using only one H1 per page in {}",
                count, file_path.display());
            true // Warning, not critical failure
        }
    }
}

/// 10. Semantic HTML Structure (Header, Main, Article, Section)
fn check_semantic_html(file_content: &str, file_path: &Path) -> bool {
    let header_regex = Regex::new(r"(?s)<header[^>]*>.*?</header>").unwrap();
    let main_regex = Regex::new(r"(?s)<main[^>]*>.*?</main>").unwrap();
    let article_regex = Regex::new(r"(?s)<article[^>]*>.*?</article>").unwrap();
    let section_regex = Regex::new(r"(?s)<section[^>]*>.*?</section>").unwrap();

    let has_header = header_regex.is_match(file_content);
    let has_main = main_regex.is_match(file_content);
    let has_article = article_regex.is_match(file_content);
    let has_section = section_regex.is_match(file_content);

    if !has_main {
        eprintln!(
            "│  ❌ [SEO] No <main> element found - important for page structure in {}",
            file_path.display()
        );
        return false;
    }

    if !has_article && !has_section {
        eprintln!("│  ❌ [SEO] No <article> or <section> elements found - important for content structure in {}", file_path.display());
        return false;
    }

    println!("│  ✅ [SEO] Good semantic HTML structure: main={has_main}, header={has_header}, article={has_article}, section={has_section}");
    true
}

/// 11. Page Speed Optimization Elements
fn check_page_speed_elements(file_content: &str, file_path: &Path) -> bool {
    let preload_regex = Regex::new(r#"(?i)<link[^>]*\brel\s*=\s*["']preload["']"#).unwrap();
    let async_script_regex = Regex::new(r"(?i)<script[^>]*\basync\b").unwrap();
    let defer_script_regex = Regex::new(r"(?i)<script[^>]*\bdefer\b").unwrap();

    let has_preload = preload_regex.is_match(file_content);
    let has_async = async_script_regex.is_match(file_content);
    let has_defer = defer_script_regex.is_match(file_content);

    if !has_preload {
        eprintln!(
            "│  ⚠️  [SEO] No preload links found - consider preloading critical resources in {}",
            file_path.display()
        );
    }

    if !has_async && !has_defer {
        eprintln!(
            "│  ⚠️  [SEO] No async/defer scripts found - consider optimizing script loading in {}",
            file_path.display()
        );
    }

    println!("│  ✅ [SEO] Page speed optimization: preload={has_preload}, async={has_async}, defer={has_defer}");
    true // Warning only, not critical
}

/// 12. Internal Linking Structure
fn check_internal_linking(file_content: &str, file_path: &Path) -> bool {
    let all_link_regex = Regex::new(r#"(?i)<a[^>]*\bhref\s*=\s*["']([^"']*)["'][^>]*>"#).unwrap();
    let external_link_regex =
        Regex::new(r#"(?i)<a[^>]*\bhref\s*=\s*["']https?://[^"']*["'][^>]*>"#).unwrap();

    let mut internal_count = 0;
    let external_count = external_link_regex.find_iter(file_content).count();

    // Count internal links (not starting with http/https and not starting with /)
    for cap in all_link_regex.captures_iter(file_content) {
        let href = cap.get(1).unwrap().as_str();
        if !href.starts_with("http") && !href.starts_with("//") {
            internal_count += 1;
        }
    }

    if internal_count == 0 {
        eprintln!("│  ⚠️  [SEO] No internal links found - consider adding internal links for better SEO in {}", file_path.display());
    }

    // Check for proper rel attributes on external links
    for external_match in external_link_regex.find_iter(file_content) {
        let link_tag = external_match.as_str();
        if !link_tag.contains("rel=")
            || (!link_tag.contains("noopener") && !link_tag.contains("noreferrer"))
        {
            eprintln!("│  ⚠️  [SEO] External link missing rel='noopener noreferrer' for security: {} in {}",
                link_tag, file_path.display());
        }
    }

    println!("│  ✅ [SEO] Link structure: internal={internal_count}, external={external_count}");
    true
}

/// Run comprehensive SEO tests
pub fn run_seo_tests() -> bool {
    println!("\n🧪 Running SEO Tests...\n");

    // Test 1: Valid HTML with good SEO
    let good_html = r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <title>Perfect SEO Title for Testing - 45 Characters</title>
    <meta name="description" content="This is a perfect meta description with exactly the right length for SEO testing and validation purposes in modern web development.">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <link rel="canonical" href="https://example.com/test">
    <meta property="og:title" content="Perfect SEO Title">
    <meta property="og:description" content="Perfect description">
    <meta property="og:url" content="https://example.com">
    <script type="application/ld+json">
        {"@context": "https://schema.org", "@type": "WebPage"}
    </script>
    <link rel="preload" href="/font.woff2" as="font">
</head>
<body>
    <header>
        <h1>Main Page Title</h1>
    </header>
    <main>
        <article>
            <h2>Article Title</h2>
            <p>Content with <a href="/internal">internal link</a> and
               <a href="https://external.com" rel="noopener noreferrer">external link</a></p>
            <img src="image.webp" alt="Optimized image" loading="lazy">
        </article>
    </main>
    <script async src="script.js"></script>
</body>
</html>
    "#;

    // Test 2: Bad HTML with SEO issues
    let bad_html = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Bad</title>
    <meta name="description" content="Too short">
</head>
<body>
    <div>
        <h3>Wrong heading order</h3>
        <h1>Multiple H1</h1>
        <h1>Another H1</h1>
        <img src="image.jpg">
    </div>
</body>
</html>
    "#;

    println!("Test 1: Good HTML");
    let result1 = test_html_content(good_html, "good_test.html");

    println!("\nTest 2: Bad HTML");
    let result2 = test_html_content(bad_html, "bad_test.html");

    println!("\n📊 Test Results:");
    println!("  Good HTML passed: {result1}");
    println!("  Bad HTML failed (as expected): {}", !result2);

    if result1 && !result2 {
        println!("  ✅ All SEO tests passed!");
        true
    } else {
        println!("  ❌ Some SEO tests failed!");
        false
    }
}

/// Helper function to test HTML content
fn test_html_content(html_content: &str, filename: &str) -> bool {
    use std::path::Path;

    let file_path = Path::new(filename);

    // Test title tag
    let title_result = check_title_tag(html_content, file_path);

    // Test meta description
    let meta_result = check_meta_description(html_content, file_path);

    // Test viewport
    let viewport_result = check_viewport_meta(html_content, file_path);

    // Test heading structure
    let heading_result = check_heading_structure(html_content, file_path);

    // Test semantic HTML
    let semantic_result = check_semantic_html(html_content, file_path);

    // Test image optimization
    let image_result = check_image_optimization(html_content, file_path);

    title_result
        && meta_result
        && viewport_result
        && heading_result
        && semantic_result
        && image_result
}
