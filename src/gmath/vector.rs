use num_traits::Zero;

pub struct Vector<T>{
    pub data: Vec<T>,
}

impl <T: Clone + Copy 
    + std::ops::Mul<Output=T> 
    + std::ops::Div<Output=T> 
    + num_traits::Zero 
    + std::cmp::PartialEq> Vector<T> {
    
    pub fn new(data: Vec<T>) -> Self {
        Vector { data }
    }

    pub fn from_slice(slice: &[T]) -> Self {
        Vector { data: slice.to_vec() }
    }

    pub fn to_vec(&self) -> Vec<T> {
        self.data.clone()
    }

    pub fn to_slice(&self) -> &[T] {
        &self.data
    }

    pub fn size(&self) -> usize {
        self.data.len()
    }

    pub fn norm(&self) -> f64
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy + Zero + Into<f64>,
    {
        let mut sum = 0.0;
        for value in &self.data {
            let val: f64 = (*value).into();
            sum += val * val;
        }
        sum.sqrt()
    }

    pub fn dot(&self, other: &Self) -> T
    where
        T: std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy + Zero,
    {
        if self.data.len() != other.data.len() {
            panic!("Vectors must be of the same length for dot product");
        }
        
        let mut result = T::zero();
        for (a, b) in self.data.iter().zip(&other.data) {
            result = result + (*a * *b);
        }
        result
    }

    fn mul(self, scalar: T) -> Self {
        let mut result = self.data.clone();
        for value in &mut result {
            *value = *value * scalar;
        }
        Vector { data: result }
    }

    fn div(self, scalar: T) -> Self {
        if scalar == T::zero() {
            panic!("Division by zero is not allowed");
        }
        
        let mut result = self.data.clone();
        for value in &mut result {
            *value = *value / scalar;
        }
        Vector { data: result }
    }
}

impl <T: std::ops::Add<Output = T> + Clone + Copy> std::ops::Add for Vector<T> {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        if self.data.len() != other.data.len() {
            panic!("Vectors must be of the same length for addition");
        }
        
        let mut result = self.data.clone();
        for (i, value) in other.data.iter().enumerate() {
            result[i] = result[i] + *value;
        }
        Vector { data: result }
    }
}

impl <T: std::ops::Sub<Output = T> + Clone + Copy> std::ops::Sub for Vector<T> {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        if self.data.len() != other.data.len() {
            panic!("Vectors must be of the same length for subtraction");
        }
        
        let mut result = self.data.clone();
        for (i, value) in other.data.iter().enumerate() {
            result[i] = result[i] - *value;
        }
        Vector { data: result }
    }
}