use crate::parser::Model;
use tera::{Tera, Context};
use std::path::Path;
use anyhow::{Result, Context as AnyhowContext};
use std::fs;
use include_dir::{include_dir, Dir};

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/templates/java");

pub fn generate(model: &Model, out_dir: &Path) -> Result<()> {
    let mut tera = Tera::default();
    for file in TEMPLATE_DIR.files() {
        if let Some(content) = file.contents_utf8() {
            let name = file.path().file_name().unwrap().to_string_lossy();
            tera.add_raw_template(&name, content)?;
        }
    }

    let mut combined = String::new();
    for cls in &model.classes {
        let mut ctx = Context::new();
        ctx.insert("class", &cls);
        combined.push_str(&tera.render("class.java.tera", &ctx)?);
        combined.push_str("\n\n");
    }

    let out_path = out_dir.join("model.java");
    fs::write(&out_path, combined)
        .with_context(|| format!("Failed to write Java output to {:?}", out_path))?;
    Ok(())
}
