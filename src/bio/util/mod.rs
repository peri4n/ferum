#[derive(Debug)]
pub(crate) struct Table {
    pub rows: usize,
    pub cols: usize,
    elements: Vec<i32>,
}

impl Table {
    pub fn new(rows: usize, cols: usize) -> Self {
        return Table {
            rows,
            cols,
            elements: vec![0; rows * cols]
        };
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        self.rows * row + col
    }

    pub fn get(&self, row: usize, col: usize) -> &i32 {
        unsafe { self.elements.get_unchecked(self.idx(row, col)) }
    }

    pub fn set(&mut self, row: usize, col: usize, elem: i32) {
        let index = self.idx(row, col);
        self.elements[index] = elem
    }
}

#[cfg(test)]
mod tests {
    use super::Table;

    #[test]
    fn can_be_accessed() {
        let mut table = Table::new(4, 4);
        let val = 3;
        table.set(0, 0, val);
        table.set(1, 1, val);

        assert_eq!(table.get(0,0), &val);
        assert_eq!(table.get(1,1), &val);
    }

    #[test]
    fn creates_the_correct_number_of_elements() {
        let table = Table::new(5, 4);
        assert_eq!(table.elements.len(), 20)

    }

}
