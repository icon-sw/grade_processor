use num_traits::{Float, Zero};
use crate::gmath::complex::{Complex, ComplexTrait};
use crate::gmath::numbers::factorize;
pub fn dft<T> (signal: &Vec<Complex<T>>) -> Vec<Complex<T>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let n = signal.len();
    let mut result = vec![Complex::<T>::new(T::zero(), T::zero()); n];
    for k in 0..n {
        for t in 0..n {
            let angle: T = <f64 as Into<T>>::into(-2.0 * std::f64::consts::PI * (k as f64 * t as f64 / n as f64));
            result[k] += signal[t] * Complex::from_polar(<f64 as Into<T>>::into(1.0), angle);
        }
    }
    result
}

pub fn idft<T> (signal: &Vec<Complex<T>>) -> Vec<Complex<T>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let n = signal.len();
    let mut result = vec![Complex::<T>::new(T::zero(), T::zero()); n];
    for k in 0..n {
        for t in 0..n {
            let angle : T = <f64 as Into<T>>::into(2.0 * std::f64::consts::PI * (k as f64 * t as f64 / n as f64));
            result[k] += signal[t] * Complex::from_polar(<f64 as Into<T>>::into(1.0), angle);
        }
        result[k] /= Complex::<T>::new(<f64 as Into<T>>::into(n as f64), T::zero());
    }
    result
} 

fn fft_core<T>( signal: &Vec<Complex<T>>,
                size: usize, 
                mut factors: Vec<u32>, 
                start: usize,
                step: usize,
                reverse: bool) -> Vec<Complex<T>>
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    if size == 1 || factors.len() == 0 {
        return std::vec![signal[start].clone()];
    }
    let chunk_number = factors[0] as usize;
    let chunk_size = size / chunk_number;
    let remaining = factors.drain(1..).collect::<Vec<u32>>();
    let mut chunks: Vec<Vec<Complex<T>>> = Vec::new();
    for i in 0..chunk_number {
        let chunk_fft = fft_core(&signal, chunk_size, remaining.clone(), start + i * step, step * chunk_number, reverse);
        chunks.push(chunk_fft);
    }
    let mut result = vec![Complex::<T>::new(T::zero(), T::zero()); size];
    let mut rotation_factor = -2.0 * std::f64::consts::PI;
    if reverse {
        rotation_factor *= -1.0;
    }
    for k in 0..size {
        for i in 0..chunk_number as usize {
            let index_sel = k % chunk_size as usize;
            let angle: T = <f64 as Into<T>>::into(rotation_factor * (k as f64 * i as f64 / size as f64));
            result[k] += chunks[i][index_sel] * Complex::from_polar(<f64 as Into<T>>::into(1.0), angle);
        }
    }
    result
}

pub fn fft<T>(signal: &Vec<Complex<T>>, 
                size: Option<usize>) -> Vec<Complex<T>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let fft_size = size.unwrap_or(signal.len());
    
    let mut factors = factorize(fft_size as u32);
    factors.sort_unstable_by(|a, b| b.cmp(a));
    // Call the core FFT function here
    fft_core(signal, fft_size, factors, 0, 1, false)
}

pub fn ifft<T>(signal: &Vec<Complex<T>>, 
                size: Option<usize>) -> Vec<Complex<T>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let fft_size = size.unwrap_or(signal.len());
    
    let mut factors = factorize(fft_size as u32);
    factors.sort_unstable_by(|a, b| b.cmp(a));
    // Call the core FFT function here
    let mut result = fft_core(signal, fft_size, factors, 0, 1, true);

    for i in 0..fft_size {
        result[i] /= Complex::<T>::new(<f64 as Into<T>>::into(fft_size as f64), T::zero());
    }
    result
}

