/// Iterator for traversing a [`Table`][super::Table] in order: header, rows, and footer
pub struct TableIter<'a> {
    header: Option<&'a Vec<String>>,
    rows: std::slice::Iter<'a, Vec<String>>,
    footer: Option<&'a Vec<String>>,
    state: IterState,
}

/// Tracks the current iteration state
enum IterState {
    Header,
    Rows,
    Footer,
    Done,
}

impl<'a> Iterator for TableIter<'a> {
    type Item = &'a Vec<String>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.state {
            IterState::Header => {
                self.state = IterState::Rows;
                self.header
            }
            IterState::Rows => self.rows.next().or_else(|| {
                self.state = IterState::Footer;
                self.next()
            }),
            IterState::Footer => {
                self.state = IterState::Done;
                self.footer
            }
            IterState::Done => None,
        }
    }
}

impl<'a> IntoIterator for &'a super::Table {
    type Item = &'a Vec<String>;
    type IntoIter = TableIter<'a>;
    fn into_iter(self) -> Self::IntoIter {
        // Skip iteration states if they are empty
        let initial_state = if !self.header.is_empty() {
            IterState::Header
        } else if !self.rows.is_empty() {
            IterState::Rows
        } else {
            IterState::Footer
        };

        TableIter {
            header: (!self.header.is_empty()).then_some(&self.header),
            rows: self.rows.iter(),
            footer: (!self.footer.is_empty()).then_some(&self.footer),
            state: initial_state,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn test_table_iterator_order() {
        let mut table = Table::from_csv("a,b\nc,d");
        table.with_header(vec!["H1".to_string(), "H2".to_string()]);
        table.with_footer(vec!["F1".to_string(), "F2".to_string()]);

        let iterated: Vec<Vec<String>> = table.into_iter().cloned().collect();

        let expected = vec![
            vec!["H1".to_string(), "H2".to_string()],
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
            vec!["F1".to_string(), "F2".to_string()],
        ];

        assert_eq!(iterated, expected);
    }

    #[test]
    fn test_table_iterator_empty_header() {
        let mut table = Table::from_csv("a,b\nc,d");
        table.with_footer(vec!["F1".to_string(), "F2".to_string()]);

        let iterated: Vec<Vec<String>> = table.into_iter().cloned().collect();

        let expected = vec![
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
            vec!["F1".to_string(), "F2".to_string()],
        ];

        assert_eq!(iterated, expected);
    }

    #[test]
    fn test_table_iterator_empty_footer() {
        let mut table = Table::from_csv("a,b\nc,d");
        table.with_header(vec!["H1".to_string(), "H2".to_string()]);

        let iterated: Vec<Vec<String>> = table.into_iter().cloned().collect();

        let expected = vec![
            vec!["H1".to_string(), "H2".to_string()],
            vec!["a".to_string(), "b".to_string()],
            vec!["c".to_string(), "d".to_string()],
        ];

        assert_eq!(iterated, expected);
    }

    #[test]
    fn test_table_iterator_empty_rows() {
        let mut table = Table::from_csv("");
        table.with_header(vec!["H1".to_string(), "H2".to_string()]);
        table.with_footer(vec!["F1".to_string(), "F2".to_string()]);

        let iterated: Vec<Vec<String>> = table.into_iter().cloned().collect();

        let expected = vec![
            vec!["H1".to_string(), "H2".to_string()],
            vec!["F1".to_string(), "F2".to_string()],
        ];

        assert_eq!(iterated, expected);
    }
}
