use super::{Display, Formatter};

#[derive(Default, Debug)]
pub struct JSONFormatter {}

impl Formatter for JSONFormatter {
    fn format(&self, results: &crate::scanner::ScanResults, config: &Display) -> String {
        let json_files: Vec<serde_json::Value> = results
            .files
            .iter()
            .map(|file| {
                let mut map = serde_json::Map::new();

                map.insert(
                    "path".into(),
                    serde_json::Value::String(file.path.to_string_lossy().to_string()),
                );

                if config.language {
                    map.insert(
                        "language".into(),
                        serde_json::Value::String(file.language.to_string()),
                    );
                }

                if config.lines {
                    map.insert("lines".into(), serde_json::Value::Number(file.lines.into()));
                }

                if config.words {
                    map.insert("words".into(), serde_json::Value::Number(file.words.into()));
                }

                if config.chars {
                    map.insert("chars".into(), serde_json::Value::Number(file.chars.into()));
                }

                if config.bytes {
                    map.insert("bytes".into(), serde_json::Value::Number(file.bytes.into()));
                }

                serde_json::Value::Object(map)
            })
            .collect();

        let output = serde_json::json!({
            "files": json_files,
        });

        serde_json::to_string_pretty(&output).unwrap_or_default()
    }
}
