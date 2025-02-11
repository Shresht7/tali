use clap::Parser;

/// Simple CLI to count the number of lines of code in a project
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// The path to scan (defaults to the current directory)
    #[arg(default_value = ".")]
    paths: Vec<std::path::PathBuf>,
}

/// The main entry-point of the application
fn main() -> std::io::Result<()> {
    // Parse the command-line arguments
    let args = Args::parse();
    // Run the main logic with the given command-line arguments
    if let Err(e) = run(&args) {
        eprintln!("Error: {}", e);
        std::process::exit(1)
    }
    Ok(())
}

/// Run the main logic of the application
fn run(args: &Args) -> std::io::Result<()> {
    let results = loc::scan(&args.paths);
    println!("{:?}", results);
    Ok(())
}
