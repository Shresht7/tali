/// Describe a macro to generate the [`Language`] enum
macro_rules! define_languages {
    ( $(
        $language:ident from [$($extension:literal),*]
        $(with RGB($colorR:expr, $colorG:expr, $colorB:expr))?
        $(as $display:literal)?
    ),* $(,)? ) => {
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
    Text        from ["txt", "text"]     with RGB(255, 255, 255),
    Rust        from ["rs"]              with RGB(255, 165, 0),
    Markdown    from ["md", "markdown"]  with RGB(0, 102, 204),
    TOML        from ["toml"]            with RGB(120, 120, 120),
    JSON        from ["json", "jsonc"]   with RGB(255, 224, 102),
    JavaScript  from ["js"]              with RGB(247, 223, 30),
    TypeScript  from ["ts"]              with RGB(0, 122, 204),
    Go          from ["go"]              with RGB(0, 173, 216),
    C           from ["c"]               with RGB(70, 70, 240),
    CPP         from ["cpp"]             with RGB(45, 45, 255)       as "C++",
    CSharp      from ["cs"]              with RGB(98, 164, 228)      as "C#",
    Python      from ["py"]              with RGB(53, 114, 165),
    Java        from ["java"]            with RGB(176, 114, 25),
    HTML        from ["html", "htm"]     with RGB(227, 76, 38),
    CSS         from ["css"]             with RGB(86, 61, 124),
    YAML        from ["yaml", "yml"]     with RGB(255, 255, 0)
}
