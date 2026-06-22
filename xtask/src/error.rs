pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    // common errors
    #[error("cannot determine current directory: {0}")]
    CurrentDir(#[from] std::io::Error),

    #[error("workspace root not found (no Cargo.toml + .issues/ directory)")]
    WorkspaceRootNotFound,

    #[error("cannot read {path}: {source}")]
    FileRead {
        path: String,
        source: std::io::Error,
    },

    #[error("invalid JSON in {path}: {source}")]
    ConfigParse {
        path: String,
        source: serde_json::Error,
    },

    // parsing frontmatter.json errors
    #[error("contentType 'Issues' not found in frontmatter.json")]
    ContentTypeNotFound,

    #[error("field '{field}' is choice type but has no choices defined")]
    ChoiceFieldMissingChoices { field: String },

    #[error("unknown field type '{kind}' for field '{field}'")]
    UnknownFieldKind { field: String, kind: String },

    // parsing *.md files errors
    #[error("cannot read .issues/ directory: {0}")]
    IssuesDirRead(std::io::Error),

    #[error("{file}: missing or malformed frontmatter")]
    MissingFrontmatter { file: String },

    #[error("{file}: YAML parse error: {source}")]
    YamlParse {
        file: String,
        source: serde_yaml::Error,
    },

    #[error("{file}: missing required field '{field}'")]
    MissingRequiredField { file: String, field: String },

    #[error("{file}: field '{field}' has invalid value '{value}' (allowed: {allowed})")]
    InvalidChoice {
        file: String,
        field: String,
        value: String,
        allowed: String,
    },

    #[error("{file}: field '{field}' has invalid category '{value}' (allowed: {allowed})")]
    InvalidCategory {
        file: String,
        field: String,
        value: String,
        allowed: String,
    },

    #[error("{file}: field '{field}' must be an array")]
    NotArray { file: String, field: String },

    #[error(
        "{file}: field '{field}' has invalid datetime format '{value}' (expected YYYY-MM-DDTHH:MM:SSZ)"
    )]
    InvalidDatetime {
        file: String,
        field: String,
        value: String,
    },

    #[error("{file}: field '{field}' must be a non-empty string")]
    EmptyRequiredString { file: String, field: String },

    #[error("{file}: field '{field}' must be a string")]
    NotString { file: String, field: String },

    #[error("no categories defined in frontmatter.json or taxonomyDb.json")]
    NoCategoriesDefined,

    #[error("lint-issues failed: {count} error(s) in {files} file(s)")]
    LintIssuesFailed { count: usize, files: usize },
}

pub trait ErrorCode {
    fn exit_code(&self) -> i32;
}

impl ErrorCode for AppError {
    fn exit_code(&self) -> i32 {
        match self {
            AppError::CurrentDir(_)
            | AppError::WorkspaceRootNotFound
            | AppError::FileRead { .. }
            | AppError::ConfigParse { .. }
            | AppError::ContentTypeNotFound
            | AppError::ChoiceFieldMissingChoices { .. }
            | AppError::UnknownFieldKind { .. }
            | AppError::IssuesDirRead(_)
            | AppError::NoCategoriesDefined => 2,

            AppError::MissingFrontmatter { .. }
            | AppError::YamlParse { .. }
            | AppError::MissingRequiredField { .. }
            | AppError::InvalidChoice { .. }
            | AppError::InvalidCategory { .. }
            | AppError::NotArray { .. }
            | AppError::InvalidDatetime { .. }
            | AppError::EmptyRequiredString { .. }
            | AppError::NotString { .. }
            | AppError::LintIssuesFailed { .. } => 1,
        }
    }
}
