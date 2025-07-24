
pub fn average<T>(data: &Vec<T>) -> T 
where
    T: num_traits::Zero + num_traits::One + std::ops::Add<Output = T> 
       + std::ops::Div<Output = T> + Copy + From<usize>,
{
    let len = T::from(data.len());

    if data.len() == 0 {
        return T::zero();
    }
    let sum: T = data.iter().cloned().fold(T::zero(), |acc, x| acc + x);
    sum / len
}

pub fn square_mean<T>(data: &Vec<T>) -> T 
where
    T: num_traits::Zero + num_traits::One + std::ops::Add<Output = T> 
       + std::ops::Div<Output = T> + Copy + From<usize>,
{
    if data.is_empty() {
        return T::zero();
    }
    let sum: T = data.iter().cloned().fold(T::zero(), |acc, x| acc + x * x);
    sum / T::from(data.len())
}

pub fn harmonic_mean<T>(data: &Vec<T>) -> T 
where
    T: num_traits::Zero + num_traits::One + std::ops::Add<Output = T> 
       + std::ops::Div<Output = T> + Copy + From<usize>,
{
    if data.is_empty() {
        return T::zero();
    }
    let n = T::from(data.len());
    let sum: T = data.iter().cloned().fold(T::zero(), |acc, x| acc + T::one() / x);
    n / sum
}

pub fn geometric_mean<T>(data: &Vec<T>) -> T 
where
    T: num_traits::Zero + num_traits::One + std::ops::Mul<Output = T> 
       + num_traits::Pow<T, Output = T> + Copy + From<f64>,
{
    if data.is_empty() {
        return T::zero();
    }
    let product: T = data.iter().cloned().fold(T::one(), |acc, x| acc * x);
    product.pow(T::from(1.0 / data.len() as f64))
}

pub fn variance<T>(data: &Vec<T>) -> T 
where
    T: num_traits::Zero + num_traits::One + std::ops::Add<Output = T> 
       + std::ops::Div<Output = T> + std::ops::Sub<Output = T> + Copy + From<usize>,
{
    if data.len() == 0 {
        return T::zero();
    }
    let avg = average(data);
    let sum: T = data.iter().cloned().fold(T::zero(), |acc, x| acc + (x - avg) * (x - avg));
    sum / T::from(data.len())
}

pub fn standard_deviation<T>(data: &Vec<T>) -> T 
where
    T: num_traits::Zero + num_traits::One + std::ops::Add<Output = T> + num_traits::Float
       + std::ops::Div<Output = T> + std::ops::Sub<Output = T> + Copy + From<usize>,
{
    variance(data).sqrt()
}

pub fn median<T>(data: &mut Vec<T>) -> T 
where
    T: num_traits::Zero + num_traits::One + std::ops::Add<Output = T> 
       + std::ops::Div<Output = T> + Copy + From<usize> + PartialOrd,
{
    if data.is_empty() {
        return T::zero();
    }
    data.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = data.len() / 2;
    if data.len() % 2 == 0 {
        (data[mid - 1] + data[mid]) / T::from(2)
    } else {
        data[mid]
    }
}

pub fn mode<T>(data: &Vec<T>) -> Vec<T> 
where
    T: std::cmp::Eq + std::hash::Hash + Clone,
{
    use std::collections::HashMap;

    let mut occurrences = HashMap::new();
    for item in data {
        *occurrences.entry(item.clone()).or_insert(0) += 1;
    }

    let max_count = occurrences.values().cloned().max().unwrap_or(0);
    occurrences.into_iter()
        .filter(|&(_, count)| count == max_count)
        .map(|(item, _)| item)
        .collect()
}

pub fn unique<T>(data: &Vec<T>) -> Vec<T> 
where
    T: std::cmp::Eq + std::hash::Hash + Clone,
{
    use std::collections::HashSet;

    let mut seen = HashSet::new();
    data.iter().filter(|&x| seen.insert(x.clone())).cloned().collect()
}
