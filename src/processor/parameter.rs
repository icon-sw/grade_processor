use std::sync::{Arc, Mutex};
use std::cmp::Ord;
use std::collections::HashMap;

pub trait ParameterTrait {
    fn name(&self) -> &str;
    fn get_value(&self, block_id: u64) -> Box<dyn std::any::Any>;
    fn set_value(&mut self, value: Box<dyn std::any::Any>, block_id: u64) -> Result<(), &str>;
}
#[derive(Clone)]
pub struct Parameter<T> {
    name: String,
    description: String,
    values: Arc<Mutex<HashMap<u64,T>>>,
    default_value: T,
    min_value: Option<T>,
    max_value: Option<T>,
    allowed_values: Vec<T>,
}

impl<T> Parameter<T> {
    fn new(
        name: String,
        description: String,
        default_value: T,
        min_value: Option<T>,
        max_value: Option<T>,
        allowed_values: Vec<T>,
    ) -> Self {
        Parameter {
            name,
            description,
            values: Arc::new(Mutex::new(HashMap::new())),
            default_value,
            min_value,
            max_value,
            allowed_values,
        }
    }

}
impl<T: 'static + Ord + Clone> ParameterTrait for Parameter<T> {
    fn name(&self) -> &str {
        &self.name
    }
    fn get_value(&self, block_id: u64) -> Box<dyn std::any::Any> {
        match self.values.lock().unwrap().get(&block_id) {
            Some(value) => Box::new(value.clone()),
            None => Box::new(self.default_value.clone()),
        }
    }
    fn set_value(&mut self, value: Box<dyn std::any::Any>, block_id: u64) -> Result<(), &str> {
        if let Ok(v) = value.downcast::<T>() {
            if let Some(min) = &self.min_value {
                if *v < *min {
                    return Err("Value is below minimum limit");
                }
            }
            if let Some(max) = &self.max_value {
                if *v > *max {
                    return Err("Value is above maximum limit");
                }
            }
            if !self.allowed_values.contains(&v) {
                return Err("Value is not in the list of allowed values");
            }
            self.values.lock().unwrap().insert(block_id, *v);
        } else {
            return Err("Mismatched data type")
        }
        Ok(())
    }
}