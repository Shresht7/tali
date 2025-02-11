pub fn scan_file<P: AsRef<std::path::Path>>(filepath: P) -> std::io::Result<()> {
    let file = loc::File::from_path(filepath)?;
    println!("{}\t{}\t{}", file.language, file.path.display(), file.lines);
    Ok(())
}
