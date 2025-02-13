pub struct TableIter<'a> {
    header: Option<&'a Vec<String>>,
    rows: std::slice::Iter<'a, Vec<String>>,
    footer: Option<&'a Vec<String>>,
    state: IterState,
}

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
        TableIter {
            header: if self.header.is_empty() {
                None
            } else {
                Some(&self.header)
            },
            rows: self.rows.iter(),
            footer: if self.footer.is_empty() {
                None
            } else {
                Some(&self.footer)
            },
            state: IterState::Header,
        }
    }
}
