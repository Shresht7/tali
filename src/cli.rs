use std::io::IsTerminal;

use clap::Parser;
use globset::{Glob, GlobSet, GlobSetBuilder};

use tali::{
    output::Config,
    scanner::{Scanner, SortOrder},
};

/// A structural representation of the command-line arguments
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    // ** === PATHS === **
    /// The paths to scan (defaults to the current directory or STDIN (if being redirected))
    pub paths: Vec<String>,

    // ** === COLUMN OPTIONS === **
    /// Show language
    #[clap(short('e'), long, aliases = ["lang", "kind", "type"])]
    pub language: bool,

    /// Show line count
    #[clap(short, long)]
    pub lines: bool,

    /// Show word count
    #[clap(short, long)]
    pub words: bool,

    /// Show char count
    #[clap(short, long)]
    pub chars: bool,

    /// Show byte count
    #[clap(short, long)]
    pub bytes: bool,

    /// Group the results by language
    #[clap(short, long, aliases=["compact", "overview"])]
    pub group: bool,

    // ** === SORTING OPTIONS === **
    /// Sort on category
    #[clap(long)]
    pub sort_by: Option<String>,

    /// The order in which to sort
    #[clap(long, default_value = "descending")]
    pub sort_order: SortOrder,

    // ** === GRAPH OPTIONS === **
    /// Show visualization
    #[clap(short = 'v', long, aliases = ["vis", "visualize", "visualization"])]
    pub graph: bool,

    /// The character to use for the graph
    #[clap(long, default_value = "▬")]
    pub graph_fill: String,

    /// The character to use for the empty spaces in the graph
    #[clap(long, default_value = " ")]
    pub graph_blank: String,

    /// The max column width alloted to the graph
    #[clap(long, default_value_t = 20)]
    pub graph_size: usize,

    /// The property to visualize in the graph
    #[clap(long)]
    pub graph_by: Option<String>,

    // ** === SCANNING OPTIONS === **
    /// The maximum depth to recurse when scanning
    #[clap(short = 'd', long)]
    pub max_depth: Option<usize>,

    #[clap(long, alias = "size-limit")]
    pub max_filesize: Option<u64>,

    /// Scan hidden files
    #[clap(short = 'a', long, alias = "all")]
    pub hidden: bool,

    /// Exclude files that match the pattern from the scan
    #[clap(long)]
    pub exclude: Option<String>,

    // ** === TABLE OPTIONS === **
    /// Disable the header row
    #[clap(long, default_value_t = false)]
    pub no_header: bool,

    /// Disable the footer row
    #[clap(long, default_value_t = false)]
    pub no_footer: bool,

    /// Disable the table columns
    #[clap(long, default_value_t = false)]
    pub no_align: bool,

    // ** === OUTPUT OPTIONS=== **
    /// Disable ANSI colors
    #[clap(long, alias="plain", default_value_t = std::env::var("NO_COLOR").is_ok_and(|v| v.to_lowercase() == "true"))]
    pub no_color: bool,

    /// The output format
    #[clap(short, long, default_value = "table")]
    pub format: tali::output::Format,
}

impl Args {
    pub fn process(mut self) -> Self {
        // If paths is empty, determine what the default behaviour should be
        if self.paths.is_empty() {
            // If STDIN is not a tty, assume input is being piped in...
            if !std::io::stdin().is_terminal() {
                // ... then set the default value to `-` to indicate that we want to scan STDIN
                self.paths.push("-".into())
            } else {
                // ... otherwise, set the default value to `.` to indicate that we want to scan the current directory
                self.paths.push(".".into())
            }
        }

        // If all the flags are false, then do nothing and just use the defaults
        let show_all = vec![
            self.language,
            self.lines,
            self.words,
            self.chars,
            self.bytes,
            self.graph,
        ]
        .iter()
        .all(|toggle| *toggle == false);

        if show_all {
            self.language = true;
            self.lines = true;
            self.words = true;
            self.chars = true;
            self.bytes = true;
            self.graph = true;
        }

        // Determine the default sort
        if self.sort_by.is_none() {
            if self.lines {
                self.sort_by = Some("lines".into())
            } else if self.words {
                self.sort_by = Some("words".into())
            } else if self.chars {
                self.sort_by = Some("chars".into())
            } else {
                self.sort_by = Some("bytes".into())
            }
        }

        self
    }
}

impl From<&Args> for Config {
    fn from(args: &Args) -> Self {
        let sort_by = args.sort_by.clone().unwrap_or("bytes".into());
        let graph_by = args.graph_by.clone().unwrap_or(sort_by.clone());
        Self {
            path: !args.group,

            language: args.language,
            lines: args.lines,
            words: args.words,
            chars: args.chars,
            bytes: args.bytes,
            group_by_language: args.group,

            graph: args.graph,
            graph_by,
            graph_fill: args.graph_fill.clone(),
            graph_blank: args.graph_blank.clone(),
            graph_size: args.graph_size,

            sort_by,
            sort_order: args.sort_order,

            header: !args.no_header,
            footer: !args.no_footer,
            alignment: !args.no_align,

            use_colors: !args.no_color,
            format: args.format,
        }
    }
}

impl From<&Args> for Scanner {
    fn from(args: &Args) -> Self {
        let mut scanner = Self::new()
            .ignore_hidden(!args.hidden)
            .max_filesize(args.max_filesize)
            .scan_depth(args.max_depth);

        if let Some(patterns) = &args.exclude {
            let exclude = build_glob_set(patterns);
            scanner = scanner.exclude(exclude);
        }

        scanner
    }
}

fn build_glob_set(patterns: &String) -> GlobSet {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns.split(",") {
        if let Ok(glob) = Glob::new(pattern.trim()) {
            builder.add(glob);
        }
    }
    builder.build().unwrap_or_else(|_| GlobSet::empty())
}
