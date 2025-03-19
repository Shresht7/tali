use serde::Serialize;

use std::collections::HashMap;

use crate::output::Metric;

use super::{File, Max, Totals};

// ------------
// SCAN RESULTS
// ------------

/// Represents the aggregate scan results
#[derive(Debug, Serialize)]
pub struct ScanResults {
    /// The collection of all file results containing information like the number of lines, words, chars and bytes
    pub files: Vec<File>,
    /// The aggregate total number of lines, words, chars and bytes
    pub total: Totals,
    /// The max values for the number of lines, words, chars, and bytes across the results
    pub max: Max,
}

impl ScanResults {
    /// Groups the files by language and returns a new [`ScanResults`] instance
    pub fn group_by_language(&self) -> ScanResults {
        // Group the files by language in a HashMap
        let mut groups = HashMap::new();
        for file in self.files.iter().cloned() {
            groups
                .entry(file.language.clone())
                .or_insert_with(Vec::new)
                .push(file)
        }

        // Create a new condensed ScanResult
        let mut files = Vec::new();
        let mut total = Totals::default();
        let mut max = Max::default();
        for lang in groups.keys() {
            if let Some(v) = groups.get(lang) {
                if let Some(file) = v.iter().cloned().reduce(|acc, e| acc + e) {
                    total.add(&file);
                    max.track(&file);
                    files.push(file)
                }
            }
        }
        ScanResults { files, total, max }
    }

    /// Sort the [`ScanResults`] files based on the given column and sort order
    pub fn sort_by(&mut self, category: Metric, order: &SortOrder) {
        match category {
            Metric::Lines => self.files.sort_by(|a, b| match order {
                SortOrder::Ascending => a.lines.cmp(&b.lines),
                SortOrder::Descending => b.lines.cmp(&a.lines),
            }),
            Metric::Words => self.files.sort_by(|a, b| match order {
                SortOrder::Ascending => a.words.cmp(&b.words),
                SortOrder::Descending => b.words.cmp(&a.words),
            }),
            Metric::Chars => self.files.sort_by(|a, b| match order {
                SortOrder::Ascending => a.chars.cmp(&b.chars),
                SortOrder::Descending => b.chars.cmp(&a.chars),
            }),
            Metric::Bytes | _ => self.files.sort_by(|a, b| match order {
                SortOrder::Ascending => a.bytes.cmp(&b.bytes),
                SortOrder::Descending => b.bytes.cmp(&a.bytes),
            }),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum SortOrder {
    Ascending,
    Descending,
}

impl std::str::FromStr for SortOrder {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "a" | "asc" | "ascending" => Ok(Self::Ascending),
            "d" | "desc" | "descending" => Ok(Self::Descending),
            x => Err(format!("Invalid sorting order: {x}")),
        }
    }
}
