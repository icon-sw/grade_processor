use num_traits::{Zero, One};

#[derive(Debug, Clone)]
pub struct Matrix<T> {
    pub data: Vec<Vec<T>>,
    pub shape: Option<(usize, usize)>,
}

impl <T: Clone + Zero + One + Copy + std::cmp::PartialEq> Matrix<T> {
    pub fn new(data: Vec<Vec<T>>) -> Self {
        let shape = if data.is_empty() {
            None
        } else {
            Some((data.len(), data[0].len()))
        };
        Matrix {
            data,
            shape,
        }
    }

    pub fn from_slice(slice: &[Vec<T>]) -> Self {
        Matrix::new(slice.to_vec())
    }

    pub fn to_vec(&self) -> Vec<Vec<T>> {
        self.data.clone()
    }

    pub fn size(&self) -> (usize, usize) {
        (self.data.len(), if !self.data.is_empty() { self.data[0].len() } else { 0 })
    }

    pub fn is_square(&self) -> bool {
        match self.shape {
            Some((rows, cols)) => rows == cols,
            None => false,
        }
    }

    pub fn zeros(rows: usize, cols: usize) -> Self
    where
        T: num_traits::Zero + Clone,
    {
        let data = vec![vec![T::zero(); cols]; rows];
        Matrix::new(data)
    }
    pub fn ones(rows: usize, cols: usize) -> Self
    where
        T: num_traits::One + Clone,
    {
        let data = vec![vec![T::one(); cols]; rows];
        Matrix::new(data)
    }

    pub fn identity(size: usize) -> Self
    where
        T: num_traits::One + num_traits::Zero + Clone,
    {
        let mut data = vec![vec![T::zero(); size]; size];
        for i in 0..size {
            data[i][i] = T::one();
        }
        Matrix::new(data)
    }

    pub fn transpose(&mut self) -> Matrix<T> {
        
        let mut transposed: Vec<Vec<T>> = vec![vec![]; self.size().1];
        for row in &self.data {
            for (i, value) in row.iter().enumerate() {
                transposed[i].push(value.clone());
            }
        }
        Matrix::new(transposed)      
    }

    pub fn p_l_u_factorization(&self) -> (Vec<Vec<T>>,Vec<Vec<T>>) {
        unimplemented!()
    }
    pub fn determinant(&self) -> T
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy + num_traits::Zero,
    {
        if !self.is_square() {
            panic!("Determinant can only be calculated for square matrices");
        }
        // Implement determinant calculation here (e.g., using recursion or LU decomposition)
        unimplemented!()
    }
}