use crate::file::File;

#[derive(Debug)]
pub struct ScanResults {
    pub total: usize,
    pub files: Vec<File>,
}

pub fn scan(dir: &str) -> std::io::Result<ScanResults> {
    // Build a directory walker that respects `.gitignore` and other hidden files
    let walker = ignore::WalkBuilder::new(&dir).build();

    // Collect the file info in a vector
    let mut files = Vec::new();
    let mut total = 0;

    // Iterate over all the results
    for result in walker {
        match result {
            Ok(entry) if entry.path().is_file() => {
                let file = File::from_path(entry.path())?;
                total += file.lines;
                files.push(file);
            }
            Ok(_) => {}                          // Ignore directories and symlinks
            Err(e) => eprintln!("Error: {}", e), // Report errors
        }
    }

    Ok(ScanResults { files, total })
}

impl std::fmt::Display for ScanResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "")?;
        for file in &self.files {
            writeln!(
                f,
                "[{}] {}: {}",
                file.language,
                file.path.display(),
                file.lines,
            )?;
        }
        writeln!(f, "")?;
        writeln!(f, "Total: {}", self.total)?;
        Ok(())
    }
}
