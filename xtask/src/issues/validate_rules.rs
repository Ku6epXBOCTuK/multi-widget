use serde_json::Value;
use std::collections::HashMap;

use crate::common::{GREEN, RESET, YELLOW};
use crate::{
    error::{AppError, AppResult},
    issues::{
        lint_md::FileContent,
        parse_frontmatter_json::{FieldKind, FieldSpec, ValidationRules},
    },
};

pub(super) fn validate_file(file: FileContent, rules: &ValidationRules) -> AppResult<()> {
    let fm_block = extract_frontmatter(&file)?;

    let yaml: HashMap<String, Value> =
        serde_yaml::from_str(&fm_block).map_err(|source| AppError::YamlParse {
            file: file.filename.clone(),
            source,
        })?;

    for (field_name, spec) in &rules.fields {
        match yaml.get(field_name) {
            None if spec.required => {
                return Err(AppError::MissingRequiredField {
                    file: file.filename.clone(),
                    field: field_name.clone(),
                });
            }
            None => {}
            Some(val) => {
                validate_field(
                    &file.filename,
                    field_name,
                    val,
                    spec,
                    &rules.allowed_categories,
                )?;
            }
        }
    }

    for key in yaml.keys() {
        if !rules.fields.contains_key(key) {
            eprintln!(
                "  {YELLOW}⚠ {}: unknown field '{key}' (not in frontmatter.json){RESET}",
                file.filename
            );
        }
    }

    println!("  {GREEN}✓ {}{RESET}", file.filename);
    Ok(())
}

fn extract_frontmatter(file: &FileContent) -> AppResult<String> {
    let trimmed = file.content.trim_start();
    if !trimmed.starts_with("---") {
        return Err(AppError::MissingFrontmatter {
            file: file.filename.clone(),
        });
    }
    let after_first = &trimmed[3..];
    let end = after_first
        .find("---")
        .ok_or(AppError::MissingFrontmatter {
            file: file.filename.clone(),
        })?;
    Ok(after_first[..end].to_string())
}

fn validate_field(
    file: &str,
    field_name: &str,
    val: &Value,
    spec: &FieldSpec,
    allowed_categories: &[String],
) -> AppResult<()> {
    match &spec.kind {
        FieldKind::String => {
            if !spec.required {
                return Ok(());
            }
            if val.as_str().is_none_or(|s| s.trim().is_empty()) {
                return Err(AppError::EmptyRequiredString {
                    file: file.to_string(),
                    field: field_name.to_string(),
                });
            }
        }
        FieldKind::Choice(options) => {
            if let Some(s) = val.as_str() {
                if !options.iter().any(|o| o == s) {
                    return Err(AppError::InvalidChoice {
                        file: file.to_string(),
                        field: field_name.to_string(),
                        value: s.to_string(),
                        allowed: options.join(", "),
                    });
                }
            }
        }
        FieldKind::Categories => {
            if let Some(arr) = val.as_array() {
                if arr.is_empty() {
                    return Err(AppError::MissingRequiredField {
                        file: file.to_string(),
                        field: field_name.to_string(),
                    });
                }
                if arr.len() > 1 {
                    eprintln!(
                        "  {YELLOW}⚠ {file}: field '{field_name}' has {count} categories (recommended: 1){RESET}",
                        count = arr.len()
                    );
                }
                for item in arr {
                    if let Some(s) = item.as_str() {
                        if !allowed_categories.contains(&s.to_string()) {
                            return Err(AppError::InvalidCategory {
                                file: file.to_string(),
                                field: field_name.to_string(),
                                value: s.to_string(),
                                allowed: allowed_categories.join(", "),
                            });
                        }
                    }
                }
            } else {
                return Err(AppError::NotArray {
                    file: file.to_string(),
                    field: field_name.to_string(),
                });
            }
        }
        FieldKind::Datetime => {
            if let Some(s) = val.as_str() {
                if !s.contains('T') || !s.ends_with('Z') {
                    return Err(AppError::InvalidDatetime {
                        file: file.to_string(),
                        field: field_name.to_string(),
                        value: s.to_string(),
                    });
                }
            } else {
                return Err(AppError::NotString {
                    file: file.to_string(),
                    field: field_name.to_string(),
                });
            }
        }
    }
    Ok(())
}
