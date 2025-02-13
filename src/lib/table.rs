use crate::helpers::ansi;

#[derive(Debug, Clone)]
pub enum Alignment {
    Left,
    Center,
    Right,
}

#[derive(Debug)]
struct Separator {
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

#[derive(Debug, Default)]
pub struct Table {
    header: Vec<String>,
    rows: Vec<Vec<String>>,
    footer: Vec<String>,

    separator: Separator,

    col_widths: Vec<usize>,
    alignments: Vec<Alignment>,
}

impl Table {
    pub fn with_horizontal_separator(mut self, separator: &str) -> Self {
        self.separator.horizontal = separator.to_owned();
        self
    }

    pub fn with_vertical_separator(mut self, separator: &str) -> Self {
        self.separator.vertical = separator.to_owned();
        self
    }
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
                col_widths[i] = std::cmp::max(col_widths[i], ansi::visible_width(cell));
            }
        }

        Table {
            header: Vec::new(),
            rows,
            footer: Vec::new(),
            col_widths,
            alignments,
            ..Default::default()
        }
    }

    pub fn with_header(&mut self, header: Vec<String>) -> &mut Self {
        self.update_col_widths(&header);
        self.header = header;
        self
    }

    pub fn with_footer(&mut self, footer: Vec<String>) -> &mut Self {
        self.update_col_widths(&footer);
        self.footer = footer;
        self
    }

    pub fn with_alignments(&mut self, alignments: Vec<Alignment>) -> &mut Self {
        self.alignments = alignments;
        self
    }

    fn update_col_widths(&mut self, row: &Vec<String>) -> &mut Self {
        for (i, cell) in row.iter().enumerate() {
            self.col_widths[i] = std::cmp::max(self.col_widths[i], ansi::visible_width(cell));
        }
        self
    }

    pub fn add_row(&mut self, row: Vec<String>) -> &mut Self {
        self.update_col_widths(&row);
        self.rows.push(row);
        self
    }

    fn format_vertical_separator(&self) -> String {
        let sep_v = &self.separator.vertical;
        let sep_h = &self.separator.horizontal;
        self.col_widths
            .iter()
            .map(|w| sep_v.repeat(*w))
            .collect::<Vec<_>>()
            .join(&sep_v.repeat(sep_h.len()))
            + &sep_v
            + "\n"
    }

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

    fn format_row(&self, row: &Vec<String>) -> String {
        let mut res = String::new();
        for (i, cell) in row.iter().enumerate() {
            res.push_str(&self.format_cell(cell, self.col_widths[i], self.alignments.get(i)));
            res.push_str(&self.separator.horizontal);
        }
        res.push_str("\n");
        res
    }

    pub fn display(&self) -> String {
        let mut res = String::new();

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
