use std::{format, fmt::Display};


#[derive(Debug)]
pub struct AppError {
    Kind:u32,
    Message: String,
}

impl Display for AppError  {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "application error! error code is {}, error message is {}", self.Kind, self.Message)
    }
}