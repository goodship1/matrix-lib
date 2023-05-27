use crate::matrix::Matrix;
use crate::sparse::SparseMatrix;
use crate::banded::BandSparseMatrix;
use std::collections::HashMap;

pub fn add(mat: &Matrix, matone: &Matrix) -> Option<Matrix> {
    if mat.rows != matone.rows || mat.cols != matone.cols {
        return None; // Return None if the matrices have different dimensions
    }

    let mut new_mat = Matrix::zero(mat.rows, mat.cols); // Create a new zero matrix

    for x in 0..mat.rows {
        for y in 0..mat.cols {
            let sum = mat.get(x, y) + matone.get(x, y);
            new_mat.set(x, y, sum);
        }
    }

    Some(new_mat)
}
pub fn trace(mat:&Matrix) -> f64{
      assert_eq!(mat.rows,mat.col,"Matrix has to be square.");
      let mut trace = 0.0;
      for x in 0..mat.rows{
           trace += mat.get(x,x);
    }
    trace
}

pub fn sparsetrace(mat:&SparseMatrix) -> f64{
    assert_eq!(mat.rows,mat.cols,"Matrix has to be square.");
    let mut trace  = 0.0;
    for x in 0..mat.rows{
        let Some(value) = mat.get(x,x){
            trace += value;
        }
    }
    trace
}
          
pub fn multiply(mat: &Matrix, matone: &Matrix) -> Option<Matrix> {
    if mat.cols != matone.rows {
        return None; // Return None if the matrices have incompatible dimensions
    }

    let mut result = Matrix::zero(mat.rows, matone.cols); // Create a new zero matrix

    for x in 0..mat.rows {
        for y in 0..matone.cols {
            let mut sum = 0.0;

            for z in 0..mat.cols {
                sum += mat.get(x, z) * matone.get(z, y);
            }

            result.set(x, y, sum);
        }
    }

    Some(result)
}

// Scalar multiplication
pub fn scalar_multiply(mat: &Matrix, scalar: f64) -> Matrix {
    let mut result = Matrix::zero(mat.rows, mat.cols);

    for x in 0..mat.rows {
        for y in 0..mat.cols {
            let product = mat.get(x, y) * scalar;
            result.set(x, y, product);
        }
    }

    result
}

// Matrix transpose
pub fn transpose(mat: &Matrix) -> Matrix {
    let mut result = Matrix::zero(mat.cols, mat.rows);

    for x in 0..mat.rows {
        for y in 0..mat.cols {
            let value = mat.get(x, y);
            result.set(y, x, value);
        }
    }

    result
}

// Matrix exponentiation
pub fn power(mat: &Matrix, exponent: usize) -> Option<Matrix> {
    if mat.rows != mat.cols {
        return None; // Return None if the matrix is not square
    }

    let mut result = mat.clone();

    for _ in 1..exponent {
        result = match multiply(&result, mat) {
            Some(m) => m,
            None => return None, // Return None if matrix multiplication fails
        };
    }

    Some(result)
}

pub fn add_sparse(mat: &SparseMatrix, matone: &SparseMatrix) -> Option<SparseMatrix> {
    assert_eq!(mat.rows, matone.rows, "Matrix dimensions mismatch");
    assert_eq!(mat.cols, matone.cols, "Matrix dimensions mismatch");

    let mut new_matrix = SparseMatrix::new(mat.rows, mat.cols);

    for (row, col, value) in mat.iter_nonzero() {
        let valueone = matone.get(row, col);
        let add = value + valueone;

        if add != 0.0 {
            new_matrix.set(row, col, add);
        }
    }

    Some(new_matrix)
}

pub fn minus_sparse(mat: &SparseMatrix, matone: &SparseMatrix) -> Option<SparseMatrix> {
    assert_eq!(mat.rows, matone.rows, "Matrix dimensions mismatch");
    assert_eq!(mat.cols, matone.cols, "Matrix dimensions mismatch");

    let mut new_matrix = SparseMatrix::new(mat.rows, mat.cols);

    for (row, col, value) in mat.iter_nonzero() {
        let valueone = matone.get(row, col);
        let sub = value - valueone;

        if sub != 0.0 {
            new_matrix.set(row, col, sub);
        }
    }

    Some(new_matrix)
}

