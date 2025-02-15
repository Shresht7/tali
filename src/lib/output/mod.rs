use crate::scanner::ScanResults;

mod json;
use json::*;
mod table;
use table::*;

pub trait Formatter {
    fn format(&self, results: &ScanResults, config: &Config) -> String;
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Table,
    JSON,
}

impl std::str::FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "table" => Ok(Self::Table),
            "json" => Ok(Self::JSON),
            x => Err(format!("Unsupported Format: {x}")),
        }
    }
}

#[derive(Debug)]
pub struct Config {
    pub group_by_language: bool,
    pub path: bool,
    pub lines: bool,
    pub words: bool,
    pub chars: bool,
    pub bytes: bool,
    pub language: bool,
    pub visualization: bool,
    pub use_colors: bool,
    pub format: Format,
}

impl Default for Config {
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

impl Config {
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
}

pub fn display(results: &ScanResults, config: &Config) -> String {
    match config.format {
        Format::Table => TableFormatter::default().format(results, config),
        Format::JSON => JSONFormatter::default().format(results, config),
    }
}
