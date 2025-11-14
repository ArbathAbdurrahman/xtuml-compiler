use clap::Parser;
use std::path::PathBuf;

mod parser;
mod semantic;
mod generator;

use anyhow::Result;

#[derive(Parser, Debug)]
#[command(author, version, about = "xtUML Compiler (JSON -> code)")]
struct Args {
    /// Path to xtUML model JSON
    #[arg(value_name = "MODEL")]
    model: PathBuf,

    /// Output directory
    #[arg(short, long, default_value = "output/generated_code")]
    out: PathBuf,

    /// Target language (only "python" implemented in sample)
    #[arg(short, long, default_value = "python")]
    lang: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Parse model
    let model = parser::load_model(&args.model)?;

    // Semantic checks
    semantic::validate(&model)?;

    // Ensure output dir exists
    std::fs::create_dir_all(&args.out)?;

    // Generate
    match args.lang.as_str() {
        "python" => generator::python::generate(&model, &args.out)?,
        "javascript" => generator::javascript::generate(&model, &args.out)?,
        "c" => generator::c::generate(&model, &args.out)?,
        "java" => generator::java::generate(&model, &args.out)?,
        "php" => generator::php::generate(&model, &args.out)?,
        "golang" => generator::golang::generate(&model, &args.out)?,
        "swift" => generator::swift::generate(&model, &args.out)?,
        "typescript" => generator::typescript::generate(&model, &args.out)?,
        "rust" => generator::rust::generate(&model, &args.out)?,
        "ruby" => generator::ruby::generate(&model, &args.out)?,
        "csharp" => generator::csharp::generate(&model, &args.out)?,
        other => {
            eprintln!("Language '{}' not supported, only python, javascript, c, java, php, golang, swift, typescript, rust, ruby, csharp. ", other);
        }
    }

    println!("Generation complete. Output in {}", args.out.display());
    Ok(())
}
