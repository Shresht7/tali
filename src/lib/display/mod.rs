use crate::scanner::ScanResults;

mod table;
use table::*;

pub trait Formatter {
    fn format(&self, results: &ScanResults, config: &Display) -> String;
}

#[derive(Debug)]
pub enum Format {
    Table,
}

#[derive(Debug)]
pub struct Display {
    pub group_by_language: bool,
    pub path: bool,
    pub lines: bool,
    pub words: bool,
    pub chars: bool,
    pub bytes: bool,
    pub language: bool,
    pub visualization: bool,
    pub use_colors: bool,
    format: Format,
}

impl Default for Display {
    fn default() -> Self {
        Self {
            group_by_language: false,
            path: true,
            lines: true,
            words: true,
            chars: true,
            bytes: true,
            language: true,
            visualization: true,
            use_colors: true,
            format: Format::Table,
        }
    }
}

impl Display {
    /// Sets whether to group output by language
    pub fn group_by_language(&mut self, yes: bool) -> &mut Self {
        self.group_by_language = yes;
        self
    }

    /// Sets whether to show file path
    pub fn path(&mut self, yes: bool) -> &mut Self {
        self.path = yes;
        self
    }

    /// Sets whether to show the line count
    pub fn lines(&mut self, yes: bool) -> &mut Self {
        self.lines = yes;
        self
    }

    /// Sets whether to show the word count
    pub fn words(&mut self, yes: bool) -> &mut Self {
        self.words = yes;
        self
    }

    /// Sets whether to show the character count
    pub fn chars(&mut self, yes: bool) -> &mut Self {
        self.chars = yes;
        self
    }

    /// Sets whether to show byte count
    pub fn bytes(&mut self, yes: bool) -> &mut Self {
        self.bytes = yes;
        self
    }

    pub fn color(&mut self, yes: bool) -> &mut Self {
        self.use_colors = yes;
        self
    }

    pub fn visualization(&mut self, yes: bool) -> &mut Self {
        self.visualization = yes;
        self
    }

    pub fn display(&self, results: &ScanResults) -> String {
        match self.format {
            Format::Table => TableFormatter::default().format(results, &self),
        }
    }
}
