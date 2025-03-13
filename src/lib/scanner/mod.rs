mod accumulators;
use accumulators::{Max, Totals};
mod file;
pub use file::File;
mod results;
pub use results::{ScanResults, SortOrder};

// -------
// SCANNER
// -------

#[derive(Default)]
pub struct Scanner {
    /// The maximum depth to recurse when scanning
    scan_depth: Option<usize>,
    // Ignore hidden files in the scan
    ignore_hidden: bool,
}

impl Scanner {
    /// Instantiates a new [`Scanner`]
    pub fn new() -> Scanner {
        Scanner {
            ..Default::default()
        }
    }

    /// Set the maximum depth to recurse when scanning
    pub fn scan_depth(mut self, depth: Option<usize>) -> Self {
        self.scan_depth = depth;
        self
    }

    /// Whether or not the scanner should ignore hidden files
    pub fn ignore_hidden(mut self, yes: bool) -> Self {
        self.ignore_hidden = yes;
        self
    }

    /// Scan the given [`paths`][std::path::Path] and record [file information][File] such as the line, word, character, and byte counts for each file.
    pub fn scan<P: AsRef<std::path::Path>>(&self, paths: &[P]) -> std::io::Result<ScanResults> {
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
                    let walker = self.configure_walker(path);

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

    /// Setup the walker with the provided configuration
    fn configure_walker<P: AsRef<std::path::Path>>(&self, path: P) -> ignore::Walk {
        ignore::WalkBuilder::new(path)
            .max_depth(self.scan_depth)
            .hidden(self.ignore_hidden)
            .build()
    }
}
