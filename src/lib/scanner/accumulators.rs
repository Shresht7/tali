use serde::Serialize;

use super::File;

// ------------
// ACCUMULATORS
// ------------

/// Represents the accumulated total number of lines, words, chars and bytes in [`ScanResults`]
#[derive(Debug, Default, Serialize)]
pub struct Totals {
    pub files: usize,
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: u64,
}

impl Totals {
    /// Add the [`File`] statistics to the totals accumulator
    pub(crate) fn add(&mut self, file: &File) {
        self.files += 1;
        self.lines += file.lines;
        self.words += file.words;
        self.chars += file.chars;
        self.bytes += file.bytes;
    }
}

/// Represents the max values for the number of lines, words, chars and bytes in [`ScanResults`]
#[derive(Debug, Default, Serialize)]
pub struct Max {
    pub lines: usize,
    pub words: usize,
    pub chars: usize,
    pub bytes: u64,
}

impl Max {
    /// Update the max values for the number of lines, words, chars and bytes by comparing it with a [`File`]
    pub(crate) fn track(&mut self, file: &File) {
        self.lines = self.lines.max(file.lines);
        self.words = self.words.max(file.words);
        self.chars = self.chars.max(file.chars);
        self.bytes = self.bytes.max(file.bytes);
    }
}
