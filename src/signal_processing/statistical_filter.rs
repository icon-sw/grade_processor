use crate::gmath::vector::Vector;

pub fn average_moving_window_filter<T>(data: Vector<T>,
                                        window_size: usize) 
                                        -> Vector<T> 
where T: Clone + Copy 
    + std::ops::Mul<Output=T> 
    + std::ops::Div<Output=T> 
    + num_traits::Zero 
    + std::cmp::PartialEq
{
    Vector::<T>::new(Vec::<T>::new())
}

pub fn median_moving_window_filter() {

}