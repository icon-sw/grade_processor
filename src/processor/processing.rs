use std::ffi::c_void;
use std::collections::HashMap;
use std::any::Any;
use std::sync::mpsc::{Receiver, Sender, channel};
use crate::processor::parameter::ParameterModel;

pub trait ProcessingBlockTrait {
    fn process(&self, inputs: *const c_void) -> *mut c_void;
    fn get_block_id(&self) -> u64;
    fn get_parameters_model(&self) -> HashMap<String, Box<dyn ParameterModel + Send>>;
    fn get_parameters_value(&self) -> HashMap<String, Box<dyn Any + Send>>;

    fn get_input_number(&self) -> u32;
    fn get_output_number(&self) -> u32;
    fn get_input_type(&self, input_number: u32) -> Box<dyn Any + 'static>;
    fn get_output_type(&self, output_number: u32) -> Box<dyn Any + 'static>;
}

pub struct ProcessingBlockProcessor {
    processors: HashMap<u64, Box<dyn ProcessingBlockTrait>>,
    input_receiver: Receiver<*const c_void>,
    output_sender: Sender<*mut c_void>,
}