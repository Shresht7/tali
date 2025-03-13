use std::collections::HashMap;

use serde::Serialize;

mod file;
pub use file::*;

// -------
// SCANNER
// -------

/// Scan the given [`paths`][std::path::Path] and record [file information][File] such as the line, word, character, and byte counts for each file.
pub fn scan<P: AsRef<std::path::Path>>(paths: &[P]) -> std::io::Result<ScanResults> {
    // A vector to collect the files
    let mut files = Vec::new();

    // Accumulators
    let mut total = Totals::default();
    let mut max = Max::default();

    for path in paths {
        match path {
            // If the path is -, then scan STDIN
            p if p.as_ref().to_str() == Some("-") => {
                let reader = std::io::BufReader::new(std::io::stdin());
                if let Ok(results) = File::scan_reader(reader) {
                    total.add(&results);
                    max.track(&results);
                    files.push(results);
                }
            }

            // If path points to a file, then parse the file, accumulate stats, and add to the collection
            p if p.as_ref().is_file() => {
                if let Ok(file) = File::scan(path) {
                    total.add(&file);
                    max.track(&file);
                    files.push(file);
                }
            }

            // If path points to a directory, then walk the directory accumulating stats, and add them to the collection
            p if p.as_ref().is_dir() => {
                // Build a directory walker that respects `.gitignore` and other hidden files
                let walker = ignore::WalkBuilder::new(&path).build();

                // Iterate over all the entries
                for result in walker {
                    match result {
                        Ok(entry) if entry.path().is_file() => {
                            // Parse the file, accumulate stats, and add it to the collection
                            if let Ok(file) = File::scan(entry.path()) {
                                total.add(&file);
                                max.track(&file);
                                files.push(file);
                            }
                        }

                        Ok(_) => {} // Ignore directories and symlinks
                        Err(e) => eprintln!("Error: {}", e), // Report errors
                    }
                }
            }

            _ => {} // Ignore all other cases
        }
    }

    Ok(ScanResults { files, total, max })
}

// ------------
// SCAN RESULTS
// ------------

/// Represents the aggregate scan results
#[derive(Debug, Serialize)]
pub struct ScanResults {
    /// The collection of all file results containing information like the number of lines, words, chars and bytes
    pub files: Vec<File>,
    /// The aggregate total number of lines, words, chars and bytes
    pub total: Totals,
    /// The max values for the number of lines, words, chars, and bytes across the results
    pub max: Max,
}

impl ScanResults {
    /// Groups the files by language and returns a new [`ScanResults`] instance
    pub fn group_by_language(&self) -> ScanResults {
        // Group the files by language in a HashMap
        let mut groups = HashMap::new();
        for file in self.files.iter().cloned() {
            groups
                .entry(file.language.clone())
                .or_insert_with(Vec::new)
                .push(file)
        }

        // Create a new condensed ScanResult
        let mut files = Vec::new();
        let mut total = Totals::default();
        let mut max = Max::default();
        for lang in groups.keys() {
            if let Some(v) = groups.get(lang) {
                if let Some(file) = v.iter().cloned().reduce(|acc, e| acc + e) {
                    total.add(&file);
                    max.track(&file);
                    files.push(file)
                }
            }
        }
        ScanResults { files, total, max }
    }

    /// Sort the [`ScanResults`] files based on the given column and sort order
    pub fn sort_by(&mut self, category: &str, order: &SortOrder) {
        match category.to_lowercase().as_str() {
            "line" | "lines" => self.files.sort_by(|a, b| match order {
                SortOrder::Ascending => a.lines.cmp(&b.lines),
                SortOrder::Descending => b.lines.cmp(&a.lines),
            }),
            "word" | "words" => self.files.sort_by(|a, b| match order {
                SortOrder::Ascending => a.words.cmp(&b.words),
                SortOrder::Descending => b.words.cmp(&a.words),
            }),
            "char" | "chars" => self.files.sort_by(|a, b| match order {
                SortOrder::Ascending => a.chars.cmp(&b.chars),
                SortOrder::Descending => b.chars.cmp(&a.chars),
            }),
            "byte" | "bytes" | _ => self.files.sort_by(|a, b| match order {
                SortOrder::Ascending => a.bytes.cmp(&b.bytes),
                SortOrder::Descending => b.bytes.cmp(&a.bytes),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl std::str::FromStr for SortOrder {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" | "asc" | "ascending" => Ok(Self::Ascending),
            "d" | "desc" | "descending" => Ok(Self::Descending),
            x => Err(format!("Invalid sorting order: {x}")),
        }
    }
}

// ------------
// ACCUMULATORS
// ------------

/// Represents the accumulated total number of lines, words, chars and bytes in [`ScanResults`]
#[derive(Debug, Default, Serialize)]
pub struct Totals {
    pub files: usize,
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: u64,
}

impl Totals {
    /// Add the [`File`] statistics to the totals accumulator
    fn add(&mut self, file: &File) {
        self.files += 1;
        self.lines += file.lines;
        self.words += file.words;
        self.chars += file.chars;
        self.bytes += file.bytes;
    }
}

/// Represents the max values for the number of lines, words, chars and bytes in [`ScanResults`]
#[derive(Debug, Default, Serialize)]
pub struct Max {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: u64,
}

impl Max {
    /// Update the max values for the number of lines, words, chars and bytes by comparing it with a [`File`]
    fn track(&mut self, file: &File) {
        self.lines = self.lines.max(file.lines);
        self.words = self.words.max(file.words);
        self.chars = self.chars.max(file.chars);
        self.bytes = self.bytes.max(file.bytes);
    }
}
