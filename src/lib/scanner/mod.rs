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

    /// Scan the given paths and return results as [`ScanResults`]
    pub fn scan<P: AsRef<std::path::Path>>(&self, paths: &[P]) -> std::io::Result<ScanResults> {
        let mut files = Vec::new();

        for path in paths {
            match path {
                // If the path is -, then scan STDIN
                p if p.as_ref().to_str() == Some("-") => {
                    let reader = std::io::BufReader::new(std::io::stdin());
                    if let Ok(results) = File::scan_reader(reader) {
                        files.push(results);
                    }
                }

                // If path points to a file, then parse the file, accumulate stats, and add to the collection
                p if p.as_ref().is_file() => {
                    if let Ok(file) = File::scan(path) {
                        files.push(file);
                    }
                }

                // If path points to a directory, then walk the directory accumulating stats, and add them to the collection
                p if p.as_ref().is_dir() => {
                    files.extend(self.scan_directory(path));
                }

                _ => {} // Ignore all other cases
            }
        }

        // Return the aggregated results
        let (total, max) = Self::aggregate(&files);
        Ok(ScanResults { files, total, max })
    }

    /// Scan a directory for [files][File]
    fn scan_directory<P: AsRef<std::path::Path>>(&self, path: P) -> Vec<File> {
        let walker = self.configure_walker(path);
        if self.parallel {
            walker.par_bridge().filter_map(Self::scan_entry).collect()
        } else {
            walker.filter_map(Self::scan_entry).collect()
        }
    }

    /// Scan a single entry and return a [`File`] if valid
    fn scan_entry(result: Result<ignore::DirEntry, ignore::Error>) -> Option<File> {
        match result {
            Ok(entry) if entry.path().is_file() => File::scan(entry.path()).ok(),
            Err(e) => {
                eprintln!("Error: {}", e);
                None
            }
            _ => None,
        }
    }

    /// Aggregate [totals][Totals] and [max][Max] from a list of files
    fn aggregate(files: &[File]) -> (Totals, Max) {
        let mut total = Totals::default();
        let mut max = Max::default();
        for file in files {
            total.add(file);
            max.track(file);
        }
        (total, max)
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn scan_parallel_matches_serial() {
        let dir = tempdir().unwrap();

        fs::write(dir.path().join("one.txt"), "hello world\n").unwrap();
        fs::write(dir.path().join("two.txt"), "one two three\nfour\n").unwrap();

        let paths = [dir.path()];

        let serial = Scanner::new().scan(&paths).unwrap();
        let parallel = Scanner::new().run_parallel(true).scan(&paths).unwrap();

        assert_eq!(serial.total.files, parallel.total.files);
        assert_eq!(serial.total.lines, parallel.total.lines);
        assert_eq!(serial.total.words, parallel.total.words);
        assert_eq!(serial.total.chars, parallel.total.chars);
        assert_eq!(serial.total.bytes, parallel.total.bytes);

        assert_eq!(serial.max.lines, parallel.max.lines);
        assert_eq!(serial.max.words, parallel.max.words);
        assert_eq!(serial.max.chars, parallel.max.chars);
        assert_eq!(serial.max.bytes, parallel.max.bytes);
    }

    #[test]
    fn scan_parallel_respects_exclusions() {
        let dir = tempdir().unwrap();

        fs::write(dir.path().join("include.txt"), "keep me\n").unwrap();
        fs::write(dir.path().join("exclude.log"), "ignore me\n").unwrap();

        let paths = [dir.path()];

        let exclude_glob = globset::Glob::new("*.log").unwrap();
        let exclude_globset = globset::GlobSetBuilder::new()
            .add(exclude_glob)
            .build()
            .unwrap();

        let serial = Scanner::new()
            .exclude(exclude_globset.clone())
            .scan(&paths)
            .unwrap();

        let parallel = Scanner::new()
            .run_parallel(true)
            .exclude(exclude_globset)
            .scan(&paths)
            .unwrap();

        assert_eq!(serial.total.files, 1);
        assert_eq!(parallel.total.files, 1);
        assert_eq!(serial.total.bytes, parallel.total.bytes);
    }
}
