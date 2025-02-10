use clap::Parser;

/// Simple CLI to count the number of lines of code in a project
#[derive(Debug, Parser)]
#[command(author, version, about)]
struct Args {
    /// The directory to scan (defaults to the current directory)
    #[arg(default_value = ".")]
    directory: String,
}

fn main() -> std::io::Result<()> {
    // Parse the command-line arguments
    let args = Args::parse();

    // Perform the scanning operation
    let result = loc::scan(&args.directory)?;

    // Print the results
    println!("{:#?}", result);
    Ok(())
}
