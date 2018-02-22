pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows: Vec<Vec<_>> = vec![];

        for i in 0..row_count as usize {
            let mut row = vec![1; i + 1];

            for ii in 1..row.len() - 1 {
                row[ii] = rows[i - 1][ii] + rows[i - 1][ii - 1];
            }

            rows.push(row);
        }
        PascalsTriangle { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
