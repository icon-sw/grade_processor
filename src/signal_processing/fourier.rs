use num_traits::{Float, Zero};
use crate::gmath::complex::{Complex, ComplexTrait};
use crate::gmath::numbers::{is_pow2, factorize};
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

pub fn fft<T>(signal: &mut Vec<Complex<T>>, size: Option<usize>) -> Vec<Complex<T>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let mut fft_size = size.unwrap_or(signal.len());
    if fft_size == 1 {
        return signal.clone();
    }
    if !is_pow2(fft_size.try_into().unwrap()) {
        fft_size = fft_size.next_power_of_two();
        println!("FFT size must be a power of 2, set to {}", fft_size);
    }
    signal.resize(fft_size, Complex::<T>::new(T::zero(), T::zero()));
    let mut even: Vec<Complex<T>> = signal.iter().step_by(2).cloned().collect();
    let mut odd: Vec<Complex<T>> = signal.iter().skip(1).step_by(2).cloned().collect();
    
    let even_fft = fft(&mut even, Some(fft_size/2));
    let odd_fft = fft(&mut odd, Some(fft_size/2));

    let mut result = vec![Complex::<T>::new(T::zero(), T::zero()); fft_size];

    for k in 0..(fft_size / 2) {
        let t = Complex::<T>::from_polar(<f64 as Into<T>>::into(1.0),
                <f64 as Into<T>>::into(-2.0 * std::f64::consts::PI * k as f64 / fft_size as f64)) * odd_fft[k];
        result[k] = even_fft[k] + t;
        result[k + fft_size / 2] = even_fft[k] - t;
    }
    
    result
}   

pub fn ifft<T>(signal: &mut Vec<Complex<T>>, size: Option<usize>, scale: Option<bool>) -> Vec<Complex<T>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let mut ifft_size = size.unwrap_or(signal.len());
    if ifft_size == 1 {
        return signal.clone();
    }
    if !is_pow2(ifft_size.try_into().unwrap()) {
        ifft_size = ifft_size.next_power_of_two();
        println!("FFT size must be a power of 2, set to {}", ifft_size);
    }
    signal.resize(ifft_size, Complex::<T>::new(T::zero(), T::zero()));
    let mut even: Vec<Complex<T>> = signal.iter().step_by(2).cloned().collect();
    let mut odd: Vec<Complex<T>> = signal.iter().skip(1).step_by(2).cloned().collect();
    
    let even_fft = ifft(&mut even, Some(ifft_size/2), Some(false));
    let odd_fft = ifft(&mut odd, Some(ifft_size/2), Some(false));

    let mut result = vec![Complex::<T>::new(T::zero(), T::zero()); ifft_size];

    for k in 0..(ifft_size / 2) {
        let t = Complex::<T>::from_polar(<f64 as Into<T>>::into(1.0),
                <f64 as Into<T>>::into(2.0 * std::f64::consts::PI * k as f64 / ifft_size as f64)) * odd_fft[k];
        result[k] = even_fft[k] + t;
        result[k + ifft_size / 2] = even_fft[k] - t;
    }
    if scale.unwrap_or(true) {
        for value in &mut result {
            *value /= Complex::<T>::new(<f64 as Into<T>>::into(ifft_size as f64), T::zero());
        }
    }
    
    result
}

