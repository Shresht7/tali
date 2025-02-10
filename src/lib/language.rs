/// Describe a macro to generate the [`Language`] enum
macro_rules! define_languages {
    ( $( $language:ident from [$($extension:literal),*] $(as $display:literal)? ),* $(,)? ) => {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub enum Language {
            $($language),*,
            Unknown(String),
        }

        impl Language {
            pub fn from_extension(ext: &str) -> Language {
                let ext = ext.to_lowercase();
                match ext.as_str() {
                    $( $( $extension => Language::$language, )* )*
                    _ => Language::Unknown(ext),
                }
            }

            pub fn from_path(path: &std::path::PathBuf) -> Language {
                let extension = match path.extension().and_then(|ext| ext.to_str()) {
                    Some(ext) => ext,
                    None => return Language::Text,
                };
                Language::from_extension(extension)
            }
        }

        impl std::fmt::Display for Language {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                match self {
                    $(Language::$language => write!(f, "{}", define_languages!(@display $language $(, $display)?))),*,
                    Language::Unknown(x) => write!(f, ".{}", x),
                }
            }
        }
    };

    (@display $language:ident) => { stringify!($language) };
    (@display $language:ident, $display:literal) => { $display };
}

// Holy cow, macros are witchcraft
define_languages! {
    Text from ["txt", "text"],
    Rust from ["rs"],
    Markdown from ["md", "markdown"],
    TOML from ["toml"],
    JSON from ["json", "jsonc"],
    JavaScript from ["js"],
    TypeScript from ["ts"],
    Go from ["go"],
    C from ["c"],
    CPP from ["cpp"] as "C++",
    CSharp from ["cs"] as "C#",
    Python from ["py"],
    Java from ["java"],
    HTML from ["html", "htm"],
    CSS from ["css"],
    YAML from ["yaml", "yml"]
}
