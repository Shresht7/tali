use std::collections::HashMap;

use crate::{
    file::File,
    helpers::table::{Alignment, Table},
    language::Language,
};

// -------
// SCANNER
// -------

/// Scan the given [`paths`][std::path::Path] and record [file information][File] such as the line, word, character, and byte counts for each file.
pub fn scan<P: AsRef<std::path::Path>>(paths: &[P]) -> std::io::Result<ScanResults> {
    // A vector to collect the files
    let mut files = Vec::new();

    for path in paths {
        if path.as_ref().is_file() {
            // Parse the file and add to the collection
            files.push(File::scan(path)?)
        } else {
            // Build a directory walker that respects `.gitignore` and other hidden files
            let walker = ignore::WalkBuilder::new(&path).build();

            // Iterate over all the entries
            for result in walker {
                match result {
                    Ok(entry) if entry.path().is_file() => {
                        // Parse the file and add it to the collection
                        let file = File::scan(entry.path())?;
                        files.push(file);
                    }

                    Ok(_) => {} // Ignore directories and symlinks
                    Err(e) => eprintln!("Error: {}", e), // Report errors
                }
            }
        }
    }

    Ok(ScanResults { files })
}

// ------------
// SCAN RESULTS
// ------------

/// Represents the aggregate scan results
#[derive(Debug)]
pub struct ScanResults {
    pub files: Vec<File>,
}

impl ScanResults {
    /// Groups the files by language and returns a [`HashMap`]
    fn group_by_language(&self) -> HashMap<Language, Vec<&File>> {
        let mut groups = HashMap::new();

        for file in &self.files {
            groups
                .entry(file.language.clone())
                .or_insert_with(Vec::new)
                .push(file)
        }

        groups
    }

    pub fn display(&self) -> String {
        let mut res = String::new();
        let mut total_no_of_files = 0;
        for (language, files) in self.group_by_language() {
            res.push_str(&format!(
                "{}\t{}\t{}\n",
                color(&language, &language.to_string()),
                files.len(),
                files.iter().fold(0, |mut acc, curr| {
                    acc += curr.lines;
                    acc
                })
            ));
            total_no_of_files += files.len();
        }
        let mut res = Table::from(&res, '\t');
        res.with_header(vec!["Language".into(), "Files".into(), "Lines".into()])
            .with_footer(vec!["Total".into(), total_no_of_files.to_string()])
            .with_alignments(vec![Alignment::Left, Alignment::Center, Alignment::Right]);
        res.display()
    }
}

/// A helper function to color a string according to the language's color
fn color(language: &Language, text: &str) -> String {
    let (r, g, b) = language.color();
    format!("\u{001b}[38;2;{};{};{}m{}\u{001b}[0m", r, g, b, text)
}

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
    pub fn group_by_language(mut self, yes: bool) -> Self {
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

        let mut table = Table::from(&res, '\t');
        table
            .with_header(header)
            .with_footer(footer)
            .with_alignments(alignments);

        table.display()
    }

    fn build_header(&self) -> Vec<String> {
        let mut headers = Vec::new();

        if self.path {
            headers.push("Path".to_string());
        }

        if self.lines {
            headers.push("Lines".to_string());
        }

        if self.words {
            headers.push("Words".to_string());
        }

        if self.chars {
            headers.push("Chars".to_string());
        }

        if self.bytes {
            headers.push("Bytes".to_string());
        }

        headers
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
        let mut footer = Vec::new();

        if self.path {
            footer.push("Total".to_string());
        }

        if self.lines {
            footer.push(totals.lines.to_string());
        }

        if self.words {
            footer.push(totals.words.to_string());
        }

        if self.chars {
            footer.push(totals.chars.to_string());
        }

        if self.bytes {
            footer.push(totals.bytes.to_string());
        }

        footer
    }

    fn build_alignments(&self) -> Vec<Alignment> {
        let mut alignments = Vec::with_capacity(5);

        if self.path {
            alignments.push(Alignment::Left);
        }

        if self.lines {
            alignments.push(Alignment::Right);
        }

        if self.words {
            alignments.push(Alignment::Right);
        }

        if self.chars {
            alignments.push(Alignment::Right);
        }

        if self.bytes {
            alignments.push(Alignment::Right);
        }

        alignments
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
