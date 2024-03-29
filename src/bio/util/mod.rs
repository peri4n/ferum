use std::ops::{Index, IndexMut};

pub(crate) struct Table {
    rows: usize,
    cols: usize,
    elements: Vec<Vec<i32>>,
}

impl Index<usize> for Table {
    type Output = Vec<i32>;

    fn index(&self, row: usize) -> &Self::Output {
        &self.elements[row]
    }
}

impl IndexMut<usize> for Table {
    fn index_mut(&mut self, row: usize) -> &mut Self::Output {
        &mut self.elements[row]
    }
}

impl Table {
    pub fn new(rows: usize, cols: usize) -> Self {
        let elements = (0..rows).map(|_| vec![0; cols]).collect();

        Table {
            rows,
            cols,
            elements,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::Table;

    #[test]
    fn can_be_accessed() {
        let mut table = Table::new(4, 4);
        let val = 3;
        table[0][0] = val;
        table[1][1] = val;

        assert_eq!(table[0][0], val);
        assert_eq!(table[1][1], val);
    }
}
