/// A builder for constructing [`Table`][super::Table] instance from raw text input
#[derive(Debug)]
pub struct TableBuilder {
    // Raw input string containing the table data
    contents: String,
    // Whether the first row is treated as a header
    has_header: bool,
    // Whether the last row is treater as a footer
    has_footer: bool,
    // Delimtier to use for splitting columns (defaults to \t)
    delimiter: char,
}

impl Default for TableBuilder {
    /// Initializes a default [`TableBuilder`] that allows for maximum flexibility by the virtue of chaining methods.
    fn default() -> Self {
        Self {
            contents: String::new(),
            delimiter: '\t',
            has_header: false,
            has_footer: false,
        }
    }
}

impl TableBuilder {
    /// Creates a new [`TableBuilder`] with the given input string and delimiter
    pub fn new(input: impl Into<String>, delimiter: char) -> Self {
        Self {
            contents: input.into(),
            delimiter,
            ..Default::default()
        }
    }

    /// Set the raw contents of the table
    pub fn from(&mut self, contents: impl Into<String>) -> &mut Self {
        self.contents = contents.into();
        self
    }

    /// Sets the delimiter for parsing columns
    pub fn with_delimiter(&mut self, delimiter: char) -> &mut Self {
        self.delimiter = delimiter;
        self
    }

    /// Marks whether the first row should be treated as a header
    pub fn with_header(&mut self, yes: bool) -> &mut Self {
        self.has_header = yes;
        self
    }

    /// Marks whether the last row should be treated as a footer
    pub fn with_footer(&mut self, yes: bool) -> &mut Self {
        self.has_footer = yes;
        self
    }

    /// Constructs a [`TableBuilder`] from a CSV-formatted string
    pub fn from_csv(input: impl Into<String>) -> Self {
        Self::new(input, ',')
    }

    /// Constructs a [`TableBuilder`] from a TSV-formatted string
    pub fn from_tsv(input: impl Into<String>) -> Self {
        Self::new(input, '\t')
    }

    /// Builds a [`Table`][super::Table] from the provided configuration
    pub fn build(&self) -> super::Table {
        // Builds the rows
        let mut rows = self
            .contents
            .lines()
            .map(|line| {
                line.split(self.delimiter)
                    .map(|w| w.trim().to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        // Builds the header
        let header = if self.has_header && !rows.is_empty() {
            rows.remove(0)
        } else {
            Vec::new()
        };
        // Builds the footer
        let footer = if self.has_footer {
            rows.pop().unwrap_or(Vec::new())
        } else {
            Vec::new()
        };

        let mut table = super::Table {
            header,
            rows,
            footer,
            ..Default::default()
        };
        table.columns.mark_for_recalc(); // Ensure column widths are calculated when they're needed
        table
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_builder_defaults() {
        let builder = TableBuilder::default();
        assert_eq!(builder.contents, "");
        assert_eq!(builder.delimiter, '\t');
        assert!(!builder.has_header);
        assert!(!builder.has_footer);
    }

    #[test]
    fn test_builder_from_csv() {
        let builder = TableBuilder::from_csv("a,b,c\nd,e,f");
        assert_eq!(builder.delimiter, ',');
    }

    #[test]
    fn test_builder_from_tsv() {
        let builder = TableBuilder::from_tsv("a\tb\tc\nd\te\tf");
        assert_eq!(builder.delimiter, '\t');
    }

    #[test]
    fn test_builder_with_header_footer() {
        let mut builder = TableBuilder::new("a,b,c\nd,e,f\ng,h,i", ',');
        builder.with_header(true).with_footer(true);
        assert!(builder.has_header);
        assert!(builder.has_footer);
    }

    #[test]
    fn test_builder_build() {
        let table = TableBuilder::from_csv("H1,H2\na,b\nc,d")
            .with_header(true)
            .build();
        assert_eq!(table.header, vec!["H1", "H2"]);
        assert_eq!(table.rows, vec![vec!["a", "b"], vec!["c", "d"]]);
    }
}