pub fn transpose_sparse(mat: &SparseMatrix) -> SparseMatrix {
    let rows = mat.rows;
    let cols = mat.cols;
    let mut new_matrix = SparseMatrix::new(cols, rows);

    for (row, col, value) in mat.iter_nonzero() {
        new_matrix.set(col, row, value);
    }

    new_matrix
}

pub fn sparse_scalar_multiply(mat: &SparseMatrix, scalar: f64) -> SparseMatrix {
    let rows = mat.rows;
    let cols = mat.cols;
    let mut new_matrix = SparseMatrix::new(rows, cols);

    for (row, col, value) in mat.iter_nonzero() {
        new_matrix.set(row, col, scalar * value);
    }

    new_matrix
}

pub fn sparse_scalar_divide(mat: &SparseMatrix, scalar: f64) -> SparseMatrix {
    assert_ne!(scalar, 0.0, "Scalar cannot be zero");

    let rows = mat.rows;
    let cols = mat.cols;
    let mut new_matrix = SparseMatrix::new(rows, cols);

    for (row, col, value) in mat.iter_nonzero() {
        new_matrix.set(row, col, value / scalar);
    }

    new_matrix
}

pub fn row_scale(mat&Matrix,row:usize,scalar:f64) {
    for x in 0..mat.cols{
        let val = mat.get(row,x) * scalar;
        mat.set(row,x,val);
    }
}
pub fn col_scale(mat&Matrix,col:usize,scalar:f64){
    for x in 0..mat.rows{
        let val = mat.get(col,x) * scalar;
        mat.set(col,x,val);
    }
}




pub fn sparse_matrix_multiply(mat: &SparseMatrix, matone: &SparseMatrix) -> SparseMatrix {
    assert_eq!(mat.cols, matone.rows, "Matrix dimensions mismatch");

    let rows = mat.rows;
    let cols = matone.cols;
    let mut new_matrix = SparseMatrix::new(rows, cols);

    for (row, col, value) in mat.iter_nonzero() {
        for (matone_row, matone_col, matone_value) in matone.iter_nonzero() {
            if col == matone_row {
                let times = value * matone_value;
                let change = new_matrix.get(row, matone_col);
                new_matrix.set(row, matone_col, times + change);
            }
        }
    }

    new_matrix
}


pub fn determinant(mat: &Matrix) -> Option<f64> {
    if mat.rows != mat.cols {
        return None; // Return None if the matrix is not square
    }

    let n = mat.rows;

    if n == 1 {
        return Some(mat.get(0, 0)); // Return the single element for a 1x1 matrix
    }

    if n == 2 {
        let a = mat.get(0, 0);
        let b = mat.get(0, 1);
        let c = mat.get(1, 0);
        let d = mat.get(1, 1);

        return Some(a * d - b * c); // Return the determinant of a 2x2 matrix
    }

    let mut det = 0.0;

    for j in 0..n {
        let cofactor = mat.get(0, j) * cofactor_sign(0, j) * determinant(&submatrix(mat, 0, j))?;

        det += cofactor;
    }

    Some(det)
}

fn cofactor_sign(row: usize, col: usize) -> f64 {
    if (row + col) % 2 == 0 {
        1.0
    } else {
        -1.0
    }
}

fn submatrix(mat: &Matrix, exclude_row: usize, exclude_col: usize) -> Matrix {
    let n = mat.rows;

    let mut submat = Matrix::zero(n - 1, n - 1);

    let mut r = 0;

    for i in 0..n {
        if i == exclude_row {
            continue;
        }

        let mut c = 0;

        for j in 0..n {
            if j == exclude_col {
                continue;
            }

            submat.set(r, c, mat.get(i, j));
            c += 1;
        }

        r += 1;
    }

    submat
}


