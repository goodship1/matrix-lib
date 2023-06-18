pub struct BlockMatrix<T> {
    blocks: Vec<Vec<Vec<T>>>,
    block_size: usize,
}

impl<T: Clone + Default> BlockMatrix<T> {
    pub fn new(num_rows: usize, num_cols: usize, block_size: usize) -> Self {
        let num_block_rows = (num_rows as f64 / block_size as f64).ceil() as usize;
        let num_block_cols = (num_cols as f64 / block_size as f64).ceil() as usize;

        let mut blocks = Vec::with_capacity(num_block_rows);
        for _ in 0..num_block_rows {
            let mut row_blocks = Vec::with_capacity(num_block_cols);
            for _ in 0..num_block_cols {
                row_blocks.push(vec![Default::default(); block_size * block_size]);
            }
            blocks.push(row_blocks);
        }

        BlockMatrix {
            blocks,
            block_size,
        }
    }

    pub fn get(&self, row: usize, col: usize) -> Option<&T> {
        let block_row = row / self.block_size;
        let block_col = col / self.block_size;
        let inner_row = row % self.block_size;
        let inner_col = col % self.block_size;

        self.blocks
            .get(block_row)?
            .get(block_col)?
            .get(inner_row * self.block_size + inner_col)
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let block_row = row / self.block_size;
        let block_col = col / self.block_size;
        let inner_row = row % self.block_size;
        let inner_col = col % self.block_size;

        if let Some(block) = self.blocks.get_mut(block_row) {
            if let Some(row_blocks) = block.get_mut(block_col) {
                if let Some(cell) = row_blocks.get_mut(inner_row * self.block_size + inner_col) {
                    *cell = value;
                }
            }
        }
    }
}