pub fn fdft<T>(signal: &mut Vec<Complex<T>>, size: Option<usize>, mut factors: Option<Vec<u32>>) -> Vec<Complex<T>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let fdft_size: u32 = size.unwrap_or(signal.len()).try_into().unwrap();
    if is_pow2(fdft_size) {
        return fft(signal, Some(fdft_size as usize));
    }
    match factors {
        Some(ref f) => {
            if f.len() == 0 {
                factors = Some(factorize(fdft_size));
            }
        },
        None => {
            factors = Some(factorize(fdft_size));
        }
    }
    if factors.as_ref().unwrap().len() == 1 {
        return dft(signal);
    }
    let mut unwrapped_factors = factors.unwrap().clone();
    unwrapped_factors.sort_unstable_by(|a, b| b.cmp(a));
    println!("Factors: {:?}", unwrapped_factors);
    let mut result = vec![Complex::<T>::new(T::zero(), T::zero()); fdft_size as usize];

    let mut chunk_number = unwrapped_factors.clone();
    let factor_rec: Vec<u32> = chunk_number.drain(1..).collect();
    
    let chunk_number = chunk_number[0] as u32;
    println!("Chunk number: {:?}, Factors: {:?}", chunk_number, factor_rec);

    let chunk_size = fdft_size/ chunk_number;
    let mut chunks: Vec<Vec<Complex<T>>> = Vec::new();
    

    for i in 0..chunk_number {
        let mut chunk: Vec<Complex<T>> = signal.iter().skip(i as usize)
                                        .step_by(chunk_number.try_into().unwrap())
                                        .cloned()
                                        .collect();
        chunk.resize(chunk_size as usize, Complex::<T>::new(T::zero(), T::zero()));
        let chunk_fft = fdft(&mut chunk, Some(chunk_size as usize), Some(factor_rec.clone()));
        println!("Chunk {:?}, FFT: {:?}", chunk, chunk_fft);
        chunks.push(chunk_fft);
    }

    for k in 0..(fdft_size) as usize {
        for i in 0..chunk_number as usize {
            let mut t = Complex::<T>::new(T::zero(), T::zero());
            let index_sel = k % chunk_size as usize;
            let angle: T = <f64 as Into<T>>::into(-2.0 * std::f64::consts::PI * (k as f64 * i as f64 / fdft_size as f64));
            t = chunks[i][index_sel] * Complex::from_polar(<f64 as Into<T>>::into(1.0), angle);
            
            println!("Index: {}, chunk {}, value {}", k, i, t);
            result[k] += t;
        }
        println!("Final value for index {}: {}", k, result[k]);
    }
    result
}


pub fn ifdft<T>(signal: &mut Vec<Complex<T>>, size: Option<usize>, mut factors: Option<Vec<u32>>, scale: Option<bool>) -> Vec<Complex<T>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let idft_size: u32 = size.unwrap_or(signal.len()).try_into().unwrap();
    if is_pow2(idft_size) {
        return fft(signal, Some(idft_size as usize));
    }

    match factors {
        Some(ref f) => {
            if f.len() == 0 {
                factors = Some(factorize(idft_size));
            }
        },
        None => {
            factors = Some(factorize(idft_size));
        }
    }
    if factors.as_ref().unwrap().len() == 1 {
        return dft(signal);
    }
    let mut unwrapped_factors = factors.unwrap().clone();
    unwrapped_factors.sort_unstable_by(|a, b| b.cmp(a));
    println!("Unwrapped factors: {:?}", unwrapped_factors);
    let mut result = vec![Complex::<T>::new(T::zero(), T::zero()); idft_size as usize];

    let mut chunk_number = unwrapped_factors.clone();
    let factor_rec: Vec<u32> = chunk_number.drain(1..).collect();
    
    let chunk_number = chunk_number[0] as u32;
    let chunk_size = idft_size/ chunk_number;
    let mut chunks: Vec<Vec<Complex<T>>> = Vec::new();
    
    
    for i in 0..chunk_number {
        let mut chunk: Vec<Complex<T>> = signal.iter().skip(i as usize)
                                        .step_by(chunk_number.try_into().unwrap())
                                        .cloned()
                                        .collect();
        chunk.resize(chunk_size as usize, Complex::<T>::new(T::zero(), T::zero()));
        let chunk_fft = ifdft(&mut chunk, Some(chunk_size as usize), Some(factor_rec.clone()), Some(false));
        chunks.push(chunk_fft);
    }

    for k in 0..(idft_size) as usize {
        for i in 0..chunk_number as usize {
            let mut t = Complex::<T>::new(T::zero(), T::zero());
            let index_sel = k % chunk_size as usize;
            let angle: T = <f64 as Into<T>>::into(2.0 * std::f64::consts::PI * (k as f64 * i as f64 / idft_size as f64));
            t = chunks[i][index_sel] * Complex::from_polar(<f64 as Into<T>>::into(1.0), angle);
            result[k] += t;
        }
    }
    if scale.unwrap_or(true) {
        for k in 0..(idft_size) as usize {
            result[k] /= Complex::<T>::new(<f64 as Into<T>>::into(idft_size as f64), T::zero());
        }
    }
    result
}

