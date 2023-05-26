pub struct Matrix {
    rows: usize,
    cols: usize,
    elements: Vec<f64>,
}

impl Matrix {
    //implements basic matrices of n x m size.
    pub fn new(rows: usize, cols: usize, values: &[f64]) -> Self {
        assert_eq!(rows * cols, values.len());
        let elements = values.to_vec();
        Matrix {
            rows,
            cols,
            elements,
        }
    }
    
    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.elements[row * self.cols + col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.elements[row * self.cols + col] = value;
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        let elements = vec![0.0; rows * cols];
        Matrix {
            rows,
            cols,
            elements,
        }
    }

    pub fn triangle_upper(rows: usize) -> Self {
        let mut matrix = Matrix::zero(rows, rows);
        for x in 0..rows {
            for y in x..rows {
                let change = (x + 1) as f64 * (y + 1) as f64;
                matrix.set(x, y, change);
            }
        }
        matrix
    }

    pub fn triangle_lower(rows: usize) -> Self {
        let mut matrix = Matrix::zero(rows, rows);
        for row in 0..rows {
            for col in 0..=row {
                matrix.set(row, col, 1.0);
            }
        }
        matrix
    }
    //todo create a triangle lower and above with own values
    pub fn triangle_lower_data(data: &[f64], rows: usize) -> Result<Self, &'static str> {
    let expected_elements = rows * (rows + 1) / 2;
    
    if data.len() != expected_elements {
        return Err("Invalid number of data elements.");
    }
    
    let mut matrix = Matrix::zero(rows, rows);
    
    for row in 0..rows {
        for col in 0..=row {
            let index = row * (row + 1) / 2 + col;
            matrix.set(row, col, data[index]);
        }
    }
    
    Ok(matrix)
}

   
    
    pub fn identity(rows: usize) -> Self {
        let mut matrix = Matrix::zero(rows, rows);
        for x in 0..rows {
            matrix.set(x, x, 1.0);
        }
        matrix
    }
    
    
    
        
    pub fn display_matrix(&self) {
        for x in 0..self.rows {
            for y in 0..self.cols {
                print!("{:.2} ", self.get(x, y));
            }
            println!();
        }
    }
}

