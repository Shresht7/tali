#[derive(Debug)]
pub struct File {
    path: std::path::PathBuf,
    lines: usize,
}

impl File {
    pub fn from_path(path: &std::path::Path) -> File {
        File {
            path: path.to_path_buf(),
            lines: 0,
        }
    }
}
