use serde::Deserialize;
use std::collections::{HashMap, HashSet};
use std::path::Path;

use crate::common::read_file;
use crate::error::{AppError, AppResult};

const ISSUES_CONTENT_TYPE: &str = "Issues";

#[derive(Debug, Deserialize)]
struct FrontmatterConfig {
    #[serde(rename = "frontMatter.taxonomy.contentTypes")]
    content_types: Vec<ContentType>,

    #[serde(default, rename = "frontMatter.taxonomy.categories")]
    allowed_categories: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct ContentType {
    name: String,
    fields: Vec<FieldDef>,
}

#[derive(Debug)]
pub(super) struct FieldSpec {
    pub(super) required: bool,
    pub(super) kind: FieldKind,
}

#[derive(Debug, Deserialize)]
struct FieldDef {
    name: String,
    #[serde(rename = "type")]
    kind: String,
    #[serde(default)]
    required: bool,
    #[serde(default)]
    choices: Option<Vec<String>>,
}

#[derive(Debug)]
pub(super) enum FieldKind {
    String,
    Choice(Vec<String>),
    Categories,
    Datetime,
}

impl TryFrom<&FieldDef> for FieldKind {
    type Error = AppError;

    fn try_from(def: &FieldDef) -> AppResult<Self> {
        let kind = match def.kind.as_str() {
            "string" => FieldKind::String,
            "choice" => {
                let choices =
                    def.choices
                        .clone()
                        .ok_or_else(|| AppError::ChoiceFieldMissingChoices {
                            field: def.name.clone(),
                        })?;
                FieldKind::Choice(choices)
            }
            "categories" => FieldKind::Categories,
            "datetime" => FieldKind::Datetime,
            other => {
                return Err(AppError::UnknownFieldKind {
                    field: def.name.clone(),
                    kind: other.to_string(),
                });
            }
        };
        Ok(kind)
    }
}

#[derive(Debug, Deserialize)]
struct TaxonomyDb {
    taxonomy: Taxonomy,
}

#[derive(Debug, Deserialize)]
struct Taxonomy {
    #[serde(default)]
    categories: Vec<String>,
}

#[derive(Debug)]
pub(super) struct ValidationRules {
    pub(super) fields: HashMap<String, FieldSpec>,
    pub(super) allowed_categories: Vec<String>,
}

impl ValidationRules {
    pub(super) fn load(frontmatter_path: &Path, taxonomy_db_path: &Path) -> AppResult<Self> {
        let raw = read_file(frontmatter_path)?;
        let config: FrontmatterConfig =
            serde_json::from_str(&raw).map_err(|source| AppError::ConfigParse {
                path: frontmatter_path.display().to_string(),
                source,
            })?;

        let issues = config
            .content_types
            .iter()
            .find(|ct| ct.name == ISSUES_CONTENT_TYPE)
            .ok_or(AppError::ContentTypeNotFound)?;

        let mut fields = HashMap::new();
        for f in &issues.fields {
            let kind = f.try_into()?;
            fields.insert(
                f.name.clone(),
                FieldSpec {
                    required: f.required,
                    kind,
                },
            );
        }

        let taxonomy_categories = read_file(taxonomy_db_path)?;
        let taxonomy_categories = serde_json::from_str::<TaxonomyDb>(&taxonomy_categories)
            .map(|db| db.taxonomy.categories)
            .unwrap_or_default();

        let mut merged: HashSet<String> = HashSet::new();
        merged.extend(config.allowed_categories.iter().cloned());
        merged.extend(taxonomy_categories.iter().cloned());
        let mut allowed_categories: Vec<String> = merged.into_iter().collect();
        allowed_categories.sort();

        if allowed_categories.is_empty() {
            return Err(AppError::NoCategoriesDefined);
        }

        Ok(Self {
            fields,
            allowed_categories,
        })
    }
}
