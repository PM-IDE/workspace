use crate::{
  pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
  utils::dataset::dataset::LabeledDataset,
};

pub type ClusteringResult = Result<LabeledDataset, ClusteringError>;

pub enum ClusteringError {
  NoRepeatSet,
  FailedToCreateNdArray,
  FailedToCalculateSilhouetteScore,
  RawError(String),
}

impl Into<PipelinePartExecutionError> for ClusteringError {
  fn into(self) -> PipelinePartExecutionError {
    PipelinePartExecutionError::Raw(RawPartExecutionError::new(self.to_string()))
  }
}

impl ToString for ClusteringError {
  fn to_string(&self) -> String {
    match self {
      Self::NoRepeatSet => "NoRepeatSet".to_owned(),
      Self::FailedToCreateNdArray => "FailedToCreateNdArray".to_owned(),
      Self::FailedToCalculateSilhouetteScore => "FailedToCalculateSilhouetteScore".to_owned(),
      Self::RawError(message) => message.clone(),
    }
  }
}
