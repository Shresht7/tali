fn main() -> std::io::Result<()> {
    let root = std::path::Path::new(".");
    let files = loc::walk(root);
    for file in files {
        println!("{}", file.display());
    }
    Ok(())
}
