/// Recursively walk the given [`path`][std::path::Path] and collect all [file-paths][std::path::PathBuf] in a vector
pub fn walk(dir: &std::path::Path) -> Vec<std::path::PathBuf> {
    let mut res = Vec::new(); // Vector to collect all the results

    // Stack of directories to process; start with the given directory
    // We maintain our own stack to prevent stack-overflow with deep recursion since directory nesting is unpredictable
    let mut dirs = vec![dir.to_path_buf()];

    while let Some(current_dir) = dirs.pop() {
        // Try to read the current directory; If that fails, ignore the error and continue
        let entries = match std::fs::read_dir(&current_dir) {
            Ok(entries) => entries,
            Err(e) => {
                eprintln!("Error reading directory {}: {}", current_dir.display(), e);
                continue;
            }
        };

        for entry_result in entries {
            let entry = match entry_result {
                Ok(entry) => entry,
                Err(e) => {
                    eprintln!(
                        "Error reading directory entry {}: {}",
                        current_dir.display(),
                        e
                    );
                    continue;
                }
            };

            let path = entry.path();
            let metadata = match std::fs::symlink_metadata(&path) {
                Ok(m) => m,
                Err(e) => {
                    eprintln!("Error reading metadata {}: {}", path.display(), e);
                    continue;
                }
            };

            if metadata.is_symlink() {
                // Skip symbolic links to prevent cycles
                continue;
            } else if metadata.is_dir() {
                // Push directories onto the stack for later processing
                dirs.push(path);
            } else {
                // Add files to the result
                res.push(path);
            }
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::io::Write;

    // TEMP DIR UTILITY
    // ----------------

    struct TempDir {
        pub dir: std::path::PathBuf,
    }

    impl TempDir {
        /// Creates a unique temporary directory for testing
        fn create(prefix: &str) -> std::io::Result<Self> {
            let mut dir = std::env::temp_dir();
            // Use the current-time to generate a unique folder name
            let time = std::time::SystemTime::now().elapsed().unwrap().as_nanos();
            dir.push(format!("{}_{}", prefix, time));
            std::fs::create_dir_all(&dir)?;
            Ok(Self { dir })
        }

        /// Cleans up the created temporary directory
        fn cleanup(&self) -> std::io::Result<()> {
            std::fs::remove_dir_all(&self.dir)
        }
    }

    // Try to automatically cleanup the temporary directory when it goes out of scope
    impl Drop for TempDir {
        fn drop(&mut self) {
            if self.dir.exists() {
                self.cleanup().expect("Failed to cleanup directory");
            }
        }
    }

    // TESTS
    // -----

    #[test]
    fn test_walk_basic() -> std::io::Result<()> {
        // Perform Setup
        let temp_dir = TempDir::create("test_walk_basic")?;
        let subdir = temp_dir.dir.join("subdir");
        std::fs::create_dir_all(&subdir)?;
        let file1 = temp_dir.dir.join("file1.txt");
        let file2 = subdir.join("file2.txt");
        std::fs::File::create(&file1)?.write_all(b"Hello")?;
        std::fs::File::create(&file2)?.write_all(b"World")?;

        // Perform Test
        let files = walk(&temp_dir.dir);
        let files = extract_file_names(files); // Extracts files names and sorts them
        assert_eq!(files.len(), 2);
        assert_eq!(files[0], "file1.txt");
        assert_eq!(files[1], "file2.txt");

        // Perform Cleanup
        temp_dir.cleanup()?;
        Ok(())
    }

    #[test]
    fn test_walk_empty() -> std::io::Result<()> {
        // Perform Setup
        let temp_dir = TempDir::create("test_walk_empty")?;

        // Perform Test
        let files = walk(&temp_dir.dir);
        assert_eq!(files.len(), 0);

        // Perform Cleanup
        temp_dir.cleanup()?;
        Ok(())
    }

    // HELPERS
    // -------

    /// Converts a vector of [`PathBuf`][std::path::PathBuf]s to a sorted vector of file name strings.
    /// Files that have no valid UTF-8 name are skipped.
    fn extract_file_names(paths: Vec<std::path::PathBuf>) -> Vec<String> {
        let mut names: Vec<String> = paths
            .into_iter()
            .filter_map(|p| {
                p.file_name()
                    .and_then(|os_str| os_str.to_str().map(String::from))
            })
            .collect();
        names.sort();
        names
    }
}
