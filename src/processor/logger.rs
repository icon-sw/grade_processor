use std::any::Any;
use std::collections::VecDeque;
use crate::processor::parameter::{Parameter, ParameterTrait};
use crate::processor::process::{ProcessTrait};

#[derive(Clone)]
pub struct LogStruct {
    pub level: u64,
    pub message: String,
}

#[derive(Clone)]
pub struct Logger {
    name: String,
    description: String,
    level: Parameter<u64>,
    path: String,
    file_prefix: String,
    log_format: String,
    entries: VecDeque<LogStruct>,
}

impl Logger {
    pub fn new(level: Parameter<u64>, 
        path: String, 
        file_prefix: Option<String>,
        log_format: Option<String>
    ) -> Self {
        Logger {
            name: String::from("Default Logger"),
            description: String::from("This is a default logger"),
            level,
            path: path.clone(),
            file_prefix: {
                if let Some(prefix) = &file_prefix {
                    format!("{}/{}.log", path, prefix)
                } else {
                    format!("{}/log_file", path)
                }
            },
            log_format: {
                if let Some(format) = &log_format {
                    format!("{}-{}", path, format)
                } else {
                    format!("{}/log", path)
                }
            },
            entries: VecDeque::new(),
        }
    }

    pub fn log(&mut self, log_entry: LogStruct) {
        let binding = self.level.get_value(0);
        let level = binding.downcast_ref::<u64>().unwrap();
        if log_entry.level < *level {
            return; // Skip logging if the log level is below the configured level
        }
        // Here you would implement the actual logging logic, e.g., writing to a file or
        println!("[Log Level {}]: {}", level, log_entry.message);
        self.entries.push_back(log_entry);
    }
}

impl ProcessTrait for Logger {
    fn process(&mut self, data: Vec<Box<dyn Any>>) -> Vec<Box<dyn Any>> {
        data.into_iter().for_each(|item| {
            if let Some(log_entry) = item.downcast_ref::<LogStruct>() {
                self.log(log_entry.clone());
            }
        });
        Vec::new()
    }
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn get_parameters(&self) -> Vec<Box<dyn ParameterTrait>> {
        let value: Box<dyn ParameterTrait> = Box::new(self.level.clone());
        vec![value]
    }
    fn get_parameter(&self, name: &str) -> Option<Box<dyn ParameterTrait>> {
        if name == self.level.name() {
            let value: Box<dyn ParameterTrait> = Box::new(self.level.clone());
            Some(value)
        } else  {
            None
        }
    }
    fn set_parameter(&mut self, name: &str, value: Box<dyn std::any::Any>, block_id: u64) -> Result<(), &str> {
        if name == self.level.name() {
            self.level.set_value(value, block_id)
        } else  {
            Err("Parameter not found")
        }
    }
}