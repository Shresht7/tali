#[derive(Debug)]
pub struct ScanResults {
    pub total: usize,
}

pub fn scan(dir: &str) -> ScanResults {
    // Build a directory walker that respects `.gitignore` and other hidden files
    let walker = ignore::WalkBuilder::new(&dir).build();

    // Iterate over all the results
    for result in walker {
        match result {
            Ok(entry) if entry.path().is_file() => {
                println!("{}", entry.path().display());
            }
            Ok(_) => {}                          // Ignore directories and symlinks
            Err(e) => eprintln!("Error: {}", e), // Report errors
        }
    }

    ScanResults { total: 0 }
}
