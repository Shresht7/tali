use crate::{
    helpers::{
        language::Language,
        table::{Alignment, Table},
    },
    scanner::{File, ScanResults},
};

use super::{Config, Formatter};

#[derive(Debug, Default)]
pub struct TableFormatter {}

impl Formatter for TableFormatter {
    fn format(&self, results: &ScanResults, config: &Config) -> String {
        let mut res = String::new();

        let header = self.build_header(config);

        for file in &results.files {
            res.push_str(&self.build_row(file, &results, config));
        }

        let footer = self.build_footer(&results, config);

        let alignments = self.build_alignments(config);

        let mut table = Table::from_tsv(&res);
        table
            .with_header(header)
            .with_footer(footer)
            .with_alignments(alignments);

        table.display()
    }
}

impl TableFormatter {
    // Helper function to select columns
    fn selected_columns<T>(&self, values: T, config: &Config) -> Vec<T::Item>
    where
        T: IntoIterator,
        T::IntoIter: ExactSizeIterator,
        T::Item: Clone,
    {
        let options = [
            config.language,
            config.path,
            config.lines,
            config.words,
            config.chars,
            config.bytes,
            config.visualization,
        ];
        values
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| options[i].then_some(v.clone()))
            .collect()
    }

    fn build_header(&self, config: &Config) -> Vec<String> {
        self.selected_columns(
            [
                "Language", "Path", "Lines", "Words", "Chars", "Bytes", "Graph",
            ]
            .map(String::from),
            config,
        )
    }

    fn build_row(&self, file: &File, results: &ScanResults, config: &Config) -> String {
        let mut cols = Vec::new();

        if config.language {
            let lang = if config.use_colors {
                color(&file.language, &file.language.to_string())
            } else {
                file.language.to_string()
            };
            cols.push(lang);
        }

        if config.path {
            cols.push(file.path.to_string_lossy().to_string());
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

        if config.visualization {
            let filled = "█";
            let blank = "░";
            let bar_length = (file.bytes as f64 / results.max.bytes as f64 * 20.0).round() as usize;
            let bar = filled.repeat(bar_length) + &blank.repeat(20 - bar_length);
            let bar = if config.use_colors {
                color(&file.language, &bar)
            } else {
                bar
            };
            cols.push(bar);
        }

        cols.join("\t") + "\n"
    }

    fn build_footer(&self, results: &ScanResults, config: &Config) -> Vec<String> {
        self.selected_columns(
            [
                "Total".to_string(),
                "".to_string(),
                results.total.lines.to_string(),
                results.total.words.to_string(),
                results.total.chars.to_string(),
                results.total.bytes.to_string(),
            ],
            config,
        )
    }

    fn build_alignments(&self, config: &Config) -> Vec<Alignment> {
        self.selected_columns(
            [
                Alignment::Right,
                Alignment::Left,
                Alignment::Right,
                Alignment::Right,
                Alignment::Right,
                Alignment::Right,
                Alignment::Left,
            ],
            config,
        )
    }
}

/// A helper function to color a string according to the language's color
fn color(language: &Language, text: &str) -> String {
    let (r, g, b) = language.color();
    format!("\u{001b}[38;2;{};{};{}m{}\u{001b}[0m", r, g, b, text)
}
