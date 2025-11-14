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

    for cls in &model.classes {
        let mut ctx = Context::new();
        ctx.insert("class", &cls);

        // Gunakan nama template sesuai file (misalnya class.py.tera)
        let rendered = tera.render("class.py.tera", &ctx)
            .with_context(|| format!("failed to render template for class {}", cls.name))?;
        combined.push_str(&rendered);
        combined.push_str("\n\n");
    }

    // Tulis hasil gabungan ke satu file
    let out_path = out_dir.join("model.py");
    fs::write(&out_path, combined)
        .with_context(|| format!("Failed to write Python output to {:?}", out_path))?;
    Ok(())
}
