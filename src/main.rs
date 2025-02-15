use clap::Parser;

mod cli;

/// The main entry-point of the application
fn main() -> std::io::Result<()> {
    let args = cli::Args::parse().process();
    run(&args)?;
    Ok(())
}

/// Run the main logic of the application by scanning the provided paths and then displaying the results.
fn run(args: &cli::Args) -> std::io::Result<()> {
    // Scan the paths for the metrics
    let results = tali::scanner::scan(&args.paths)?;

    // Create a default display configuration
    let mut display = tali::display::Display::default();
    display
        .lines(args.lines)
        .words(args.words)
        .chars(args.chars)
        .bytes(args.bytes)
        .color(!args.no_color)
        .visualization(args.visualization);

    // Print the formatted output
    println!("{}", display.display(&results));
    Ok(())
}
