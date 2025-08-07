use std::any::Any;
use std::sync::Mutex;
use std::collections::HashMap;
use once_cell::sync::Lazy;
#[derive(Clone)]
pub enum ParameterType {
    NUMBER,
    ENUMERATION,
    BOOLEAN,
    STRING,
}

pub trait ParameterModel {
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_param_type(&self) -> ParameterType;
    fn validate_value(&self, value: &Box<dyn Any>) -> bool;

}

pub struct Parameter<T> {
    pub name: String,
    pub id: u64,
    pub value: T,
}

impl<T: Copy> Parameter<T> {
    pub fn new(name: String, id: u64, value: T) -> Self {
        Parameter {
            name,
            id,
            value,
        }
    }
    pub fn set_value(&mut self, value: T) {
        self.value = value;
    }
    pub fn get_value(&self) -> T {
        self.value
    }
}

pub struct ParameterControl {
    parameter_model_table: HashMap<String, Box<dyn ParameterModel>>,
    parameter_list: HashMap<u64, Box<dyn Any>>,
}

impl ParameterControl {
    pub fn add_parameter_model(&mut self, parameter_model: Box<dyn ParameterModel>) {
        self.parameter_model_table.insert(parameter_model.get_name(), parameter_model);
    }

    pub fn add_parameter(&mut self, model: String, block_id: u64, value: Box<dyn Any>) -> bool {
        let valid: bool;
        match self.parameter_model_table.get(&model) {
            Some(model) => valid = model.validate_value(&value),
            None => panic!("Parameter {} not found", model),
        }
        if valid {
            self.parameter_list.insert(block_id, value);
            true
        } else {
            false
        }

    }
}

pub static mut PARAMETER_CONTROL: Lazy<ParameterControl> = Lazy::new(|| {
    ParameterControl{
        parameter_model_table: HashMap::new(),
        parameter_list: HashMap::new(),
    }
});