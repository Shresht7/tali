use crate::{
    helpers::table::{Alignment, Table},
    language::Language,
    File, ScanResults,
};

#[derive(Debug)]
pub struct Display {
    pub group_by_language: bool,
    pub path: bool,
    pub lines: bool,
    pub words: bool,
    pub chars: bool,
    pub bytes: bool,

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

    pub fn display(&self, results: ScanResults) -> String {
        let mut res = String::new();
        let mut totals = Totals::default();

        let header = self.build_header();

        for file in &results.files {
            res.push_str(&self.build_row(file));
            totals.add(file);
        }

        let footer = self.build_footer(&totals);

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
        let options = [self.path, self.lines, self.words, self.chars, self.bytes];
        values
            .into_iter()
            .enumerate()
            .filter_map(|(i, v)| options[i].then_some(v.clone()))
            .collect()
    }

    fn build_header(&self) -> Vec<String> {
        self.selected_columns(["Path", "Lines", "Words", "Chars", "Bytes"].map(String::from))
    }

    fn build_row(&self, file: &File) -> String {
        let mut cols = Vec::new();

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

        cols.join("\t") + "\n"
    }

    fn build_footer(&self, totals: &Totals) -> Vec<String> {
        self.selected_columns([
            "Total".to_string(),
            totals.lines.to_string(),
            totals.words.to_string(),
            totals.chars.to_string(),
            totals.bytes.to_string(),
        ])
    }

    fn build_alignments(&self) -> Vec<Alignment> {
        self.selected_columns([
            Alignment::Left,
            Alignment::Right,
            Alignment::Right,
            Alignment::Right,
            Alignment::Right,
        ])
    }
}

/// Helper struct for accumulating totals
#[derive(Default)]
struct Totals {
    files: usize,
    lines: usize,
    words: usize,
    chars: usize,
    bytes: u64,
}

impl Totals {
    fn add(&mut self, file: &File) {
        self.files += 1;
        self.lines += file.lines;
        self.words += file.words;
        self.chars += file.chars;
        self.bytes += file.bytes;
    }
}

/// A helper function to color a string according to the language's color
fn color(language: &Language, text: &str) -> String {
    let (r, g, b) = language.color();
    format!("\u{001b}[38;2;{};{};{}m{}\u{001b}[0m", r, g, b, text)
}
