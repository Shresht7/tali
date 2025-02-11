#[derive(Debug)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub struct TableWriter {
    headers: Vec<String>,
    rows: Vec<Vec<String>>,
    footer: Vec<String>,
    alignments: Vec<Alignment>,
    column_widths: Vec<usize>,
}

impl Default for TableWriter {
    fn default() -> Self {
        Self {
            headers: Vec::with_capacity(5),
            rows: Vec::with_capacity(5),
            footer: Vec::with_capacity(5),
            alignments: Vec::with_capacity(5),
            column_widths: Vec::with_capacity(5),
        }
    }
}

impl TableWriter {
    pub fn new(columns: usize) -> Self {
        Self {
            headers: Vec::with_capacity(columns),
            rows: Vec::with_capacity(columns),
            footer: Vec::with_capacity(columns),
            alignments: Vec::with_capacity(columns),
            column_widths: Vec::with_capacity(columns),
        }
    }

    pub fn with_headers(&mut self, headers: Vec<String>) -> &mut Self {
        self.headers = headers;
        self.column_widths = self.headers.iter().map(|h| h.len()).collect();
        self
    }

    pub fn with_alignment(&mut self, alignment: Vec<Alignment>) -> &mut Self {
        self.alignments = alignment;
        self
    }

    pub fn with_footer(&mut self, footer: Vec<String>) -> &mut Self {
        self.footer = footer;
        self
    }

    pub fn add_row(&mut self, row: Vec<String>) {
        for (i, cell) in row.iter().enumerate() {
            self.column_widths[i] = std::cmp::max(self.column_widths[i], cell.len());
        }
        self.rows.push(row)
    }

    fn format_cell(&self, text: &str, width: usize, alignment: Option<&Alignment>) -> String {
        match alignment {
            Some(Alignment::Left) | None => format!("{:<width$}", text, width = width),
            Some(Alignment::Center) => format!("{:^width$}", text, width = width),
            Some(Alignment::Right) => format!("{:>width$}", text, width = width),
        }
    }
}

impl std::fmt::Display for TableWriter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        // Print headers
        if self.headers.len() > 0 {
            for (i, header) in self.headers.iter().enumerate() {
                output.push_str(&self.format_cell(
                    header,
                    self.column_widths[i],
                    self.alignments.get(i),
                ));
                output.push_str(" | ");
            }
            output.push('\n');

            // Print separator
            output.push_str(
                &"-".repeat(
                    self.column_widths.iter().sum::<usize>() + (self.headers.len() * 3) - 2,
                ),
            );
            output.push('\n');
        }

        // Print rows
        for row in &self.rows {
            for (i, cell) in row.iter().enumerate() {
                output.push_str(&self.format_cell(
                    cell,
                    self.column_widths[i],
                    self.alignments.get(i),
                ));
                output.push_str(" | ");
            }
            output.push('\n');
        }

        // Print footer
        if self.footer.len() > 0 {
            // Print separator
            output.push_str(
                &"-".repeat(
                    self.column_widths.iter().sum::<usize>() + (self.headers.len() * 3) - 2,
                ),
            );
            output.push('\n');

            for (i, footer) in self.footer.iter().enumerate() {
                output.push_str(&self.format_cell(
                    footer,
                    self.column_widths[i],
                    self.alignments.get(i),
                ));
                output.push_str(" | ");
            }
            output.push('\n');
        }

        write!(f, "{}", output)
    }
}
