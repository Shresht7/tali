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
        self.files += file.count;
        self.lines += file.lines;
        self.words += file.words;
        self.chars += file.chars;
        self.bytes += file.bytes;
    }

    pub(crate) fn merge(&mut self, rhs: &Totals) {
        self.files += rhs.files;
        self.lines += rhs.lines;
        self.words += rhs.words;
        self.chars += rhs.chars;
        self.bytes += rhs.bytes;
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

    /// Merge with another [Max] accumulator by taking the max values for each field
    pub(crate) fn merge(&mut self, rhs: &Max) {
        self.lines = self.lines.max(rhs.lines);
        self.words = self.words.max(rhs.words);
        self.chars = self.chars.max(rhs.chars);
        self.bytes = self.bytes.max(rhs.bytes);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_max_merge() {
        let mut lhs = Max {
            lines: 5,
            words: 3,
            chars: 10,
            bytes: 20,
        };

        let rhs = Max {
            lines: 6,
            words: 2,
            chars: 12,
            bytes: 15,
        };

        lhs.merge(&rhs);

        assert_eq!(lhs.lines, 6);
        assert_eq!(lhs.words, 3);
        assert_eq!(lhs.chars, 12);
        assert_eq!(lhs.bytes, 20);
    }

    #[test]
    fn test_max_merge_with_default() {
        let mut lhs = Max::default();

        let rhs = Max {
            lines: 10,
            words: 20,
            chars: 30,
            bytes: 40,
        };

        lhs.merge(&rhs);

        assert_eq!(lhs.lines, 10);
        assert_eq!(lhs.words, 20);
        assert_eq!(lhs.chars, 30);
        assert_eq!(lhs.bytes, 40);
    }
}
