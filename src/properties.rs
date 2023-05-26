use crate::matrix::{Matrix, SparseMatrix, BandMatrix};

// Check if a matrix is square
pub fn is_square(mat: &Matrix) -> bool {
    mat.rows() == mat.cols()
}



