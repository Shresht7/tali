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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::helpers::language::Language;

    #[test]
    fn test_totals_add() {
        let mut totals = Totals::default();
        let file = File {
            path: "file".into(),
            lines: 1,
            words: 2,
            chars: 3,
            bytes: 4,
            language: Language::Unknown("".into()),
            count: 1,
        };
        totals.add(&file);
        assert_eq!(totals.lines, 1);
        assert_eq!(totals.words, 2);
        assert_eq!(totals.chars, 3);
        assert_eq!(totals.bytes, 4);
        assert_eq!(totals.files, 1);
    }

    #[test]
    fn test_max_track() {
        let mut max = Max::default();
        let file1 = File {
            path: "file1".into(),
            lines: 1,
            words: 3,
            chars: 7,
            bytes: 9,
            language: Language::Unknown("".into()),
            count: 1,
        };
        let file2 = File {
            path: "file2".into(),
            lines: 3,
            words: 12,
            chars: 4,
            bytes: 21,
            language: Language::Unknown("".into()),
            count: 1,
        };
        max.track(&file1);
        max.track(&file2);
        assert_eq!(max.lines, 3);
        assert_eq!(max.words, 12);
        assert_eq!(max.chars, 7);
        assert_eq!(max.bytes, 21);
    }
}