pub fn fft2<T>(signal: &mut Vec<Vec<Complex<T>>>, size: Option<(usize, usize)>) -> Vec<Vec<Complex<T>>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let (rows, cols) = size.unwrap_or((signal.len(), signal[0].len()));
    let mut result = vec![vec![Complex::<T>::new(T::zero(), T::zero()); cols]; rows];
    
    for i in 0..rows {
        let row_fft = fft(signal.get_mut(i).unwrap(), Some(cols));
        result[i] = row_fft;
    }
    
    for j in 0..cols {
        let mut col_signal: Vec<Complex<T>> = result.iter().map(|r| r[j]).collect();
        let col_fft = fft(&mut col_signal, Some(rows));
        for i in 0..rows {
            result[i][j] = col_fft[i];
        }
    }
    
    result
}

pub fn ifft2<T>(signal: &mut Vec<Vec<Complex<T>>>, size: Option<(usize, usize)>) -> Vec<Vec<Complex<T>>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let (rows, cols) = size.unwrap_or((signal.len(), signal[0].len()));
    let mut result = vec![vec![Complex::<T>::new(T::zero(), T::zero()); cols]; rows];
    
    
    for j in 0..cols {
        let mut col_signal: Vec<Complex<T>> = signal.iter().map(|r| r[j]).collect();
        let col_fft = ifft(&mut col_signal, Some(rows));
        for i in 0..rows {
            result[i][j] = col_fft[i];
        }
    }
    for i in 0..rows {
        let row_fft = ifft(result.get_mut(i).unwrap(), Some(cols));
        result[i] = row_fft;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;
    use std::time::{Duration,Instant};

    #[test]
    fn test_dft() {
        let signal = vec![Complex::new(1.0, 0.0),
                          Complex::new(0.0, 1.0),
                          Complex::new(-1.0, 0.0),
                          Complex::new(0.0, -1.0),
                          Complex::new(1.0, 0.0),
                          Complex::new(0.0, -1.0)];
        let result = dft(&signal);
        let expected_result = vec![Complex::new(1.0,-1.0),
                                   Complex::new(1.0+3.0f64.sqrt(), 1.0+3.0f64.sqrt()),
                                   Complex::new(1.0+3.0f64.sqrt(), -1.0-3.0f64.sqrt()),
                                   Complex::new(1.0, 1.0),
                                   Complex::new(1.0-3.0f64.sqrt(), -1.0+3.0f64.sqrt()),
                                   Complex::new(1.0-3.0f64.sqrt(), 1.0-3.0f64.sqrt())];
        assert_eq!(result.len(), 6);
        for i in 0..result.len() {
            println!("Result: {:?}, Expected result: {:?}", result[i], expected_result[i]);
            assert!((result[i].real - expected_result[i].real).abs() < 1e-10);
            assert!((result[i].imag - expected_result[i].imag).abs() < 1e-10);
        }

        let idft_result = idft(&result);
        assert_eq!(idft_result.len(), 6);
        for i in 0..idft_result.len() {
            assert!((idft_result[i].real - signal[i].real).abs() < 1e-10);
            assert!((idft_result[i].imag - signal[i].imag).abs() < 1e-10);
        }
    }
    #[test]
    fn test_fft() {
        let signal = vec![Complex::new(1.0, 0.0),
                              Complex::new(0.0, 1.0),
                              Complex::new(-1.0, 0.0),
                              Complex::new(0.0, -1.0),
                              Complex::new(1.0, 0.0),
                              Complex::new(0.0, -1.0)];
        let mut result = fft(&signal, None);
        let expected_result = dft(&signal);
        assert_eq!(result.len(), 6);
        for i in 0..result.len() {
            println!("Result: {:?}, Expected result: {:?}", result[i], expected_result[i]);
            assert!((result[i].real - expected_result[i].real).abs() < 1e-10);
            assert!((result[i].imag - expected_result[i].imag).abs() < 1e-10);
        }
        let result = ifft(&mut result, None);
        assert_eq!(result.len(), 6);
        for i in 0..result.len() {
            println!("Result: {:?}, Expected result: {:?}", result[i], signal[i]);
            assert!((result[i].real - signal[i].real).abs() < 1e-10);
            assert!((result[i].imag - signal[i].imag).abs() < 1e-10);
        }
    }
}
