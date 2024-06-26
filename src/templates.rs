use std::fs;

use anyhow::Result;
use minijinja::Environment;

pub fn get_environment() -> Result<Environment<'static>> {
    let base_path = std::env::current_dir().expect("Failed to determine the current directory");
    let templates_directory = base_path.join("templates");

    let mut env = Environment::new();

    for entry in fs::read_dir(templates_directory)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_file() || path.extension() != Some("j2".as_ref()) {
            continue;
        }

        let Some(template_name) = path
            .with_extension("")
            .file_name()
            .and_then(|file| file.to_str())
            .map(|s| s.to_string())
        else {
            continue;
        };

        let template = fs::read_to_string(&path)?;
        env.add_template_owned(template_name, template)?;
    }

    Ok(env)
}
