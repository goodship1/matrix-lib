use std::collections::HashMap;

pub struct SparseMatrix {
    rows: usize,
    cols: usize,
    elements: HashMap<(usize, usize), f64>,
}

impl SparseMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        SparseMatrix {
            rows,
            cols,
            elements: HashMap::new(),
        }
    }

    pub fn iter_nonzero(&self) -> impl Iterator<Item = (usize, usize, f64)> + '_ {
        self.elements
            .iter()
            .map(|((row, col), &value)| (*row, *col, value))
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        assert!(
            row < self.rows && col < self.cols,
            "Index out of bounds"
        );
        self.elements.insert((row, col), value);
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        *self.elements.get(&(row, col)).unwrap_or(&0.0)
    }

    pub fn to_dense(&self) -> Vec<Vec<f64>> {
        let mut dense = vec![vec![0.0; self.cols]; self.rows];
        for ((row, col), &value) in self.elements.iter() {
            dense[*row][*col] = value;
        }
        dense
    }

    pub fn triangle_lower(rows: usize, data: Vec<f64>) -> Self {
        let mut new_matrix = SparseMatrix::new(rows, rows);

        for x in 0..rows {
            for y in 0..=x {
                let index = x * rows + y;
                let value = data.get(index).cloned().unwrap_or(0.0);
                if value != 0.0 {
                    new_matrix.elements.insert((x, y), value);
                }
            }
        }

        new_matrix
    }

    pub fn triangle_upper(rows: usize, data: Vec<f64>) -> Self {
        let mut new_matrix = SparseMatrix::new(rows, rows);

        for x in 0..rows {
            for y in x..rows {
                let index = x * rows + y - (x * (x + 1)) / 2;
                let value = data.get(index).cloned().unwrap_or(0.0);
                if value != 0.0 {
                    new_matrix.elements.insert((x, y), value);
                }
            }
        }

        new_matrix
    }

    pub fn tridiagonal(rows: usize, data: Vec<f64>) -> Self {
        assert_eq!(data.len(), 3 * rows - 2, "Invalid data size");

        let mut new_matrix = SparseMatrix::new(rows, rows);

        for x in 0..rows {
            let val = data[x];
            if val != 0.0 {
                new_matrix.elements.insert((x, x), val);
            }
            if x > 0 {
                let val = data[x + rows - 1];
                if val != 0.0 {
                    new_matrix.elements.insert((x, x - 1), val);
                }
            }
            if x < rows - 1 {
                let val = data[x + rows];
                if val != 0.0 {
                    new_matrix.elements.insert((x, x + 1), val);
                }
            }
        }

        new_matrix
    }

    pub fn add_sparse(&self, matone: &SparseMatrix) -> Option<SparseMatrix> {
        assert_eq!(
            self.rows, matone.rows,
            "Matrix dimensions mismatch"
        );
        assert_eq!(
            self.cols, matone.cols,
            "Matrix dimensions mismatch"
        );

        let mut new_matrix = SparseMatrix::new(self.rows, self.cols);

        for (row, col, value) in self.iter_nonzero() {
            let valueone = matone.get(row, col);
            let add = value + valueone;

            if add != 0.0 {
                new_matrix.set(row, col, add);
            }
        }

        Some(new_matrix)
    }

    pub fn minus_sparse(&self, matone: &SparseMatrix) -> Option<SparseMatrix> {
        assert_eq!(self.rows, matone.rows, "Matrix dimensions mismatch.");
        assert_eq!(self.cols, matone.cols, "Matrix dimension mismatch.");

        let mut new_matrix = SparseMatrix::new(self.rows, self.cols);
        for (x, y, z) in self.iter_nonzero() {
            let valueone = matone.get(x, y);
            let minus = z - valueone;
            if minus != 0.0 {
                new_matrix.set(x, y, minus);
            }
        }
        Some(new_matrix)
    }

    pub fn multiply_sparse(&self, matone: &SparseMatrix) -> Option<SparseMatrix> {
        assert_eq!(self.cols, matone.rows, "Matrix dimensions mismatch");

        let mut new_matrix = SparseMatrix::new(self.rows, matone.cols);

        for (row, col, value) in self.iter_nonzero() {
            for x in 0..matone.cols {
                let valueone = matone.get(col, x);
                let product = value * valueone;
                let sum = new_matrix.get(row, x) + product;
                if sum != 0.0 {
                    new_matrix.set(row, x, sum);
                }
            }
        }

        Some(new_matrix)
    }

    pub fn transpose_sparse(&self) -> SparseMatrix {
        let mut new_matrix = SparseMatrix::new(self.cols, self.rows);

        for (row, col, value) in self.iter_nonzero() {
            new_matrix.set(col, row, value);
        }

        new_matrix
    }
}
