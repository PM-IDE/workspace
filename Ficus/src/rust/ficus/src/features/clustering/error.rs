use crate::{
  pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
  utils::dataset::dataset::LabeledDataset,
};
use std::fmt::Display;

pub type ClusteringResult = Result<LabeledDataset, ClusteringError>;

pub enum ClusteringError {
  NoRepeatSet,
  FailedToCreateNdArray,
  FailedToCalculateSilhouetteScore,
  RawError(String),
}

impl From<ClusteringError> for PipelinePartExecutionError {
  fn from(val: ClusteringError) -> Self {
    PipelinePartExecutionError::Raw(RawPartExecutionError::new(val.to_string()))
  }
}

impl Display for ClusteringError {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    f.write_str(
      match self {
        Self::NoRepeatSet => "NoRepeatSet".to_owned(),
        Self::FailedToCreateNdArray => "FailedToCreateNdArray".to_owned(),
        Self::FailedToCalculateSilhouetteScore => "FailedToCalculateSilhouetteScore".to_owned(),
        Self::RawError(message) => message.clone(),
      }
      .as_str(),
    )
  }
}
