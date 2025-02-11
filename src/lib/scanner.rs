use std::collections::HashMap;

use crate::{file::File, language::Language, table::Alignment, table::Table};

#[derive(Debug)]
pub struct ScanResults {
    pub total: usize,
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
            .with_footer(vec![
                "Total".into(),
                total_no_of_files.to_string(),
                self.total.to_string(),
            ])
            .with_alignments(vec![Alignment::Left, Alignment::Center, Alignment::Right]);
        res.display()
    }
}

/// A helper function to color a string according to the language's color
fn color(language: &Language, text: &str) -> String {
    let (r, g, b) = language.color();
    format!("\u{001b}[38;2;{};{};{}m{}\u{001b}[0m", r, g, b, text)
}

pub fn scan<P: AsRef<std::path::Path>>(paths: &Vec<P>) -> std::io::Result<ScanResults> {
    let mut files = Vec::new();
    for path in paths {
        if path.as_ref().is_file() {
            files.push(File::from_path(path)?)
        } else {
            // Build a directory walker that respects `.gitignore` and other hidden files
            let walker = ignore::WalkBuilder::new(&path).build();

            // Iterate over all the results
            for result in walker {
                match result {
                    Ok(entry) if entry.path().is_file() => {
                        let file = File::from_path(entry.path())?;
                        files.push(file);
                    }
                    Ok(_) => {} // Ignore directories and symlinks
                    Err(e) => eprintln!("Error: {}", e), // Report errors
                }
            }
        }
    }

    let total = 0;
    Ok(ScanResults { files, total })
}
