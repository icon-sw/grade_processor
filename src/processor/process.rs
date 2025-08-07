use std::any::Any;
use std::collections::HashMap;
use crate::processor::parameter::ParameterTrait;

pub trait ProcessTrait {
    fn process(&mut self, inputs: Vec<Box<dyn Any>>) -> Vec<Box<dyn Any>>;
    fn get_name(&self) -> String;
    fn get_description(&self) -> String;
    fn get_parameters(&self) -> Vec<Box<dyn ParameterTrait>>;
    fn get_parameter(&self, name: &str) -> Option<Box<dyn ParameterTrait>>;
    fn set_parameter(&mut self, name: &str, value: Box<dyn Any>, block_id: u64) -> Result<(), &str>;
}

pub struct ProcessSequence {
    processes: HashMap<u64, Box<dyn ProcessTrait>>,
}

impl ProcessSequence {
    pub fn new() -> Self {
        ProcessSequence {
            processes: HashMap::new(),
        }
    }

    pub fn add_process(&mut self, process: Box<dyn ProcessTrait>, block_id: Option<u64>) {
        let id = block_id.unwrap_or(self.processes.len() as u64);
        if self.processes.contains_key(&id) {
            panic!("Process with ID {} already exists", id);
        }
        if id == 0 {
            panic!("Process ID 0 is reserved and cannot be used");
        }
        self.processes.insert(id, process);
    }

    pub fn execute(&mut self, inputs: Vec<Box<dyn Any>>) -> Vec<Box<dyn Any>> {
        let mut current_inputs = inputs;
        for (id, process) in &mut self.processes {
            println!("Executing process {} with ID {}", process.get_name(), id);
            current_inputs = process.process(current_inputs);
        }
        current_inputs
    }
}