use clap::Parser;
use tali::display::Display;

/// A structural representation of the command-line arguments
#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    /// The paths to scan (defaults to the current directory)
    #[arg(default_value = ".")]
    pub paths: Vec<std::path::PathBuf>,

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

    /// Show visualization
    #[clap(short, long, aliases = ["graph", "vis"])]
    pub visualization: bool,

    /// Disable ANSI colors
    #[clap(short, long, alias="plain", default_value_t = std::env::var("NO_COLOR").is_ok_and(|v| v.to_lowercase() == "true"))]
    pub no_color: bool,
}

impl Args {
    pub fn process(mut self) -> Self {
        // If all the flags are false, then do nothing and just use the defaults
        let show_all = vec![
            self.lines,
            self.words,
            self.chars,
            self.bytes,
            self.visualization,
        ]
        .iter()
        .all(|toggle| *toggle == false);

        if show_all {
            self.lines = true;
            self.words = true;
            self.chars = true;
            self.bytes = true;
            self.visualization = true;
        }

        self
    }
}

impl From<&Args> for Display {
    fn from(args: &Args) -> Self {
        Self {
            group_by_language: false,
            path: true,
            language: true,
            lines: args.lines,
            words: args.words,
            chars: args.chars,
            bytes: args.bytes,
            visualization: args.visualization,
            use_colors: !args.no_color,
            format: tali::display::Format::Table,
        }
    }
}
