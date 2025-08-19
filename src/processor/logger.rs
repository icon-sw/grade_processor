use std::any::Any;
use std::sync::OnceLock;
use crate::processor::parameter::{ParameterType, ParameterModel, Parameter,
                                  add_parameter_model, add_parameter};

#[derive(Copy, Clone, PartialEq)]
pub enum LogLevel {
    Emergency,
    Alert,
    Critical,
    Error,
    Warning,
    Notice,
    Info,
    Debug,
}

#[derive(Clone)]
pub struct  LogLevelParameter {
    name: String,
    description: String,
    parameter_type: ParameterType,
    default_level: LogLevel,
    allowed_levels: Vec<LogLevel>,
    min_value: LogLevel,
    max_value: LogLevel
}
impl LogLevelParameter {
    pub fn new() -> LogLevelParameter {
        let log_level_param = LogLevelParameter {
            name: "LogLevel".to_string(),
            description: "Set the level of log to apply".to_string(),
            parameter_type: ParameterType::ENUMERATION,
            default_level: LogLevel::Error,
            allowed_levels: {
                let mut all = Vec::new();
                all.push(LogLevel::Emergency);
                all.push(LogLevel::Alert);
                all.push(LogLevel::Critical);
                all.push(LogLevel::Error);
                all.push(LogLevel::Warning);
                all.push(LogLevel::Notice);
                all.push(LogLevel::Info);
                all.push(LogLevel::Debug);

                all
            },
            min_value: LogLevel::Debug,
            max_value: LogLevel::Emergency,
        };
        let box_param: Box<dyn ParameterModel+Send+'static> = Box::new(log_level_param.clone());
        add_parameter_model(box_param);
        log_level_param
    }
    pub fn get_min_value(&self) -> LogLevel {
        self.min_value
    }
    pub fn get_max_value(&self) -> LogLevel {
        self.max_value
    }
    pub fn get_allowed_values(&self) -> &Vec<LogLevel> {
        &self.allowed_levels
    }
}
impl ParameterModel for LogLevelParameter {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_description(&self) -> String {
        self.description.clone()
    }
    fn get_param_type(&self) -> ParameterType {
        self.parameter_type.clone()
    }
    fn validate_value(&self, value: &Box<dyn Any + Send>) -> bool {
        let value = value.downcast_ref::<LogLevel>().unwrap();

        if self.get_allowed_values().contains(value) {
            true
        } else {
            false
        }
    }
}
pub struct Logger {
    log_level: Parameter<LogLevel>,
}

impl Logger {
    pub fn new(block_id: u64, log_level: LogLevel) -> Self {
        static LOG_LEVEL_PARAM: OnceLock<LogLevelParameter> = OnceLock::new();
        LOG_LEVEL_PARAM.get_or_init(|| LogLevelParameter::new());
        let value: Box<dyn Any + Send> = Box::new(log_level);
            if !add_parameter("LogLevel".to_string(), block_id, value) {
                panic!("Not allowed value")
            }
        Logger{
            log_level: Parameter::new("LogLevel".to_string(),block_id,log_level)
        }
    }
}