pub fn sparse_slice(mat:&SparseMatrix, row_start:usize,row_end:usize,col_start:usize,col_end) -> Option<SparseMatrix>{
    assert!(row_start <= row_end && row_end < mat.rows,"Invalid row indices.");
    assert!(col_start <= col_end && col_end < mat.col,"Invalid columns indices.");
    let new_rows =  row_start - row_end;
    let new_cols= col_start- col_end;
    let new_matrix  = SparseMatrix(new_row,new_col);//creates new matrix
    for (x,y z) in mat.iter_nonezero(){
        if x >= row_start && x < row_end && y >= col_start && y < col_end{
            let new_row =  x  - row_start;
            let new_col =  y - col_start;
            new_matrix.set(new_row,new_col,z);
        }
    }
    Some(new_matrix)
}


pub fn sparse_matrix_vector_multiply(mat: &SparseMatrix, vector: &[f64]) -> Vec<f64> {
    assert_eq!(mat.cols, vector.len(), "Incompatible types");

    let rows = mat.rows;
    let mut new_vector = vec![0.0; rows];

    for (row, col, value) in mat.iter_nonzero() {
        new_vector[row] += value * vector[col];
    }

    new_vector
}




pub fn add_bandsparse_matrices(mat: &BandSparseMatrix, matone: &BandSparseMatrix) -> BandSparseMatrix {
    assert_eq!(mat.rows, matone.rows, "Matrices must have the same number of rows");
    assert_eq!(mat.cols, matone.cols, "Matrices must have the same number of columns");

    let rows = mat.rows;
    let cols = mat.cols;
    let mut new_matrix = BandSparseMatrix::new(rows, cols);

    // Iterate over each band in the matrices
    for (band, data1) in mat.bands.iter() {
        let data2 = matone.bands.get(band).unwrap_or(&vec![0.0; cols]);

        // Add corresponding elements from the same band
        let sum = data1.iter().zip(data2.iter()).map(|(&a, &b)| a + b).collect::<Vec<_>>();

        // Store the sum in the result matrix
        new_matrix.bands.insert(*band, sum);
    }

    new_matrix
}



pub fn subtract_bandsparse_matrices(matrix1: &BandSparseMatrix, matrix2: &BandSparseMatrix) -> BandSparseMatrix {
    assert_eq!(matrix1.rows, matrix2.rows, "Matrices must have the same number of rows");
    assert_eq!(matrix1.cols, matrix2.cols, "Matrices must have the same number of columns");

    let rows = matrix1.rows;
    let cols = matrix1.cols;
    let mut result = BandSparseMatrix::new(rows, cols);

    // Iterate over each band in the matrices
    for (band, data1) in matrix1.bands.iter() {
        let data2 = matrix2.bands.get(band).unwrap_or(&vec![0.0; cols]);

        // Subtract corresponding elements from the same band
        let diff = data1.iter().zip(data2.iter()).map(|(&a, &b)| a - b).collect::<Vec<_>>();

        // Store the difference in the result matrix
        result.bands.insert(*band, diff);
    }

    result
}


pub fn banded_sparse_multiply(a: &BandSparseMatrix, b: &BandSparseMatrix) -> Option<BandSparseMatrix> {
    // Check if the number of columns in matrix A is equal to the number of rows in matrix B
    if a.cols != b.rows {
        return None;
    }

    let mut result = BandSparseMatrix::new(a.rows, b.cols);

    // Iterate over the rows of matrix A
    for (row_a, band_a) in &a.bands {
        // Iterate over the columns of matrix B
        for (col_b, band_b) in &b.bands {
            // Calculate the resulting band for the product matrix
            let result_band = row_a + col_b;

            // Check if the resulting band is within the matrix dimensions
            if result_band >= -(result.cols as isize - 1) && result_band <= (result.rows as isize - 1) {
                let row_data_a = band_a;
                let col_data_b = band_b;

                let mut row_data_result = result.bands.entry(result_band).or_insert(vec![0.0; result.cols]);

                // Perform the dot product of the corresponding bands of A and B
                for i in 0..a.cols {
                    row_data_result[i] += row_data_a[i] * col_data_b[i];
                }
            }
        }
    }

    Some(result)
}

