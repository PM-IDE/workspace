use crate::{
  pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError},
  utils::dataset::dataset::LabeledDataset,
};

pub type ClusteringResult = Result<LabeledDataset, ClusteringError>;

#[derive(Debug)]
pub enum ClusteringError {
  NoRepeatSet,
  FailedToCreateNdArray,
  FailedToCalculateSilhouetteScore,
  RawError(String),
}

impl From<ClusteringError> for PipelinePartExecutionError {
  fn from(err: ClusteringError) -> Self {
    PipelinePartExecutionError::Raw(RawPartExecutionError::new(format!("{err:?}")))
  }
}
