#[derive(Debug)]
pub(crate) enum Language {
    Text,
    Rust,
    Markdown,
    TOML,
    JSON,
    JavaScript,
    TypeScript,
    Go,
    C,
    CPP,
    CSharp,
    Python,
    Java,
    HTML,
    CSS,
    YAML,
    Unknown(String),
}

impl From<&std::path::PathBuf> for Language {
    fn from(path: &std::path::PathBuf) -> Self {
        let extension = match path.extension().and_then(|ext| ext.to_str()) {
            Some(ext) => ext,
            None => return Language::Text,
        };

        match extension {
            "txt" | "text" => Language::Text,
            "rs" => Language::Rust,
            "md" | "markdown" => Language::Markdown,
            "toml" => Language::TOML,
            "json" | "jsonc" => Language::JSON,
            "js" => Language::JavaScript,
            "ts" => Language::TypeScript,
            "go" => Language::Go,
            "c" => Language::C,
            "cpp" => Language::CPP,
            "cs" => Language::CSharp,
            "py" => Language::Python,
            "java" => Language::Java,
            "html" | "htm" => Language::HTML,
            "css" => Language::CSS,
            "yaml" | "yml" => Language::YAML,
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
            Language::TOML => write!(f, "TOML"),
            Language::JSON => write!(f, "JSON"),
            Language::JavaScript => write!(f, "JavaScript"),
            Language::TypeScript => write!(f, "TypeScript"),
            Language::Go => write!(f, "Go"),
            Language::C => write!(f, "C"),
            Language::CPP => write!(f, "C++"),
            Language::CSharp => write!(f, "C#"),
            Language::Python => write!(f, "Python"),
            Language::Java => write!(f, "Java"),
            Language::HTML => write!(f, "HTML"),
            Language::CSS => write!(f, "CSS"),
            Language::YAML => write!(f, "YAML"),
            Language::Unknown(x) => write!(f, "Unknown: {}", x),
        }
    }
}
