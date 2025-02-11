use clap::Parser;

mod cli;

/// The main entry-point of the application
fn main() -> std::io::Result<()> {
    // Parse the command-line arguments
    let args = cli::Args::parse();
    // Run the main logic with the given command-line arguments
    if let Err(e) = run(&args) {
        eprintln!("Error: {}", e);
        std::process::exit(1)
    }
    Ok(())
}

/// Run the main logic of the application
fn run(args: &cli::Args) -> std::io::Result<()> {
    let results = loc::scan(&args.paths)?;

    let mut display = loc::Display::default();

    // If all the flags are false, then do nothing and just use the defaults
    let all_modes = vec![args.lines, args.words, args.chars, args.bytes]
        .iter()
        .all(|toggle| toggle == &false);

    // If even a single flag was specified, then adhere to that selection and override the defaults
    if !all_modes {
        display
            .lines(args.lines)
            .words(args.words)
            .chars(args.chars)
            .bytes(args.bytes);
    }

    println!("{}", display.display(results));
    Ok(())
}
