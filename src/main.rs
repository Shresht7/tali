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

    // Setup the display configuration from the command-line arguments
    let display = tali::display::Display::from(args);

    // Print the formatted output
    println!("{}", display.display(&results));
    Ok(())
}
