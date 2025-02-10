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

    // Build a directory walker that respects `.gitignore` and other hidden files
    let walker = ignore::WalkBuilder::new(&args.directory).build();

    // Iterate over all the results
    for result in walker {
        match result {
            Ok(entry) if entry.path().is_file() => {
                println!("{}", entry.path().display());
            }
            Ok(_) => {}                          // Ignore directories and symlinks
            Err(e) => eprintln!("Error: {}", e), // Report errors
        }
    }
    Ok(())
}
