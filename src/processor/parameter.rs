use std::any::Any;
use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;
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
    fn validate_value(&self, value: &Box<dyn Any + Send>) -> bool;

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
    parameter_model_table: HashMap<String, Box<dyn ParameterModel + Send>>,
    parameter_list: HashMap<u64, Box<dyn Any + Send>>,
}

static PARAMETER_CONTROL: OnceLock<Mutex<ParameterControl>> = OnceLock::new();

impl ParameterControl {
    pub fn get() -> &'static Mutex<ParameterControl> {
        PARAMETER_CONTROL.get_or_init(|| Mutex::new(ParameterControl {
            parameter_model_table: HashMap::new(),
            parameter_list: HashMap::new(),
        }))
    }
}

pub fn add_parameter_model(parameter_model: Box<dyn ParameterModel + Send>)  {
    let mut parameter_control = PARAMETER_CONTROL.get().expect("").lock().unwrap();
    parameter_control.parameter_model_table.insert(parameter_model.get_name(), parameter_model);
}

pub fn add_parameter(parameter_name: String, block_id: u64, value: Box<dyn Any + Send>) -> bool {
    let valid: bool;
    let mut parameter_control = PARAMETER_CONTROL.get().expect("").lock().unwrap();
    match parameter_control.parameter_model_table.get(&parameter_name) {
        Some(parameter_model) => valid = parameter_model.validate_value(&value),
        None => panic!("Parameter {} not found", parameter_name),
    }
    if valid {
        parameter_control.parameter_list.insert(block_id, value);
    }
    valid
}