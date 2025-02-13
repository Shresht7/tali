/// Represents the column widths of a table
#[derive(Debug, Default)]
pub struct Columns {
    /// Stores the computed widths for each column.
    pub widths: Vec<usize>,
    /// Indicates whether the widths need to be recalculated
    needs_recalculation: bool,
}

impl Columns {
    /// Marks the column widths as needing recalculation
    pub fn mark_for_recalc(&mut self) {
        self.needs_recalculation = true;
    }

    /// Retrieves the width of the column at the given index, if it exists
    pub fn get(&self, index: usize) -> Option<&usize> {
        self.widths.get(index)
    }

    /// Retrieves the width of the column at the given index, or returns the fallback value
    pub fn get_or(&self, index: usize, fallback: usize) -> usize {
        self.widths.get(index).copied().unwrap_or(fallback)
    }

    /// Calculates column widths based on the provided rows.
    ///
    /// Each row is expected to be an iterable of strings, representing table cells.
    /// This function ensures that `widths` are updated to the longest/widest value per column
    pub fn calculate<I>(&mut self, rows: I)
    where
        I: IntoIterator,
        I::Item: AsRef<[String]>,
    {
        if !self.needs_recalculation {
            return; // Skip re-calculation if it is not needed
        }

        let mut max_col_count = 0;

        for row in rows {
            let row = row.as_ref();
            max_col_count = max_col_count.max(row.len());

            // Ensure widths vector is long enough
            if self.widths.len() < row.len() {
                self.widths.resize(row.len(), 0);
            }

            for (i, cell) in row.iter().enumerate() {
                self.widths[i] = self.widths[i].max(cell.len());
            }
        }

        self.widths.resize(max_col_count, 0); // Ensure final size matches longest row
        self.needs_recalculation = false; // Unset the flag when done
    }
}

/// Implements iteration over `Columns` by returning an iterator over widths
impl<'a> IntoIterator for &'a Columns {
    type Item = &'a usize;
    type IntoIter = std::slice::Iter<'a, usize>;
    fn into_iter(self) -> Self::IntoIter {
        self.widths.iter()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mark_for_recalc() {
        let mut cols = Columns::default();
        assert_eq!(cols.needs_recalculation, false);
        cols.mark_for_recalc();
        assert_eq!(cols.needs_recalculation, true);
    }

    #[test]
    fn test_get() {
        let cols = Columns {
            widths: vec![5, 10, 15],
            needs_recalculation: false,
        };
        assert_eq!(cols.get(1), Some(&10));
        assert_eq!(cols.get(3), None);
    }

    #[test]
    fn test_get_or() {
        let cols = Columns {
            widths: vec![5, 10, 15],
            needs_recalculation: false,
        };
        assert_eq!(cols.get_or(1, 42), 10);
        assert_eq!(cols.get_or(3, 42), 42);
    }

    #[test]
    fn test_calculate() {
        let mut cols = Columns::default();
        cols.mark_for_recalc();

        let rows = vec![
            vec!["Hello".to_string(), "World".to_string()],
            vec!["Rust".to_string(), "Lang".to_string()],
            vec!["LongerText".to_string(), "Test".to_string()],
        ];

        cols.calculate(rows);
        assert_eq!(cols.widths, vec![10, 5]);
    }

    #[test]
    fn test_iteration() {
        let cols = Columns {
            widths: vec![8, 12, 6],
            needs_recalculation: false,
        };
        let widths: Vec<usize> = cols.into_iter().copied().collect();
        assert_eq!(widths, vec![8, 12, 6]);
    }
}
