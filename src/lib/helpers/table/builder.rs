#[derive(Debug)]
pub struct TableBuilder {
    contents: String,
    has_header: bool,
    has_footer: bool,
    delimiter: char,
}

impl Default for TableBuilder {
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
    pub fn new(input: impl Into<String>, delimiter: char) -> Self {
        Self {
            contents: input.into(),
            delimiter,
            ..Default::default()
        }
    }

    pub fn from(&mut self, contents: impl Into<String>) -> &mut Self {
        self.contents = contents.into();
        self
    }

    pub fn with_delimiter(&mut self, delimiter: char) -> &mut Self {
        self.delimiter = delimiter;
        self
    }

    pub fn with_header(&mut self, yes: bool) -> &mut Self {
        self.has_header = yes;
        self
    }

    pub fn with_footer(&mut self, yes: bool) -> &mut Self {
        self.has_footer = yes;
        self
    }

    pub fn from_csv(input: impl Into<String>) -> Self {
        Self::new(input, ',')
    }

    pub fn from_tsv(input: impl Into<String>) -> Self {
        Self::new(input, '\t')
    }

    pub fn build(&self) -> super::Table {
        let mut rows = self
            .contents
            .lines()
            .map(|line| {
                line.split(self.delimiter)
                    .map(|w| w.trim().to_string())
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        let header = if self.has_header && !rows.is_empty() {
            rows.remove(0)
        } else {
            Vec::new()
        };
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

        table.columns.mark_for_recalc();
        table
    }
}
