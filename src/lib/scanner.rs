use std::collections::HashMap;

use crate::{file::File, language::Language};

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
}
