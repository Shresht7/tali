#[derive(Debug, Clone)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
pub struct Table {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    footer: Vec<String>,
    separator: String,
    col_widths: Vec<usize>,
    alignments: Vec<Alignment>,
}

impl Table {
    pub fn from(input: &str, delimiter: char) -> Table {
        let rows: Vec<Vec<String>> = input
            .lines()
            .map(|line| {
                line.split(delimiter)
                    .map(|s| s.trim().to_string())
                    .collect()
            })
            .collect();

        let col_count = rows.iter().map(|row| row.len()).max().unwrap_or(0);
        let alignments = vec![Alignment::Left; col_count];
        let mut col_widths = vec![0; col_count];

        for row in &rows {
            for (i, cell) in row.iter().enumerate() {
                col_widths[i] = std::cmp::max(col_widths[i], cell.len());
            }
        }

        let separator = String::from(" | ");
        Table {
            header: Vec::new(),
            rows,
            footer: Vec::new(),
            separator,
            col_widths,
            alignments,
        }
    }

    pub fn with_header(&mut self, header: Vec<String>) -> &mut Self {
        self.header = header;
        self
    }

    pub fn with_footer(&mut self, footer: Vec<String>) -> &mut Self {
        self.footer = footer;
        self
    }

    pub fn with_alignments(&mut self, alignments: Vec<Alignment>) -> &mut Self {
        self.alignments = alignments;
        self
    }

    fn update_col_widths(&mut self, row: &Vec<String>) -> &mut Self {
        for (i, cell) in row.iter().enumerate() {
            self.col_widths[i] = std::cmp::max(self.col_widths[i], cell.len());
        }
        self
    }

    pub fn add_row(&mut self, row: Vec<String>) -> &mut Self {
        self.update_col_widths(&row);
        self.rows.push(row);
        self
    }

    fn format_cell(&self, text: &str, width: usize, alignment: Option<&Alignment>) -> String {
        match alignment {
            Some(Alignment::Left) | None => format!("{:<width$}", text, width = width),
            Some(Alignment::Center) => format!("{:^width$}", text, width = width),
            Some(Alignment::Right) => format!("{:>width$}", text, width = width),
        }
    }

    fn format_row(&self, row: &Vec<String>) -> String {
        let mut res = String::new();
        for (i, cell) in row.iter().enumerate() {
            res.push_str(&self.format_cell(cell, self.col_widths[i], self.alignments.get(i)));
            res.push_str(&self.separator);
        }
        res
    }

    pub fn display(&self) -> String {
        let mut res = String::new();
        for row in &self.rows {
            res.push_str(&self.format_row(row));
            res.push_str("\n");
        }
        res
    }
}
