use handlebars::{
    handlebars_helper, Context, DirectorySourceOptions, Handlebars, Helper, HelperResult,
    JsonRender, JsonValue, Output, RenderContext,
};
use std::collections::HashSet;
use std::path::Path;
use walkdir::WalkDir;

handlebars_helper!(limit: |arr: JsonValue, n: u64| {
    let n = n as usize;
    if let Some(array) = arr.as_array() {
        array.iter().take(n).cloned().collect::<Vec<_>>()
    } else {
        Vec::new()
    }
});

handlebars_helper!(hb_month_name_en_helper: |month_num: u64| match month_num {
    1 => "Jan.",
    2 => "Feb.",
    3 => "Mar.",
    4 => "Apr.",
    5 => "May",
    6 => "June",
    7 => "July",
    8 => "Aug.",
    9 => "Sept.",
    10 => "Oct.",
    11 => "Nov.",
    12 => "Dec.",
    _ => "Error!",
});

handlebars_helper!(hb_month_name_th_helper: |month_num: u64| match month_num {
    1 => "มกราคม",
    2 => "กุมภาพันธ์",
    3 => "มีนาคม",
    4 => "เมษายน",
    5 => "พฤษภาคม",
    6 => "มิถุนายน",
    7 => "กรกฎาคม",
    8 => "สิงหาคม",
    9 => "กันยายน",
    10 => "ตุลาคม",
    11 => "พฤศจิกายน",
    12 => "ธันวาคม",
    _ => "ข้อผิดพลาด!",
});

handlebars_helper!(hb_month_short_th_helper: |month_num: u64| match month_num {
    1 => "ม.ค.",
    2 => "ก.พ.",
    3 => "มี.ค.",
    4 => "เม.ย.",
    5 => "พ.ค.",
    6 => "มิ.ย.",
    7 => "ก.ค.",
    8 => "ส.ค.",
    9 => "ก.ย.",
    10 => "ต.ค.",
    11 => "พ.ย.",
    12 => "ธ.ค.",
    _ => "ข้อผิดพลาด!",
});

handlebars_helper!(hb_thai_year_helper: |year: u64| year + 543);

pub fn create_hbs_options(hidden: bool) -> DirectorySourceOptions {
    let mut options = DirectorySourceOptions::default();
    options.tpl_extension = ".html".to_owned();
    options.hidden = hidden;
    options.temporary = false;
    options
}

pub fn register_unique_templates(
    handlebars: &mut Handlebars<'_>,
    directories: &[&str],
    options: &DirectorySourceOptions,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut registered = HashSet::new();

    for dir in directories {
        let root = Path::new(dir);

        for entry in WalkDir::new(root).into_iter().filter_map(Result::ok) {
            let path = entry.path();

            if path.is_file() {
                if let Some(ext) = path.extension() {
                    if ext == std::ffi::OsStr::new(options.tpl_extension.trim_start_matches('.')) {
                        let relative_path = path.strip_prefix(root)?;
                        let mut template_name = relative_path
                            .to_string_lossy()
                            .replace(std::path::MAIN_SEPARATOR, "/"); 

                        if let Some(stripped) = template_name.strip_suffix(&options.tpl_extension) {
                            template_name = stripped.to_string(); 
                        }

                        if !registered.contains(&template_name) {
                            handlebars.register_template_file(&template_name, path)?;
                            registered.insert(template_name.clone());
                            println!("|  ✅ Registered template: {template_name}");
                        } else {
                            println!("|  ⚠️ Skipped duplicate template: {template_name}");
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

pub fn register_all_templates_and_helpers(
    handlebars: &mut Handlebars<'_>,
    directories: &[&str],
    options: &DirectorySourceOptions,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    register_unique_templates(handlebars, directories, options)?;

    handlebars.register_helper("month_name_en", Box::new(hb_month_name_en_helper));
    handlebars.register_helper("month_name_th", Box::new(hb_month_name_th_helper));
    handlebars.register_helper("month_short_th", Box::new(hb_month_short_th_helper));
    handlebars.register_helper("thai_year", Box::new(hb_thai_year_helper));
    handlebars.register_helper("eq", Box::new(eq_helper));
    handlebars.register_helper("limit", Box::new(limit));

    Ok(())
}

pub fn eq_helper(
    h: &Helper<'_>,
    _: &Handlebars,
    _: &Context,
    _: &mut RenderContext<'_, '_>,
    out: &mut dyn Output,
) -> HelperResult {
    #[allow(deprecated)]
    let param0 = h
        .param(0)
        .ok_or_else(|| handlebars::RenderError::new("Missing param 0"))?;
    #[allow(deprecated)]
    let param1 = h
        .param(1)
        .ok_or_else(|| handlebars::RenderError::new("Missing param 1"))?;

    let result = param0.value().render() == param1.value().render();
    out.write(if result { "true" } else { "false" })?;
    Ok(())
}
