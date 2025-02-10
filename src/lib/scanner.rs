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

    // Iterate over all the results
    for result in walker {
        match result {
            Ok(entry) if entry.path().is_file() => files.push(File::from_path(entry.path())?), // Record the file
            Ok(_) => {}                          // Ignore directories and symlinks
            Err(e) => eprintln!("Error: {}", e), // Report errors
        }
    }

    Ok(ScanResults { files, total: 0 })
}
