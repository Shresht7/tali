fn main() -> std::io::Result<()> {
    let directory = ".";
    let walker = ignore::WalkBuilder::new(&directory).build();
    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();
                if !path.is_file() {
                    continue;
                }
                println!("{}", path.display());
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
    Ok(())
}
