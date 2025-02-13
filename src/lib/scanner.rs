use std::collections::HashMap;

use crate::{file::File, language::Language};

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
        if path.as_ref().is_file() {
            // Parse the file, accumulate stats, and add to the collection
            let file = File::scan(path)?;
            total.add(&file);
            max.track(&file);
            files.push(file);
        } else {
            // Build a directory walker that respects `.gitignore` and other hidden files
            let walker = ignore::WalkBuilder::new(&path).build();

            // Iterate over all the entries
            for result in walker {
                match result {
                    Ok(entry) if entry.path().is_file() => {
                        // Parse the file, accumulate stats, and add it to the collection
                        let file = File::scan(entry.path())?;
                        total.add(&file);
                        max.track(&file);
                        files.push(file);
                    }

                    Ok(_) => {} // Ignore directories and symlinks
                    Err(e) => eprintln!("Error: {}", e), // Report errors
                }
            }
        }
    }

    Ok(ScanResults { files, total, max })
}

// ------------
// SCAN RESULTS
// ------------

/// Represents the aggregate scan results
#[derive(Debug)]
pub struct ScanResults {
    pub files: Vec<File>,
    pub total: Totals,
    pub max: Max,
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
}

// ------------
// ACCUMULATORS
// ------------

/// Helper struct for accumulating totals
#[derive(Debug, Default)]
pub struct Totals {
    pub files: usize,
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: u64,
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

/// Helper struct to accumulate max
#[derive(Debug, Default)]
pub struct Max {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: u64,
}

impl Max {
    fn track(&mut self, file: &File) {
        self.lines = self.lines.max(file.lines);
        self.words = self.words.max(file.words);
        self.chars = self.chars.max(file.chars);
        self.bytes = self.bytes.max(file.bytes);
    }
}
