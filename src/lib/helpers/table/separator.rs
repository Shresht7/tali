/// Defines the separators used in a table for formatting
#[derive(Debug)]
pub struct Separator {
    /// The string used for horizontal separation (e.g., column spacing)
    pub horizontal: String,
    /// The string used for vertical separation (e.g., row dividers)
    pub vertical: String,
}

impl Default for Separator {
    fn default() -> Self {
        Self {
            horizontal: "    ".into(), // Default to four spaces for horizontal spacing
            vertical: "-".into(),      // Default to a single dash for vertical dividers
        }
    }
}

impl super::Table {
    /// Sets a custom horizontal separator (e.g., column spacing)
    pub fn with_horizontal_separator(&mut self, separator: impl Into<String>) -> &mut Self {
        self.separator.horizontal = separator.into();
        self
    }

    /// Sets a custom vertical separator (e.g., row dividers).
    pub fn with_vertical_separator(&mut self, separator: impl Into<String>) -> &mut Self {
        self.separator.vertical = separator.into();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_separator_default() {
        let separator = Separator::default();
        assert_eq!(separator.horizontal, "    ");
        assert_eq!(separator.vertical, "-");
    }

    #[test]
    fn test_with_horizontal_separator() {
        let mut table = Table::default();
        table.with_horizontal_separator("|");
        assert_eq!(table.separator.horizontal, "|");
    }

    #[test]
    fn test_with_vertical_separator() {
        let mut table = Table::default();
        table.with_vertical_separator("#");
        assert_eq!(table.separator.vertical, "#");
    }
}
