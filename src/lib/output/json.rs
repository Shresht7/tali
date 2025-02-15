use super::{Formatter, Output};

#[derive(Default, Debug)]
pub struct JSONFormatter {}

impl Formatter for JSONFormatter {
    fn format(&self, results: &crate::scanner::ScanResults, _config: &Output) -> String {
        serde_json::to_string_pretty(&results).unwrap_or_default()
    }
}

// The following helper methods construct json object based on the values allowed by the config
//
// impl JSONFormatter {
//     fn jsonify_files(
//         &self,
//         results: &crate::scanner::ScanResults,
//         config: &Display,
//     ) -> serde_json::Value {
//         let json_files: Vec<serde_json::Value> = results
//             .files
//             .iter()
//             .map(|file| {
//                 let mut map = serde_json::Map::new();

//                 map.insert(
//                     "path".into(),
//                     serde_json::Value::String(file.path.to_string_lossy().to_string()),
//                 );

//                 if config.language {
//                     map.insert(
//                         "language".into(),
//                         serde_json::Value::String(file.language.to_string()),
//                     );
//                 }

//                 if config.lines {
//                     map.insert("lines".into(), serde_json::Value::Number(file.lines.into()));
//                 }

//                 if config.words {
//                     map.insert("words".into(), serde_json::Value::Number(file.words.into()));
//                 }

//                 if config.chars {
//                     map.insert("chars".into(), serde_json::Value::Number(file.chars.into()));
//                 }

//                 if config.bytes {
//                     map.insert("bytes".into(), serde_json::Value::Number(file.bytes.into()));
//                 }

//                 serde_json::Value::Object(map)
//             })
//             .collect();

//         serde_json::Value::Array(json_files)
//     }

//     fn jsonify_total(
//         &self,
//         results: &crate::scanner::ScanResults,
//         config: &Display,
//     ) -> serde_json::Value {
//         let mut json_total = serde_json::Map::new();

//         if config.lines {
//             json_total.insert(
//                 "lines".into(),
//                 serde_json::Value::Number(results.total.lines.into()),
//             );
//         }

//         if config.words {
//             json_total.insert(
//                 "words".into(),
//                 serde_json::Value::Number(results.total.words.into()),
//             );
//         }

//         if config.chars {
//             json_total.insert(
//                 "chars".into(),
//                 serde_json::Value::Number(results.total.chars.into()),
//             );
//         }

//         if config.bytes {
//             json_total.insert(
//                 "bytes".into(),
//                 serde_json::Value::Number(results.total.bytes.into()),
//             );
//         }

//         serde_json::Value::Object(json_total)
//     }

//     fn jsonify_max(
//         &self,
//         results: &crate::scanner::ScanResults,
//         config: &Display,
//     ) -> serde_json::Value {
//         let mut json_max = serde_json::Map::new();

//         if config.lines {
//             json_max.insert(
//                 "lines".into(),
//                 serde_json::Value::Number(results.max.lines.into()),
//             );
//         }

//         if config.words {
//             json_max.insert(
//                 "words".into(),
//                 serde_json::Value::Number(results.max.words.into()),
//             );
//         }

//         if config.chars {
//             json_max.insert(
//                 "chars".into(),
//                 serde_json::Value::Number(results.max.chars.into()),
//             );
//         }

//         if config.bytes {
//             json_max.insert(
//                 "bytes".into(),
//                 serde_json::Value::Number(results.max.bytes.into()),
//             );
//         }

//         serde_json::Value::Object(json_max)
//     }
// }
