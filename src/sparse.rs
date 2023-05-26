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

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        assert!(row < self.rows && col < self.cols, "Index out of bounds");
        self.elements.insert((row, col), value);
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        if let Some(&value) = self.elements.get(&(row, col)) {
            value
        } else {
            0.0
        }
    }

    pub fn to_dense(&self) -> Vec<Vec<f64>> {
        let mut dense = vec![vec![0.0; self.cols]; self.rows];
        for ((row, col), &value) in self.elements.iter() {
            dense[row][col] = value;
        }
        dense
    }
    
    pub fn triangle_lower(rows: usize, data: Vec<f64>) -> Self {
        let mut new_matrix = SparseMatrix {
            rows,
            cols: rows,
            elements: HashMap::new(),
        };

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

    pub fn triangle_above(rows: usize, data: Vec<f64>) -> Self {
        let mut new_matrix = SparseMatrix {
            rows,
            cols: rows,
            elements: HashMap::new(),
        };

        for x in 0..rows {
            for y in x..rows {
                let index = x * rows + y;
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

        let mut new_matrix = SparseMatrix {
            rows,
            cols: rows,
            elements: HashMap::new(),
        };

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
}

