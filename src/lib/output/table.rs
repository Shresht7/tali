use crate::{
    helpers::{
        language::Language,
        path,
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

        for file in &results.files {
            res.push_str(&self.build_row(file, &results, config));
        }

        let mut table = Table::from_tsv(&res);

        if config.header {
            let header = self.build_header(config);
            table.with_header(header);
        }

        if config.footer {
            let footer = self.build_footer(&results, config);
            table.with_footer(footer);
        }

        if config.alignment {
            let alignments = self.build_alignments(config);
            table.with_alignments(alignments);
        }

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
            config.graph,
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
            let path = path::display(&file.path);
            cols.push(path);
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

        if config.graph {
            let bar = self.build_visualization(file, results, config);
            cols.push(bar);
        }

        cols.join("\t") + "\n"
    }

    fn build_visualization(&self, file: &File, results: &ScanResults, config: &Config) -> String {
        let fill = config.graph_fill.clone();
        let blank = config.graph_blank.clone();
        let max_length = 20;

        let bar_length = match config.graph_by.as_str() {
            "lines" | "line" | "l" => {
                (file.lines as f64 / results.max.lines as f64 * max_length as f64).round()
            }
            "words" | "word" | "w" => {
                (file.words as f64 / results.max.words as f64 * max_length as f64).round()
            }
            "chars" | "char" | "c" => {
                (file.chars as f64 / results.max.chars as f64 * max_length as f64).round()
            }
            "bytes" | "byte" | "b" | "size" | "filesize" | _ => {
                (file.bytes as f64 / results.max.bytes as f64 * max_length as f64).round()
            }
        } as usize;

        let bar = fill.repeat(bar_length) + &blank.repeat(20 - bar_length);
        let bar = if config.use_colors {
            color(&file.language, &bar)
        } else {
            bar
        };

        bar
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
