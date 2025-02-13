#[derive(Debug)]
pub struct Separator {
    pub horizontal: String,
    pub vertical: String,
}

impl Default for Separator {
    fn default() -> Self {
        Self {
            horizontal: "    ".into(),
            vertical: "-".into(),
        }
    }
}

impl super::Table {
    pub fn with_horizontal_separator(&mut self, separator: &str) -> &mut Self {
        self.separator.horizontal = separator.to_owned();
        self
    }

    pub fn with_vertical_separator(&mut self, separator: &str) -> &mut Self {
        self.separator.vertical = separator.to_owned();
        self
    }
}
