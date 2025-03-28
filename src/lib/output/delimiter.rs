use crate::{
    helpers::path,
    scanner::{File, ScanResults},
};

use super::{Config, Formatter};

#[derive(Debug, Default)]
pub struct DelimiterFormatter<'a> {
    delimiter: &'a str,
}

impl<'a> DelimiterFormatter<'a> {
    pub fn with(delimiter: &'a str) -> DelimiterFormatter<'a> {
        DelimiterFormatter { delimiter }
    }
}

impl Formatter for DelimiterFormatter<'_> {
    fn format(&self, results: &ScanResults, config: &Config) -> String {
        let mut res = String::new();

        if config.header {
            res.push_str(&self.build_header(config));
        }

        for file in &results.files {
            res.push_str(&self.build_row(file, results, config));
        }

        if config.footer {
            res.push_str(&self.build_footer(results, config));
        }

        res
    }
}

impl DelimiterFormatter<'_> {
    // Helper function to select columns
    fn selected_columns<T>(&self, values: T, config: &Config) -> Vec<T::Item>
    where
        T: IntoIterator,
        T::IntoIter: ExactSizeIterator,
        T::Item: Clone,
    {
        let options = [
            config.language,
            config.files,
            config.lines,
            config.words,
            config.chars,
            config.bytes,
        ];
        values
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| options[i].then_some(v.clone()))
            .collect()
    }

    fn build_header(&self, config: &Config) -> String {
        let files: &str = if config.group_by_language {
            "Files"
        } else {
            "Path"
        };
        self.selected_columns(
            ["Language", files, "Lines", "Words", "Chars", "Bytes"].map(String::from),
            config,
        )
        .join(self.delimiter)
            + "\n"
    }

    fn build_row(&self, file: &File, _results: &ScanResults, config: &Config) -> String {
        let mut cols = Vec::new();

        if config.language {
            cols.push(file.language.to_string());
        }

        if config.files {
            let file = if config.group_by_language {
                file.count.to_string()
            } else {
                path::display(&file.path)
            };
            cols.push(file);
        }

        if config.lines {
            cols.push(file.lines.to_string());
        }

        if config.words {
            cols.push(file.words.to_string());
        }

        if config.chars {
            cols.push(file.chars.to_string());
        }

        if config.bytes {
            cols.push(file.bytes.to_string());
        }

        cols.join(self.delimiter) + "\n"
    }

    fn build_footer(&self, results: &ScanResults, config: &Config) -> String {
        self.selected_columns(
            [
                "Total".to_string(),
                results.total.files.to_string(),
                results.total.lines.to_string(),
                results.total.words.to_string(),
                results.total.chars.to_string(),
                results.total.bytes.to_string(),
            ],
            config,
        )
        .join(self.delimiter)
    }
}
