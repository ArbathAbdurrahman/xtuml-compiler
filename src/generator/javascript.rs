use crate::parser::Model;
use tera::{Tera, Context};
use std::path::Path;
use anyhow::{Result, Context as AnyhowContext};
use std::fs;
use include_dir::{include_dir, Dir};

static TEMPLATE_DIR: Dir = include_dir!("$CARGO_MANIFEST_DIR/src/templates/javascript");

pub fn generate(model: &Model, out_dir: &Path) -> Result<()> {
    let mut tera = Tera::default();
    for file in TEMPLATE_DIR.files() {
        if let Some(content) = file.contents_utf8() {
            let name = file.path().file_name().unwrap().to_string_lossy();
            tera.add_raw_template(&name, content)?;
        }
    }

    let mut combined = String::new();

    let mut ctx_header = Context::new();
    ctx_header.insert("model_name", &model.model_name);
    ctx_header.insert("version", &model.version);
    combined.push_str(&tera.render("header.js.tera", &ctx_header)?);
    combined.push_str("\n");

    for cls in &model.classes {
        let mut ctx = Context::new();
        ctx.insert("class", &cls);

        // ambil event yang trigger-nya == nama class
        let class_events: Vec<_> = model.events
            .iter()
            .filter(|ev| ev.trigger.clone().unwrap_or_default() == cls.name)
            .collect();

        ctx.insert("class_events", &class_events);

        combined.push_str(&tera.render("class.js.tera", &ctx)?);
        combined.push_str("\n\n");
    }
    let mut ctx_events = Context::new();
    ctx_events.insert("events", &model.events);
    combined.push_str(&tera.render("event.js.tera", &ctx_events)?);
    combined.push_str("\n\n");


    combined.push_str(&tera.render("footer.js.tera", &Context::new())?);

    let safe_name: String = model.model_name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();

    let filename = format!("{}_model.js", safe_name);
    let out_path = out_dir.join(filename);
    
    fs::write(&out_path, combined)
        .with_context(|| format!("Failed to write Javascript output to {:?}", out_path))?;
    Ok(())
}
