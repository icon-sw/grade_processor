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

pub fn ifft<T>(signal: &mut Vec<Complex<T>>, size: Option<usize>) -> Vec<Complex<T>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let mut ifft_size = size.unwrap_or(signal.len());
    if !is_pow2(ifft_size.try_into().unwrap()) {
        ifft_size = ifft_size.next_power_of_two();
        println!("FFT size must be a power of 2, set to {}", ifft_size);
    }
    signal.resize(ifft_size, Complex::<T>::new(T::zero(), T::zero()));
    let mut even: Vec<Complex<T>> = signal.iter().step_by(2).cloned().collect();
    let mut odd: Vec<Complex<T>> = signal.iter().skip(1).step_by(2).cloned().collect();
    
    let even_fft = fft(&mut even, Some(ifft_size/2));
    let odd_fft = fft(&mut odd, Some(ifft_size/2));

    let mut result = vec![Complex::<T>::new(T::zero(), T::zero()); ifft_size];

    for k in 0..(ifft_size / 2) {
        let t = Complex::<T>::from_polar(<f64 as Into<T>>::into(1.0),
                <f64 as Into<T>>::into(2.0 * std::f64::consts::PI * k as f64 / ifft_size as f64)) * odd_fft[k];
        result[k] = even_fft[k] + t;
        result[k + ifft_size / 2] = even_fft[k] - t;
    }

    for value in &mut result {
        *value /= Complex::<T>::new(<f64 as Into<T>>::into(ifft_size as f64), T::zero());
    }
    
    result
}

pub fn fdft<T>(signal: &mut Vec<Complex<T>>, size: Option<usize>, mut factors: Option<Vec<u32>>) -> Vec<Complex<T>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let mut fdft_size: u32 = size.unwrap_or(signal.len()).try_into().unwrap();
    if is_pow2(fdft_size) {
        fft(signal, Some(fdft_size as usize));
        return signal.clone();
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
    let mut result = signal.clone();
    
    let mut chunk_number = factors.unwrap().clone();
    let factor_rec: Vec<u32> = chunk_number.drain(1..).collect();
    
    let chunk_number = chunk_number[0] as u32;
    let chunk_size = fdft_size/ chunk_number;
    let mut chunks: Vec<Vec<Complex<T>>> = Vec::new();
    
    
    for _i in 0..chunk_number {
        let mut chunk: Vec<Complex<T>> = signal.iter()
                                        .step_by(chunk_number.try_into().unwrap())
                                        .cloned()
                                        .collect();
        chunk.resize(chunk_size as usize, Complex::<T>::new(T::zero(), T::zero()));
        let chunk_fft = fdft(&mut chunk, Some(chunk_size as usize), Some(factor_rec.clone()));
        chunks.push(chunk_fft);
    }

    for k in 0..(fdft_size) as usize {
        for i in 0..chunk_number as usize {
            let mut t = Complex::<T>::new(T::zero(), T::zero());
            for j in 0..chunk_size as usize {
                let angle: T = <f64 as Into<T>>::into(-2.0 * std::f64::consts::PI * (k as f64 * j as f64 / fdft_size as f64));
                t += chunks[i][j] * Complex::from_polar(<f64 as Into<T>>::into(1.0), angle);
            }
            result[k] += t;
        }
    }
    result
}


pub fn ifdft<T>(signal: &mut Vec<Complex<T>>, size: Option<usize>, mut factors: Option<Vec<u32>>) -> Vec<Complex<T>> 
where
    T: Float + Into<f64> + Copy + Zero
    + std::fmt::Display + std::fmt::Debug
    + std::convert::From<f64>
{
    let mut idft_size: u32 = size.unwrap_or(signal.len()).try_into().unwrap();
    if is_pow2(idft_size) {
        fft(signal, Some(idft_size as usize));
        return signal.clone();
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
    let mut result = signal.clone();
    
    let mut chunk_number = factors.unwrap().clone();
    let factor_rec: Vec<u32> = chunk_number.drain(1..).collect();
    
    let chunk_number = chunk_number[0] as u32;
    let chunk_size = idft_size/ chunk_number;
    let mut chunks: Vec<Vec<Complex<T>>> = Vec::new();
    
    
    for _i in 0..chunk_number {
        let mut chunk: Vec<Complex<T>> = signal.iter()
                                        .step_by(chunk_number.try_into().unwrap())
                                        .cloned()
                                        .collect();
        chunk.resize(chunk_size as usize, Complex::<T>::new(T::zero(), T::zero()));
        let chunk_fft = fdft(&mut chunk, Some(chunk_size as usize), Some(factor_rec.clone()));
        chunks.push(chunk_fft);
    }

    for k in 0..(idft_size) as usize {
        for i in 0..chunk_number as usize {
            let mut t = Complex::<T>::new(T::zero(), T::zero());
            for j in 0..chunk_size as usize {
                let angle: T = <f64 as Into<T>>::into(2.0 * std::f64::consts::PI * (k as f64 * j as f64 / idft_size as f64));
                t += chunks[i][j] * Complex::from_polar(<f64 as Into<T>>::into(1.0), angle);
            }
            result[k] += t;
        }
        result[k] /= Complex::<T>::new(<f64 as Into<T>>::into(idft_size as f64), T::zero());
    }
    result
}
