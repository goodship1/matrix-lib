pub struct BlockMatrix<T> {
    blocks: Vec<Vec<Vec<T>>>,
    block_rows: usize,
    block_cols: usize,
    block_size: usize,
}

impl<T> BlockMatrix<T>
where
    T: Default + Clone,
{
    pub fn new(block_rows: usize, block_cols: usize, block_size: usize) -> Self {
        let mut blocks = vec![vec![vec![T::default(); block_size]; block_cols]; block_rows];
        BlockMatrix {
            blocks,
            block_rows,
            block_cols,
            block_size,
        }
    }

    pub fn get_block(&self, row: usize, col: usize) -> Option<&Vec<Vec<T>>> {
        self.blocks.get(row).and_then(|r| r.get(col))
    }

    pub fn set_block(&mut self, row: usize, col: usize, block: Vec<Vec<T>>) -> Result<(), String> {
        if block.len() == self.block_size && block[0].len() == self.block_size {
            if let Some(row_blocks) = self.blocks.get_mut(row) {
                if let Some(curr_block) = row_blocks.get_mut(col) {
                    *curr_block = block;
                    return Ok(());
                }
            }
            return Err("Invalid block index".to_string());
        }
        Err("Invalid block size".to_string())
    }

    pub fn get_element(&self, row: usize, col: usize) -> Option<T> {
        let block_row = row / self.block_size;
        let block_col = col / self.block_size;
        let block = self.get_block(block_row, block_col)?;
        let inner_row = row % self.block_size;
        let inner_col = col % self.block_size;
        block.get(inner_row).and_then(|r| r.get(inner_col)).cloned()
    }

    pub fn set_element(&mut self, row: usize, col: usize, value: T) -> Result<(), String> {
        let block_row = row / self.block_size;
        let block_col = col / self.block_size;
        let block = self.get_block(block_row, block_col)?;
        let inner_row = row % self.block_size;
        let inner_col = col % self.block_size;
        if let Some(inner_vec) = block.get_mut(inner_row) {
            if let Some(curr_value) = inner_vec.get_mut(inner_col) {
                *curr_value = value;
                return Ok(());
            }
        }
        Err("Invalid element index".to_string())
    }
}

use std::ops::Add;

impl<T> Add for BlockMatrix<T>
where
    T: Default + Clone + Add<Output = T>,
{
    type Output = Result<BlockMatrix<T>, String>;

    fn add(self, other: BlockMatrix<T>) -> Result<BlockMatrix<T>, String> {
        if self.block_rows == other.block_rows
            && self.block_cols == other.block_cols
            && self.block_size == other.block_size
        {
            let mut result = BlockMatrix::new(self.block_rows, self.block_cols, self.block_size);
            for row in 0..self.block_rows {
                for col in 0..self.block_cols {
                    let self_block = self.get_block(row, col).ok_or("Invalid block index")?;
                    let other_block = other.get_block(row, col).ok_or("Invalid block index")?;
                    let mut sum_block = vec![vec![T::default(); self.block_size]; self.block_size];
                    for i in 0..self.block_size {
                        for j in 0..self.block_size {
                            sum_block[i][j] = self_block[i][j].clone() + other_block[i][j].clone();
                        }
                    }
                    result.set_block(row, col, sum_block)?;
                }
            }
            Ok(result)
        } else {
            Err("Matrix dimensions mismatch".to_string())
        }
    }
}
