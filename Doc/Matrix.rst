===========================
Matrix.rs Documentation
===========================


Creating basic types of matrices 2 x 2.

.. code-block:: rust
   
   use::crate::matrix::Matrix;
   fn main() {
       let values = &[1.0, 2.0, 3.0, 4.0];
       let matrix = Matrix::new(2, 2, values);

       matrix.display_matrix();
   }
   
 The values slice should contain the elements of the matrix in row-major order, 
 meaning the elements of each row should be contiguous in the slice.
 For example, to create a 2x2 matrix with elements [1.0, 2.0,3.0, 4.0],
 you would pass the values slice as &[1.0, 2.0, 3.0, 4.0]. 
 The elements are arranged row-wise: the first two elements represent the first row, 
 and the next two elements represent the second row.


