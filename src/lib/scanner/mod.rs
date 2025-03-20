use std::sync::{Arc, Mutex};

use globset::GlobSet;
use rayon::prelude::*;

use crate::helpers;

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
    /// Whether to ignore files above the specified limit
    max_filesize: Option<u64>,
    /// Ignore hidden files in the scan
    ignore_hidden: bool,
    /// Exclude files that match the pattern from the scan
    exclude: Option<GlobSet>,
    /// Parallelize the scanning process
    parallel: bool,
}

impl Scanner {
    /// Instantiates a new [`Scanner`]
    pub fn new() -> Scanner {
        Scanner {
            ..Default::default()
        }
    }

    /// Set the maximum depth to recurse when scanning
    pub fn scan_depth(&mut self, depth: Option<usize>) -> &mut Self {
        self.scan_depth = depth;
        self
    }

    /// Set the max filesize limit above which the scanner ignores the file
    pub fn max_filesize(&mut self, size: Option<u64>) -> &mut Self {
        self.max_filesize = size;
        self
    }

    /// Whether or not the scanner should ignore hidden files
    pub fn ignore_hidden(&mut self, yes: bool) -> &mut Self {
        self.ignore_hidden = yes;
        self
    }

    /// Exclude files that match the pattern from the scan
    pub fn exclude(&mut self, exclude: GlobSet) -> &mut Self {
        self.exclude = Some(exclude);
        self
    }

    /// Run the scanning process in parallel
    pub fn run_parallel(&mut self, yes: bool) -> &mut Self {
        self.parallel = yes;
        self
    }

    pub fn scan<P: AsRef<std::path::Path>>(&self, paths: &[P]) -> std::io::Result<ScanResults> {
        if self.parallel {
            self.scan_parallel(paths)
        } else {
            self.scan_serial(paths)
        }
    }

    /// Scan the given [`paths`][std::path::Path] and record [file information][File] such as the line, word, character, and byte counts for each file.
    pub fn scan_serial<P: AsRef<std::path::Path>>(
        &self,
        paths: &[P],
    ) -> std::io::Result<ScanResults> {
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

    /// Scan the given [`paths`][std::path::Path] and record [file information][File] such as the line, word, character, and byte counts for each file.
    pub fn scan_parallel<P: AsRef<std::path::Path>>(
        &self,
        paths: &[P],
    ) -> std::io::Result<ScanResults> {
        // A vector to collect the files
        let files = Arc::new(Mutex::new(Vec::new()));

        // Accumulators
        let total = Arc::new(Mutex::new(Totals::default()));
        let max = Arc::new(Mutex::new(Max::default()));

        for path in paths {
            let mut local_files = Vec::new();
            let mut local_total = Totals::default();
            let mut local_max = Max::default();

            match path {
                // If the path is -, then scan STDIN
                p if p.as_ref().to_str() == Some("-") => {
                    let reader = std::io::BufReader::new(std::io::stdin());
                    if let Ok(results) = File::scan_reader(reader) {
                        local_total.add(&results);
                        local_max.track(&results);
                        local_files.push(results);
                    }
                }

                // If path points to a file, then parse the file, accumulate stats, and add to the collection
                p if p.as_ref().is_file() => {
                    if let Ok(file) = File::scan(path) {
                        local_total.add(&file);
                        local_max.track(&file);
                        local_files.push(file);
                    }
                }

                // If path points to a directory, then walk the directory accumulating stats, and add them to the collection
                p if p.as_ref().is_dir() => {
                    // Build a directory walker that respects `.gitignore` and other hidden files
                    let walker = self.configure_walker(path);

                    // Iterate over all the entries
                    walker.into_iter().par_bridge().for_each(|result| {
                        match result {
                            Ok(entry) if entry.path().is_file() => {
                                // Parse the file, accumulate stats, and add it to the collection
                                if let Ok(file) = File::scan(entry.path()) {
                                    let mut total_guard = total.lock().unwrap();
                                    let mut max_guard = max.lock().unwrap();
                                    let mut files_guard = files.lock().unwrap();

                                    total_guard.add(&file);
                                    max_guard.track(&file);
                                    files_guard.push(file);
                                }
                            }

                            Ok(_) => {} // Ignore directories and symlinks
                            Err(e) => eprintln!("Error: {}", e), // Report errors
                        }
                    });
                }

                _ => {} // Ignore all other cases
            }

            // Merge local results into global results safely
            {
                let mut files = files.lock().unwrap();
                files.extend(local_files);
            }
            {
                let mut total = total.lock().unwrap();
                total.merge(&local_total);
            }
            {
                let mut max = max.lock().unwrap();
                max.merge(&local_max);
            }
        }

        Ok(ScanResults {
            files: Arc::try_unwrap(files).unwrap().into_inner().unwrap(),
            total: Arc::try_unwrap(total).unwrap().into_inner().unwrap(),
            max: Arc::try_unwrap(max).unwrap().into_inner().unwrap(),
        })
    }

    /// Setup the walker with the provided configuration
    fn configure_walker<P: AsRef<std::path::Path>>(&self, path: P) -> ignore::Walk {
        let mut walker = ignore::WalkBuilder::new(path);

        // Set default configuration
        walker
            .max_depth(self.scan_depth)
            .max_filesize(self.max_filesize)
            .hidden(self.ignore_hidden);

        // Filter files that match the exclude pattern
        if let Some(exclude) = self.exclude.clone() {
            walker.filter_entry(move |entry| {
                let path = helpers::path::display(entry.path());
                if exclude.is_match(path) {
                    return false;
                }
                return true;
            });
        }

        walker.build()
    }
}
