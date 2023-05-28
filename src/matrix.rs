pub struct Matrix {
    rows: usize,
    cols: usize,
    elements: Vec<f64>,
}

impl Matrix {
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

    pub fn identity(rows: usize) -> Self {
        let mut matrix = Matrix::zero(rows, rows);
        for x in 0..rows {
            matrix.set(x, x, 1.0);
        }
        matrix
    }

    pub fn add(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);

        let mut new_matrix = Matrix::zero(self.rows, self.cols);

        for x in 0..self.rows {
            for y in 0..self.cols {
                let sum = self.get(x, y) + other.get(x, y);
                new_matrix.set(x, y, sum);
            }
        }

        new_matrix
    }

    pub fn subtract(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.rows, other.rows);
        assert_eq!(self.cols, other.cols);
        let mut new_matrix = Matrix::zero(self.rows, self.cols);
        for x in 0..self.rows {
            for y in 0..self.cols {
                let diff = self.get(x, y) - other.get(x, y);
                new_matrix.set(x, y, diff);
            }
        }
        new_matrix
    }

    fn submatrix(&self, start_row: usize, start_col: usize, num_rows: usize, num_cols: usize) -> Matrix {
        let mut sub = Matrix::zero(num_rows, num_cols);
        for x in 0..num_rows {
            for y in 0..num_cols {
                let val = self.get(start_row + x, start_col + y);
                sub.set(x, y, val);
            }
        }
        sub
    }

    fn assign_submatrix(&mut self, start_row: usize, start_col: usize, sub: &Matrix) {
        let num_rows = sub.rows;
        let num_cols = sub.cols;
        for x in 0..num_rows {
            for y in 0..num_cols {
                let val = sub.get(x, y);
                self.set(start_row + x, start_col + y, val);
            }
        }
    }

    pub fn multiply(&self, other: &Matrix) -> Matrix {
        assert_eq!(self.cols, other.rows);

        // Base case: if the matrix dimensions are small, use regular matrix multiplication
        if self.rows <= 1 || self.cols <= 1 || other.cols <= 1 {
            let mut new_matrix = Matrix::zero(self.rows, other.cols);

            for x in 0..self.rows {
                for y in 0..other.cols {
                    let mut sum = 0.0;
                    for z in 0..self.cols {
                        sum += self.get(x, z) * other.get(z, y);
                    }
                    new_matrix.set(x, y, sum);
                }
            }

            return new_matrix;
        }

        // Split the matrices into submatrices
        let half_rows = self.rows / 2;
        let half_cols = self.cols / 2;

        let a11 = self.submatrix(0, 0, half_rows, half_cols);
        let a12 = self.submatrix(0, half_cols, half_rows, half_cols);
        let a21 = self.submatrix(half_rows, 0, half_rows, half_cols);
        let a22 = self.submatrix(half_rows, half_cols, half_rows, half_cols);

        let b11 = other.submatrix(0, 0, half_rows, half_cols);
        let b12 = other.submatrix(0, half_cols, half_rows, half_cols);
        let b21 = other.submatrix(half_rows, 0, half_rows, half_cols);
        let b22 = other.submatrix(half_rows, half_cols, half_rows, half_cols);

        // Compute the products of submatrices using Strassen algorithm
        let p1 = a11.multiply(&b12.subtract(&b22));
        let p2 = a11.add(&a12).multiply(&b22);
        let p3 = a21.add(&a22).multiply(&b11);
        let p4 = a22.multiply(&b21.subtract(&b11));
        let p5 = a11.add(&a22).multiply(&b11.add(&b22));
        let p6 = a12.subtract(&a22).multiply(&b21.add(&b22));
        let p7 = a11.subtract(&a21).multiply(&b11.add(&b12));

        let c11 = p5.add(&p4).subtract(&p2).add(&p6);
        let c12 = p1.add(&p2);
        let c21 = p3.add(&p4);
        let c22 = p5.add(&p1).subtract(&p3).subtract(&p7);

        let mut mat = Matrix::zero(self.rows, other.cols);
        mat.assign_submatrix(0, 0, &c11);
        mat.assign_submatrix(0, half_cols, &c12);
        mat.assign_submatrix(half_rows, 0, &c21);
        mat.assign_submatrix(half_rows, half_cols, &c22);

        mat
    }

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

    pub fn triangle_above_data(data: &[f64], rows: usize) -> Result<Self, &'static str> {
        let expected_elements = rows * (rows + 1) / 2;

        if data.len() != expected_elements {
            return Err("Invalid number of data elements.");
        }

        let mut matrix = Matrix::zero(rows, rows);

        for row in 0..rows {
            for col in row..rows {
                let index = col * (col + 1) / 2 + row;
                matrix.set(row, col, data[index]);
            }
        }

        Ok(matrix)
    }

    pub fn scalar_multiply(&self, scalar: f64) -> Matrix {
        let mut new_matrix = Matrix::zero(self.rows, self.cols);

        for x in 0..self.rows {
            for y in 0..self.cols {
                let product = self.get(x, y) * scalar;
                new_matrix.set(x, y, product);
            }
        }

        new_matrix
    }

    pub fn transpose(&self) -> Matrix {
        let mut new_matrix = Matrix::zero(self.cols, self.rows);

        for x in 0..self.rows {
            for y in 0..self.cols {
                let value = self.get(x, y);
                new_matrix.set(y, x, value);
            }
        }

        new_matrix
    }


    
     
    pub fn trace(self) -> f64 {
	assert_eq!(self.rows,self.cols,"Matrix has to be square");
	let mut trace = 0.0;// stores the trace variable
	for x in 0..self.rows{
		trace +=  self.get(x,x);
	}
	trace
    }

    
    pub fn row_scale(&mut self,row:usize,scalar:f64){
	for x in 0..self.rows{
        let val =  self.get(row,x) * scalar;
	self.set(row,x,val)
       }
    }	 
     
    
    pub fn col_scale(&mut self,col:usize,scalar:f64) {
	for x in 0..self.cols{
		let val = self.get(col,x) * scalar;
		self.set(col,x ,val);
	}
      }


pub fn swap_rows(&mut self, row1: usize, row2: usize) {
        assert!(row1 < self.rows, "Row1 index out of bounds.");
        assert!(row2 < self.rows, "Row2 index out of bounds.");

        for col in 0..self.cols {
            let temp = self.get(row1, col);
            self.set(row1, col, self.get(row2, col));
            self.set(row2, col, temp);
        }
    }

    fn is_zero(&self, value: f64) -> bool {
        const EPSILON: f64 = 1e-10;
        value.abs() < EPSILON
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

