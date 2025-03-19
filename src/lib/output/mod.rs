use crate::scanner::{ScanResults, SortOrder};

mod json;
use json::*;
mod table;
use table::*;
mod delimiter;
use delimiter::*;

pub trait Formatter {
    fn format(&self, results: &ScanResults, config: &Config) -> String;
}

#[derive(Debug, Clone, Copy)]
pub enum Format {
    Table,
    Plain,
    JSON,
    TSV,
    CSV,
}

impl std::str::FromStr for Format {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "table" => Ok(Self::Table),
            "plain" => Ok(Self::Plain),
            "json" => Ok(Self::JSON),
            "tsv" => Ok(Self::TSV),
            "csv" => Ok(Self::CSV),
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
    pub visualize: bool,
    pub use_colors: bool,
    pub format: Format,
    pub header: bool,
    pub footer: bool,
    pub alignment: bool,
    pub sort_by: String,
    pub graph_by: String,
    pub graph_fill: String,
    pub graph_blank: String,
    pub sort_order: SortOrder,
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
            visualize: true,
            use_colors: true,
            format: Format::Table,
            header: true,
            footer: true,
            alignment: true,
            sort_by: "bytes".into(),
            graph_by: "bytes".into(),
            graph_fill: "▬".into(),
            graph_blank: " ".into(),
            sort_order: SortOrder::Descending,
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

    pub fn visualize(&mut self, yes: bool) -> &mut Self {
        self.visualize = yes;
        self
    }
}

/// Formats the [`ScanResults`] based on the [`Config`] and returns the output
pub fn display(results: ScanResults, mut config: Config) -> String {
    // Reform ScanResults if we need to group by language
    let mut results = if config.group_by_language {
        results.group_by_language()
    } else {
        results
    };

    // Sort the results
    results.sort_by(&config.sort_by, &config.sort_order);

    // Chose the formatter based on the configuration
    match config.format {
        Format::Table => TableFormatter::default().format(&results, &config),
        Format::Plain => {
            config.header = false;
            config.footer = false;
            config.visualize = false;
            config.use_colors = false;
            config.alignment = false;
            TableFormatter::default().format(&results, &config)
        }
        Format::JSON => JSONFormatter::default().format(&results, &config),
        Format::TSV => DelimiterFormatter::with("\t").format(&results, &config),
        Format::CSV => DelimiterFormatter::with(",").format(&results, &config),
    }
}
