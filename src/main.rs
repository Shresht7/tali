use clap::Parser;

mod cli;

/// The main entry-point of the application
fn main() -> std::io::Result<()> {
    let args = cli::Args::parse();
    run(&args)?;
    Ok(())
}

/// Run the main logic of the application by scanning the provided paths and then displaying the results.
fn run(args: &cli::Args) -> std::io::Result<()> {
    // Scan the paths for the metrics
    let results = tali::scan(&args.paths)?;

    // Create a default display configuration
    let mut display = tali::Display::default();

    // If all the flags are false, then do nothing and just use the defaults
    let show_all = vec![args.lines, args.words, args.chars, args.bytes]
        .iter()
        .all(|toggle| toggle == &false);

    // If even a single flag was specified, then adhere to that selection and override the defaults
    if !show_all {
        display
            .lines(args.lines)
            .words(args.words)
            .chars(args.chars)
            .bytes(args.bytes);
    }

    // Print the formatted output
    println!("{}", display.display(&results));
    Ok(())
}
