use super::ansi;

mod alignment;
pub use alignment::*;

mod separator;
pub use separator::*;

mod columns;
pub use columns::*;

mod iterator;

mod builder;
pub use builder::*;

/// Represents a table with headers, rows, and footers, along
/// with configurable column alignments and separators.
#[derive(Debug, Default)]
pub struct Table {
    header: Vec<String>,        // Stores the header row
    rows: Vec<Vec<String>>,     // Stores the table data
    footer: Vec<String>,        // Stores the footer row
    separator: Separator,       // Describes the column and row separators
    columns: Columns,           // Manages column width calculations
    alignments: Vec<Alignment>, // Stores column alignments
}

impl Table {
    /// Returns a new [`TableBuilder`] instance for constructing a table
    pub fn builder() -> TableBuilder {
        TableBuilder::default()
    }

    /// Construct a [`Table`] from a CSV-formatted string
    pub fn from_csv(input: impl Into<String>) -> Self {
        TableBuilder::from_csv(input).build()
    }

    /// Construct a [`Table`] from a TSV-formatted string
    pub fn from_tsv(input: impl Into<String>) -> Self {
        TableBuilder::from_tsv(input).build()
    }

    /// Sets the [header][Table::header] row of the table
    pub fn with_header(&mut self, header: Vec<String>) -> &mut Self {
        self.header = header;
        self.columns.mark_for_recalc();
        self
    }

    /// Sets the [footer][Table::footer] row of the table
    pub fn with_footer(&mut self, footer: Vec<String>) -> &mut Self {
        self.footer = footer;
        self.columns.mark_for_recalc();
        self
    }

    /// Configure the column alignments
    pub fn with_alignments(&mut self, alignments: Vec<Alignment>) -> &mut Self {
        self.alignments = alignments;
        self
    }

    /// Adds a row to the table
    pub fn add_row(&mut self, row: Vec<String>) -> &mut Self {
        self.rows.push(row);
        self.columns.mark_for_recalc();
        self
    }

    /// Builds a vertical separator
    fn format_vertical_separator(&self) -> String {
        let sep_v = &self.separator.vertical;
        let sep_h = &self.separator.horizontal;
        self.columns
            .into_iter()
            .map(|w| sep_v.repeat(*w))
            .collect::<Vec<_>>()
            .join(&sep_v.repeat(sep_h.len()))
            + &sep_v
            + "\n"
    }

    /// Formats a single cell with the appropriate alignment
    fn format_cell(&self, text: &str, width: usize, alignment: Option<&Alignment>) -> String {
        let visible_width = ansi::visible_width(text);
        let width = if visible_width < width {
            text.len() + (width - visible_width)
        } else {
            width
        };
        let res = match alignment {
            Some(&Alignment::Left) | None => format!("{:<width$}", text, width = width),
            Some(&Alignment::Center) => format!("{:^width$}", text, width = width),
            Some(&Alignment::Right) => format!("{:>width$}", text, width = width),
        };
        res
    }

    /// Formats a row of data for display
    fn format_row(&self, row: &Vec<String>) -> String {
        let mut res = String::new();
        for (i, cell) in row.iter().enumerate() {
            res.push_str(&self.format_cell(
                cell,
                self.columns.get_or(i, ansi::visible_width(cell)),
                self.alignments.get(i),
            ));
            res.push_str(&self.separator.horizontal);
        }
        res.push_str("\n");
        res
    }

    /// Generates a formatted string representation of the table
    pub fn display(&mut self) -> String {
        let mut res = String::new();

        // Calculate column widths
        let iter = self.into_iter().cloned().collect::<Vec<_>>();
        self.columns.calculate(iter);

        // Format Header
        if !self.header.is_empty() {
            res.push_str(&self.format_row(&self.header));
            res.push_str(&self.format_vertical_separator());
        }

        // Format Rows
        for row in &self.rows {
            res.push_str(&self.format_row(row));
        }

        // Format Footer
        if !self.footer.is_empty() {
            res.push_str(&self.format_vertical_separator());
            res.push_str(&self.format_row(&self.footer));
        }

        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_table_from_csv() {
        let input = "a,b,c\nd,e,f";
        let table = Table::from_csv(input);
        assert_eq!(table.rows, vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]);
    }

    #[test]
    fn test_table_from_tsv() {
        let input = "a\tb\tc\nd\te\tf";
        let table = Table::from_tsv(input);
        assert_eq!(table.rows, vec![vec!["a", "b", "c"], vec!["d", "e", "f"]]);
    }

    #[test]
    fn test_with_header() {
        let mut table = Table::from_csv("");
        table.with_header(vec!["Col1".to_string(), "Col2".to_string()]);
        assert_eq!(table.header, vec!["Col1", "Col2"]);
    }

    #[test]
    fn test_with_footer() {
        let mut table = Table::from_csv("");
        table.with_footer(vec!["Total".to_string(), "42".to_string()]);
        assert_eq!(table.footer, vec!["Total", "42"]);
    }

    #[test]
    fn test_with_alignments() {
        let mut table = Table::from_csv("");
        table.with_alignments(vec![Alignment::Left, Alignment::Right]);
        assert_eq!(table.alignments, vec![Alignment::Left, Alignment::Right]);
    }

    #[test]
    fn test_with_separators() {
        let mut table = Table::from_csv("");
        table
            .with_horizontal_separator("---")
            .with_vertical_separator("|");
        assert_eq!(table.separator.horizontal, "---");
        assert_eq!(table.separator.vertical, "|");
    }

    #[test]
    fn test_add_row() {
        let mut table = Table::from_csv("");
        table.add_row(vec!["Data1".to_string(), "Data2".to_string()]);
        assert_eq!(table.rows, vec![vec!["Data1", "Data2"]]);
    }

    #[test]
    fn test_display_output() {
        let mut table = Table::from_csv("a,b\nc,d");
        table.with_header(vec!["H1".to_string(), "H2".to_string()]);
        table.with_footer(vec!["F1".to_string(), "F2".to_string()]);
        let output = table.display();
        assert!(output.contains("H1"));
        assert!(output.contains("a"));
        assert!(output.contains("F1"));
    }
}
