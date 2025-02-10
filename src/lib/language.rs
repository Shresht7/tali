#[derive(Debug)]
pub(crate) enum Language {
    Text,
    Rust,
    Markdown,
    Unknown(String),
    None,
}

impl From<&std::path::PathBuf> for Language {
    fn from(path: &std::path::PathBuf) -> Self {
        let extension = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ext,
            None => return Language::None,
        };

        match extension {
            "txt" | "text" => Language::Text,
            "rs" => Language::Rust,
            "md" | "markdown" => Language::Markdown,
            ext => Language::Unknown(ext.to_string()),
        }
    }
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Text => write!(f, "Text"),
            Language::Rust => write!(f, "Rust"),
            Language::Markdown => write!(f, "Markdown"),
            Language::Unknown(x) => write!(f, "Unknown: {}", x),
            Language::None => write!(f, "None"),
        }
    }
}
