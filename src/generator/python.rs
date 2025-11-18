use crate::parser::Model;
use tera::{Tera, Context};
use std::path::Path;
use anyhow::{Result, Context as AnyhowContext};
use std::fs;
use include_dir::{include_dir, Dir};

// Embed semua template di src/templates/python
static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/templates/python");

pub fn generate(model: &Model, out_dir: &Path) -> Result<()> {
    // Inisialisasi Tera dari template yang sudah di-embed
    let mut tera = Tera::default();
    for file in TEMPLATE_DIR.files() {
        if let Some(contents) = file.contents_utf8() {
            let name = file.path().file_name().unwrap().to_string_lossy();
            tera.add_raw_template(&name, contents)?;
        }
    }

    let mut combined = String::new();

    // HEADER
    let mut ctx_header = Context::new();
    ctx_header.insert("model_name", &model.model_name);
    ctx_header.insert("version", &model.version);

    combined.push_str(&tera.render("header.py.tera", &ctx_header)?);
    combined.push_str("\n");

    // RENDER CLASS PER CLASS
    for cls in &model.classes {
        let mut ctx = Context::new();
        ctx.insert("class", &cls);
        let rendered = tera.render("class.py.tera", &ctx)?;
        combined.push_str(&rendered);
        combined.push_str("\n\n");
    }

    // FOOTER
    combined.push_str(&tera.render("footer.py.tera", &Context::new())?);


    // Tulis hasil gabungan ke satu file
    let safe_name: String = model.model_name
    .chars()
    .map(|c| if c.is_alphanumeric() { c } else { '_' })
    .collect();

    let filename = format!("{}_model.py", safe_name);
    let out_path = out_dir.join(filename);

    let combined = combined
        .lines()
        .map(|line| {
            let trimmed = line.trim_start();

            if trimmed.starts_with("//") {
                // hitung indentasi
                let indent = line.len() - trimmed.len();
                let indent_spaces = " ".repeat(indent);

                // ubah: // something → # something
                let after = trimmed.trim_start_matches("//").trim_start();
                format!("{}# {}", indent_spaces, after)
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");

    fs::write(&out_path, combined)
        .with_context(|| format!("Failed to write Python output to {:?}", out_path))?;
    Ok(())
}
