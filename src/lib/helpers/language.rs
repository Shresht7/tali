use serde::Serialize;

/// Describe a macro to generate the [`Language`] enum
macro_rules! define_languages {
    ( $(
        $language:ident                                             // Matches: Rust                    | Language Identifier
        $(as $display:literal)?                                     // Matches: as "RS"                 | Optional display name
        from [$($extension:literal),*]                              // Matches: from ["rs", ...]        | file-extension list
        $(with RGB($colorR:expr, $colorG:expr, $colorB:expr))?      // Matches: with RGB(255, 165, 0)   | RGB color for the language
    ),* $(,)? ) => {                                                // Matches: ,                       | Optional trailing comma
        #[derive(Debug, Serialize, Clone, PartialEq, Eq, Hash)]
        pub enum Language {
            $($language),*,
            Unknown(String),
        }

        impl Language {
            /// Parse a [`Language`] from a file-extension
            pub fn from_extension(ext: &str) -> Language {
                let ext = ext.to_lowercase();
                match ext.as_str() {
                    $( $( $extension => Language::$language, )* )*
                    _ => Language::Unknown(ext),
                }
            }

            /// Parse a [`Language`] from a file-path
            pub fn from_path<P: AsRef<std::path::Path>>(path: P) -> Language {
                let extension = match path.as_ref().extension().and_then(|ext| ext.to_str()) {
                    Some(ext) => ext,
                    None => return Language::Text,
                };
                Language::from_extension(extension)
            }

            /// Get the RGB color associated with the language
            pub fn color(&self) -> (u8, u8, u8) {
                match self {
                    $(Language::$language => define_languages!(@color $($colorR, $colorG, $colorB)?),)*
                    Language::Unknown(_) => (127, 127, 127),
                }
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

    (@color $r:expr, $g:expr, $b:expr) => { ($r, $g, $b) };
    (@color) => { (255, 255, 255) };

    (@display $language:ident) => { stringify!($language) };
    (@display $language:ident, $display:literal) => { $display };
}

// Holy cow, macros are witchcraft
define_languages! {
    Astro               from ["astro"]                  with RGB(255, 69, 0),
    Bash                from ["sh"]                     with RGB(88, 156, 88),
    C                   from ["c"]                      with RGB(70, 70, 240),
    CPP as "C++"        from ["cpp"]                    with RGB(45, 45, 255),
    CSharp as "C#"      from ["cs"]                     with RGB(98, 164, 228),
    CSS                 from ["css"]                    with RGB(86, 61, 124),
    CSV                 from ["csv"]                    with RGB(0, 123, 255),
    Go                  from ["go"]                     with RGB(0, 173, 216),
    HTML                from ["html", "htm"]            with RGB(227, 76, 38),
    Java                from ["java"]                   with RGB(176, 114, 25),
    JavaScript          from ["js", "mjs", "cjs"]       with RGB(247, 223, 30),
    JSON                from ["json", "jsonc"]          with RGB(255, 224, 102),
    Kotlin              from ["kt", "kts"]              with RGB(136, 58, 163),
    Lua                 from ["lua"]                    with RGB(0, 0, 255),
    Makefile            from ["mk", "makefile"]         with RGB(48, 77, 48),
    Markdown            from ["md", "markdown"]         with RGB(0, 102, 204),
    Perl                from ["pl", "pm"]               with RGB(129, 133, 149),
    PHP                 from ["php"]                    with RGB(79, 93, 149),
    PowerShell          from ["ps1", "psm1", "psd1"]    with RGB(1, 36, 86),
    Python              from ["py"]                     with RGB(53, 114, 165),
    React               from ["jsx", "tsx"]             with RGB(0, 122, 204),
    Ruby                from ["rb"]                     with RGB(204, 52, 51),
    Rust                from ["rs"]                     with RGB(255, 165, 0),
    Svelte              from ["svelte"]                 with RGB(255, 62, 0),
    SVG                 from ["svg"]                    with RGB(255, 181, 0),
    Swift               from ["swift"]                  with RGB(255, 102, 0),
    Text                from ["txt", "text"]            with RGB(255, 255, 255),
    TOML                from ["toml"]                   with RGB(120, 120, 120),
    TSV                 from ["tsv"]                    with RGB(0, 123, 255),
    TypeScript          from ["ts"]                     with RGB(0, 122, 204),
    XML                 from ["xml"]                    with RGB(255, 153, 51),
    YAML                from ["yaml", "yml"]            with RGB(255, 255, 0),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_extension() {
        assert_eq!(Language::from_extension("rs"), Language::Rust);
        assert_eq!(Language::from_extension("cpp"), Language::CPP);
        assert_eq!(Language::from_extension("py"), Language::Python);
        assert_eq!(
            Language::from_extension("unknown_ext"),
            Language::Unknown("unknown_ext".to_string())
        );
    }

    #[test]
    fn test_from_path() {
        assert_eq!(Language::from_path("main.rs"), Language::Rust);
        assert_eq!(Language::from_path("script.py"), Language::Python);
        assert_eq!(Language::from_path("index.html"), Language::HTML);
        assert_eq!(Language::from_path("/no_extension"), Language::Text);
    }
}
