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
    pub path: bool,

    pub language: bool,
    pub lines: bool,
    pub words: bool,
    pub chars: bool,
    pub bytes: bool,
    pub group_by_language: bool,

    pub sort_by: String,
    pub sort_order: SortOrder,

    pub graph: bool,
    pub graph_by: String,
    pub graph_fill: String,
    pub graph_blank: String,
    pub graph_size: usize,

    pub header: bool,
    pub footer: bool,
    pub alignment: bool,

    pub use_colors: bool,
    pub format: Format,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            path: true,

            language: true,
            lines: true,
            words: true,
            chars: true,
            bytes: true,
            group_by_language: false,

            sort_by: "bytes".into(),
            sort_order: SortOrder::Descending,

            graph: true,
            graph_by: "bytes".into(),
            graph_fill: "▬".into(),
            graph_blank: " ".into(),
            graph_size: 20,

            header: true,
            footer: true,
            alignment: true,

            use_colors: true,
            format: Format::Table,
        }
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
            config.graph = false;
            config.use_colors = false;
            config.alignment = false;
            TableFormatter::default().format(&results, &config)
        }
        Format::JSON => JSONFormatter::default().format(&results, &config),
        Format::TSV => DelimiterFormatter::with("\t").format(&results, &config),
        Format::CSV => DelimiterFormatter::with(",").format(&results, &config),
    }
}
