use anyhow::Result;
use std::path::Path;
use tracing::info;
use std::fs;
use std::io::Write;
use serde_json::json;

use crate::native_types::NativeCollection;

/// Generate VS Code IntelliSense data
pub fn generate_vscode_intellisense(collection: &NativeCollection, output_dir: &Path) -> Result<()> {
    // Ensure output directory exists
    fs::create_dir_all(output_dir)?;
    let mut snippets = serde_json::Map::new();

    for category in collection.categories.values() {
        for func in &category.functions {
            let func_name = &func.name;
            let params = &func.parameters;

            // Build snippet body with placeholders ${index:param}
            let mut body = format!("{}(", func_name);
            for (idx, p) in params.iter().enumerate() {
                if idx > 0 { body.push_str(", "); }
                body.push_str(&format!("${{{}:{}}}", idx + 1, p.name));
            }
            body.push(')');

            // Description includes return type and short doc if available
            let desc = if let Some(d) = &func.description {
                d.clone()
            } else {
                String::new()
            };
            let return_ts = crate::typescript_generator::map_native_type_to_ts(&func.return_type);
            let description_full = if desc.is_empty() {
                format!("Returns {}", return_ts)
            } else {
                format!("{} | Returns {}", desc, return_ts)
            };

            snippets.insert(func_name.clone(), json!({
                "prefix": func_name,
                "body": body,
                "description": description_full,
            }));
        }
    }

    let snippets_json = serde_json::Value::Object(snippets);
    let snippets_path = output_dir.join("natives.code-snippets");
    let mut file = fs::File::create(&snippets_path)?;
    file.write_all(serde_json::to_string_pretty(&snippets_json)?.as_bytes())?;

    info!("✅ VS Code IntelliSense snippets generated: {}", snippets_path.display());
    Ok(())
}

/// Validate generated wrappers
pub fn validate_generated_wrappers(output_dir: &Path) -> Result<()> {
    info!("✅ Validation not implemented yet");
    info!("📂 Would validate wrappers in: {}", output_dir.display());
    Ok(())
}

/// Validate wrappers at specific path
pub fn validate_wrappers_at_path(path: &Path, category: Option<&str>) -> Result<()> {
    info!("🔍 Path validation not implemented yet");
    if let Some(cat) = category {
        info!("📂 Would validate category '{}' at: {}", cat, path.display());
    } else {
        info!("📂 Would validate all wrappers at: {}", path.display());
    }
    Ok(())
}

/// Test wrappers
pub fn test_wrappers(path: &Path, comprehensive: bool) -> Result<()> {
    info!("🧪 Wrapper testing not implemented yet");
    if comprehensive {
        info!("📂 Would run comprehensive tests at: {}", path.display());
    } else {
        info!("📂 Would run basic tests at: {}", path.display());
    }
    Ok(())
} 