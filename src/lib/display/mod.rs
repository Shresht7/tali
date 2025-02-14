use crate::{language::Language, File, ScanResults};

mod helpers;
use helpers::table::{Alignment, Table};

#[derive(Debug)]
pub struct Display {
    pub group_by_language: bool,
    pub path: bool,
    pub lines: bool,
    pub words: bool,
    pub chars: bool,
    pub bytes: bool,
    pub language: bool,
    pub visualization: bool,

    pub use_colors: bool,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            group_by_language: false,
            path: true,
            lines: true,
            words: true,
            chars: true,
            bytes: true,
            language: true,
            visualization: true,
            use_colors: true,
        }
    }
}

impl Display {
    /// Sets whether to group output by language
    pub fn group_by_language(&mut self, yes: bool) -> &mut Self {
        self.group_by_language = yes;
        self
    }

    /// Sets whether to show file path
    pub fn path(&mut self, yes: bool) -> &mut Self {
        self.path = yes;
        self
    }

    /// Sets whether to show the line count
    pub fn lines(&mut self, yes: bool) -> &mut Self {
        self.lines = yes;
        self
    }

    /// Sets whether to show the word count
    pub fn words(&mut self, yes: bool) -> &mut Self {
        self.words = yes;
        self
    }

    /// Sets whether to show the character count
    pub fn chars(&mut self, yes: bool) -> &mut Self {
        self.chars = yes;
        self
    }

    /// Sets whether to show byte count
    pub fn bytes(&mut self, yes: bool) -> &mut Self {
        self.bytes = yes;
        self
    }

    pub fn display(&self, results: &ScanResults) -> String {
        let mut res = String::new();

        let header = self.build_header();

        for file in &results.files {
            res.push_str(&self.build_row(file, &results));
        }

        let footer = self.build_footer(&results);

        let alignments = self.build_alignments();

        let mut table = Table::from_tsv(&res);
        table
            .with_header(header)
            .with_footer(footer)
            .with_alignments(alignments);

        table.display()
    }

    // Helper function to select columns
    fn selected_columns<T>(&self, values: T) -> Vec<T::Item>
    where
        T: IntoIterator,
        T::IntoIter: ExactSizeIterator,
        T::Item: Clone,
    {
        let options = [
            self.language,
            self.path,
            self.lines,
            self.words,
            self.chars,
            self.bytes,
            self.visualization,
        ];
        values
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| options[i].then_some(v.clone()))
            .collect()
    }

    fn build_header(&self) -> Vec<String> {
        self.selected_columns(
            [
                "Language", "Path", "Lines", "Words", "Chars", "Bytes", "Graph",
            ]
            .map(String::from),
        )
    }

    fn build_row(&self, file: &File, results: &ScanResults) -> String {
        let mut cols = Vec::new();

        if self.language {
            let lang = if self.use_colors {
                color(&file.language, &file.language.to_string())
            } else {
                file.language.to_string()
            };
            cols.push(lang);
        }

        if self.path {
            cols.push(file.path.to_string_lossy().to_string());
        }

        if self.lines {
            cols.push(file.lines.to_string());
        }

        if self.words {
            cols.push(file.words.to_string());
        }

        if self.chars {
            cols.push(file.chars.to_string());
        }

        if self.bytes {
            cols.push(file.bytes.to_string());
        }

        if self.visualization {
            let filled = "█";
            let blank = "░";
            let bar_length = (file.bytes as f64 / results.max.bytes as f64 * 20.0).round() as usize;
            let bar = filled.repeat(bar_length) + &blank.repeat(20 - bar_length);
            cols.push(color(&file.language, &bar));
        }

        cols.join("\t") + "\n"
    }

    fn build_footer(&self, results: &ScanResults) -> Vec<String> {
        self.selected_columns([
            "Total".to_string(),
            "".to_string(),
            results.total.lines.to_string(),
            results.total.words.to_string(),
            results.total.chars.to_string(),
            results.total.bytes.to_string(),
        ])
    }

    fn build_alignments(&self) -> Vec<Alignment> {
        self.selected_columns([
            Alignment::Right,
            Alignment::Left,
            Alignment::Right,
            Alignment::Right,
            Alignment::Right,
            Alignment::Right,
            Alignment::Left,
        ])
    }
}

/// A helper function to color a string according to the language's color
fn color(language: &Language, text: &str) -> String {
    let (r, g, b) = language.color();
    format!("\u{001b}[38;2;{};{};{}m{}\u{001b}[0m", r, g, b, text)
}
