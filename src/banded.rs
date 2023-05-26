use std::collections::HashMap;

pub struct BandSparseMatrix {
    rows: usize,
    cols: usize,
    bands: HashMap<isize, Vec<f64>>,
}

impl BandSparseMatrix {
    pub fn new(rows: usize, cols: usize) -> Self {
        BandSparseMatrix {
            rows,
            cols,
            bands: HashMap::new(),
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        let band = (row as isize) - (col as isize);
        let row_data = self.bands.entry(band).or_insert(vec![0.0; self.cols]);
        row_data[col] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        let band = (row as isize) - (col as isize);
        if let Some(row_data) = self.bands.get(&band) {
            row_data[col]
        } else {
            0.0
        }
    }

    pub fn is_bandwidth(&self, lower_bandwidth: usize, upper_bandwidth: usize) -> bool {
        let diagonal_band = upper_bandwidth as isize;
        for (band, _) in &self.bands {
            let distance = band - diagonal_band;
            if distance.abs() > lower_bandwidth as isize {
                return false;
            }
        }
        true
    }
}

