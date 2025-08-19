use std::ffi::c_void;
use std::collections::HashMap;
use std::any::Any;

use crate::processor::parameter::ParameterModel;

pub trait ProcessingBlockTrait {
    fn process(&self, inputs: *const c_void) -> *mut c_void;
    fn get_block_id(&self) -> u64;
    fn get_parameters_model(&self) -> HashMap<String, Box<dyn ParameterModel + Send>>;
    fn get_parameters_value(&self) -> HashMap<String, Box<dyn Any + Send>>;
}