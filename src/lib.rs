pub mod matrix;
pub mod sparse;
pub mod block;
pub mod banded;
//mod operations;
pub use banded::BandSparseMatrix;
pub use matrix::Matrix;
pub use sparse::SparseMatrix;
