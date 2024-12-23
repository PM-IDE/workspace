use std::error::Error;
use std::fmt::Debug;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum PipelinePartExecutionError {
    Raw(RawPartExecutionError),
    MissingContext(MissingContextError),
    MissingRequiredMetadata(String),
}

impl Display for PipelinePartExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            PipelinePartExecutionError::Raw(raw_error) => Display::fmt(&raw_error, f),
            PipelinePartExecutionError::MissingContext(missing_context) => Display::fmt(&missing_context, f),
            PipelinePartExecutionError::MissingRequiredMetadata(missing_metadata) => {
                f.write_str(format!("Missing required metadata {} in the context", missing_metadata).as_str())
            }
        }
    }
}

pub struct MissingContextError {
    context_key_name: String,
}

impl MissingContextError {
    pub fn new(context_key_name: String) -> Self {
        Self { context_key_name }
    }
}

impl Display for MissingContextError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.context_key_name)
    }
}

impl Debug for MissingContextError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("MissingContextError")
            .field("context_key_name", &self.context_key_name)
            .finish()
    }
}

pub struct RawPartExecutionError {
    message: String,
}

impl Display for RawPartExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}

impl Debug for RawPartExecutionError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("PipelinePartExecutionError")
            .field("message", &self.message)
            .finish()
    }
}

impl Error for RawPartExecutionError {}

impl RawPartExecutionError {
    pub fn new(message: String) -> Self {
        Self { message }
    }
}
