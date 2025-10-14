use comrak::{parse_document, Arena, Options, Plugins};
use eyre::WrapErr;
use std::io::BufWriter;

use super::html_custom;

pub fn markdown_to_html(md: &str, options: &Options) -> String {
    markdown_to_html_with_plugins(md, options, &Plugins::default())
}

pub fn markdown_to_html_with_plugins(md: &str, options: &Options, plugins: &Plugins) -> String {
    match render_markdown_bytes(md, options, plugins) {
        Ok(bytes) => match String::from_utf8(bytes) {
            Ok(s) => s,
            Err(_) => String::from("<!-- Invalid UTF-8 content -->"),
        },
        Err(e) => {
            log::error!("markdown_to_html error: {e}");
            String::from("<!-- Error formatting document -->")
        }
    }
}

fn render_markdown_bytes(md: &str, options: &Options, plugins: &Plugins) -> eyre::Result<Vec<u8>> {
    let arena = Arena::new();
    let root = parse_document(&arena, md, options);
    let mut bw = BufWriter::new(Vec::new());

    html_custom::format_document_with_plugins(root, options, &mut bw, plugins)
        .wrap_err("Failed to format document with plugins")?;

    bw.into_inner()
        .map_err(|e| eyre::eyre!("Failed to flush buffer: {}", e))
}
