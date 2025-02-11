use std::collections::HashMap;

use crate::{
    file::FileScanner,
    language::Language,
    table::{Alignment, Table},
    FileScanResult,
};

// -------
// SCANNER
// -------

/// Scan the given [`paths`][std::path::Path] and record [file information][File] like line count, word count etc.
pub fn scan<P: AsRef<std::path::Path>>(paths: &Vec<P>) -> std::io::Result<ScanResults> {
    let mut files = Vec::new();

    // Instantiate and configure the file-scanner
    let scanner = FileScanner::new(true, true);

    for path in paths {
        if path.as_ref().is_file() {
            // Parse the file and add to the collection

            files.push(scanner.scan(path)?)
        } else {
            // Build a directory walker that respects `.gitignore` and other hidden files
            let walker = ignore::WalkBuilder::new(&path).build();

            // Iterate over all the entries
            for result in walker {
                match result {
                    Ok(entry) if entry.path().is_file() => {
                        // Parse the file and add it to the collection
                        let file = scanner.scan(entry.path())?;
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

#[derive(Debug)]
pub struct ScanResults {
    pub files: Vec<FileScanResult>,
}

impl ScanResults {
    /// Groups the files by language and returns a [`HashMap`]
    fn group_by_language(&self) -> HashMap<Language, Vec<&FileScanResult>> {
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
