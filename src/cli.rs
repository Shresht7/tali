use clap::Parser;

use std::io::IsTerminal;

use tali::{output::Config, scanner::SortOrder};

/// A structural representation of the command-line arguments
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// The paths to scan (defaults to the current directory or STDIN (if being redirected))
    pub paths: Vec<String>,

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

    /// Show visualization
    #[clap(short, long, aliases = ["graph", "vis"])]
    pub visualization: bool,

    /// The output format
    #[clap(short, long, default_value = "table")]
    pub format: tali::output::Format,

    /// Disable the header row
    #[clap(long, default_value_t = false)]
    pub no_header: bool,

    /// Disable the footer row
    #[clap(long, default_value_t = false)]
    pub no_footer: bool,

    /// Disable the table columns
    #[clap(long, default_value_t = false)]
    pub no_align: bool,

    /// Disable ANSI colors
    #[clap(long, alias="plain", default_value_t = std::env::var("NO_COLOR").is_ok_and(|v| v.to_lowercase() == "true"))]
    pub no_color: bool,

    /// Sort on category
    #[clap(long, default_value = "bytes")]
    pub sort: String,

    /// The order in which to sort
    #[clap(long, default_value = "descending")]
    pub sort_order: SortOrder,

    /// Scan hidden files
    #[clap(short = 'a', long, alias = "all")]
    pub hidden: bool,
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
            self.visualization,
        ]
        .iter()
        .all(|toggle| *toggle == false);

        if show_all {
            self.language = true;
            self.lines = true;
            self.words = true;
            self.chars = true;
            self.bytes = true;
            self.visualization = true;
        }

        self
    }
}

impl From<&Args> for Config {
    fn from(args: &Args) -> Self {
        Self {
            group_by_language: args.group,
            path: !args.group,
            language: args.language,
            lines: args.lines,
            words: args.words,
            chars: args.chars,
            bytes: args.bytes,
            visualization: args.visualization,
            use_colors: !args.no_color,
            format: args.format,
            header: !args.no_header,
            footer: !args.no_footer,
            alignment: !args.no_align,
            sort_by: args.sort.clone(),
            sort_order: args.sort_order,
        }
    }
}