pub fn fft2d<T>(signal: &mut Vec<Vec<Complex<T>>>, size: Option<(usize, usize)>) -> Vec<Vec<Complex<T>>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let (rows, cols) = size.unwrap_or((signal.len(), signal[0].len()));
    let mut result = vec![vec![Complex::<T>::new(T::zero(), T::zero()); cols]; rows];
    
    for i in 0..rows {
        let row_fft = fdft(signal.get_mut(i).unwrap(), Some(cols), None);
        result[i] = row_fft;
    }
    
    for j in 0..cols {
        let mut col_signal: Vec<Complex<T>> = result.iter().map(|r| r[j]).collect();
        let col_fft = fdft(&mut col_signal, Some(rows), None);
        for i in 0..rows {
            result[i][j] = col_fft[i];
        }
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
        let mut signal = vec![Complex::new(1.0, 0.0),
                              Complex::new(0.0, 1.0),
                              Complex::new(-1.0, 0.0),
                              Complex::new(0.0, -1.0),
                              Complex::new(1.0, 0.0),
                              Complex::new(0.0, -1.0)];
        let mut result = fft(&mut signal, None);
        signal.resize(8, Complex::new(0.0, 0.0));
        let expected_result = dft(&signal);
        assert_eq!(result.len(), 8);
        for i in 0..result.len() {
            println!("Result: {:?}, Expected result: {:?}", result[i], expected_result[i]);
            assert!((result[i].real - expected_result[i].real).abs() < 1e-10);
            assert!((result[i].imag - expected_result[i].imag).abs() < 1e-10);
        }
        let ifft_result = ifft(&mut result, None, None);
        assert_eq!(ifft_result.len(), 8);
        for i in 0..ifft_result.len() {
            println!("Result: {:?}, Expected result: {:?}", ifft_result[i], signal[i]);
            assert!((ifft_result[i].real - signal[i].real).abs() < 1e-10);
            assert!((ifft_result[i].imag - signal[i].imag).abs() < 1e-10);
        }
    }
    #[test]
    fn test_fdft() {
        let mut signal = vec![Complex::new(1.0, 0.0),
                              Complex::new(0.0, 1.0),
                              Complex::new(-1.0, 0.0),
                              Complex::new(0.0, -1.0),
                              Complex::new(1.0, 0.0),
                              Complex::new(0.0, -1.0)];
        let mut result = fdft(&mut signal, None, None);
        let expected_result = dft(&signal);
        assert_eq!(result.len(), 6);
        for i in 0..result.len() {
            println!("Result: {:?}, Expected result: {:?}", result[i], expected_result[i]);
            assert!((result[i].real - expected_result[i].real).abs() < 1e-10);
            assert!((result[i].imag - expected_result[i].imag).abs() < 1e-10);
        }
        let result = ifdft(&mut result, None, None, None);
        assert_eq!(result.len(), 6);
        for i in 0..result.len() {
            assert!((result[i].real - signal[i].real).abs() < 1e-10);
            assert!((result[i].imag - signal[i].imag).abs() < 1e-10);
        }
    }
    #[test]
    fn test_performance() {
        let mut rng = rand::rng();
        let n = 65536;
        let mut duration: Duration = Duration::from_secs(0);
        for i in 0..100 {
            let mut signal: Vec<Complex<f64>> = Vec::with_capacity(n);
            for k in 0..n {
                signal.push(Complex::new(rng.random(), rng.random()));
            }
            let start = Instant::now();
            let result = fft(&mut signal, None);
            duration += start.elapsed();
        }
        println!("Mean time execution: {:?}", duration.as_millis() / 100);
        assert!(duration.as_millis()/100 < 1)
    }
}
