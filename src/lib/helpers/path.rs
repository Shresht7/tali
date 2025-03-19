/// Display format for the path
pub fn display(path: &std::path::Path) -> String {
    let path = path.display().to_string();
    let path = path.trim_start_matches(".\\").trim_start_matches("./");
    path.to_string()
}
