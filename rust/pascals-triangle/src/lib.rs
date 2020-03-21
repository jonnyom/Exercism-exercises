pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let row_count = row_count as usize;
        let mut rows: Vec<Vec<u32>> = Vec::with_capacity(row_count);
        for i in 0..row_count {
            let mut row: Vec<u32> = vec![1u32; i + 1];
            if i >= 2 {
                for j in 1..i {
                    let left = rows[i - 1][j - 1];
                    let right = rows[i - 1][j];
                    row[j] = left + right;
                }
            }
            rows.push(row);
        }
        Self { rows }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.clone()
    }
}
