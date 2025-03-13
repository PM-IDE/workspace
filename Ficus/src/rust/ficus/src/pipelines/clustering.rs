use crate::event_log::xes::xes_event_log::XesEventLogImpl;
use crate::features::clustering::activities::activities_common::create_dataset;
use crate::features::clustering::activities::activities_params::{ActivitiesClusteringParams, ActivitiesVisualizationParams};
use crate::features::clustering::activities::dbscan::clusterize_activities_dbscan;
use crate::features::clustering::activities::k_means::{clusterize_activities_k_means, clusterize_activities_k_means_grid_search};
use crate::features::clustering::common::{transform_to_ficus_dataset, CommonVisualizationParams};
use crate::features::clustering::traces::dbscan::clusterize_log_by_traces_dbscan;
use crate::features::clustering::traces::k_means::clusterize_log_by_traces_kmeans_grid_search;
use crate::features::clustering::traces::traces_params::{FeatureCountKind, TracesClusteringParams};
use crate::pipelines::context::PipelineContext;
use crate::pipelines::errors::pipeline_errors::{PipelinePartExecutionError, RawPartExecutionError};
use crate::pipelines::keys::context_keys::{ACTIVITIES_REPR_SOURCE_KEY, ACTIVITY_LEVEL_KEY, CLUSTERS_COUNT_KEY, COLORS_HOLDER_KEY, DISTANCE_KEY, EVENT_CLASS_REGEX_KEY, EVENT_LOG_KEY, FEATURE_COUNT_KIND_KEY, LABELED_LOG_TRACES_DATASET_KEY, LABELED_TRACES_ACTIVITIES_DATASET_KEY, LEARNING_ITERATIONS_COUNT_KEY, MIN_EVENTS_IN_CLUSTERS_COUNT_KEY, PERCENT_FROM_MAX_VALUE_KEY, PIPELINE_KEY, TOLERANCE_KEY, TRACES_ACTIVITIES_DATASET_KEY, TRACES_REPRESENTATION_SOURCE_KEY, TRACE_ACTIVITIES_KEY};
use crate::pipelines::multithreading::FeatureCountKindDto;
use crate::pipelines::pipeline_parts::PipelineParts;
use crate::pipelines::pipelines::{PipelinePart, PipelinePartFactory};
use crate::utils::user_data::user_data::{UserData, UserDataImpl};

impl PipelineParts {
  pub(super) fn clusterize_activities_from_traces_k_means() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::CLUSTERIZE_ACTIVITIES_FROM_TRACES_KMEANS, &|context, _, config| {
      let mut params = Self::create_activities_clustering_params(context, config)?;
      let clusters_count = *Self::get_user_data(config, &CLUSTERS_COUNT_KEY)? as usize;
      let learning_iterations_count = *Self::get_user_data(config, &LEARNING_ITERATIONS_COUNT_KEY)? as usize;

      let labeled_dataset = match clusterize_activities_k_means(&mut params, clusters_count, learning_iterations_count) {
        Ok(labeled_dataset) => labeled_dataset,
        Err(error) => return Err(error.into()),
      };

      context.put_concrete(LABELED_TRACES_ACTIVITIES_DATASET_KEY.key(), labeled_dataset);
      Ok(())
    })
  }

  fn create_common_vis_params<'a>(
    context: &'a PipelineContext,
    config: &'a UserDataImpl,
  ) -> Result<CommonVisualizationParams<'a, XesEventLogImpl>, PipelinePartExecutionError> {
    let log = Self::get_user_data(context, &EVENT_LOG_KEY)?;
    let colors_holder = Self::get_user_data_mut(context, &COLORS_HOLDER_KEY)?;
    let class_extractor = match Self::get_user_data(config, &EVENT_CLASS_REGEX_KEY) {
      Ok(extractor) => Some(extractor.to_owned()),
      Err(_) => None,
    };

    Ok(CommonVisualizationParams {
      log,
      colors_holder,
      class_extractor,
    })
  }

  fn create_activities_visualization_params<'a>(
    context: &'a mut PipelineContext,
    config: &'a UserDataImpl,
  ) -> Result<ActivitiesVisualizationParams<'a, XesEventLogImpl>, PipelinePartExecutionError> {
    let common_vis_params = Self::create_common_vis_params(context, config)?;
    let traces_activities = Self::get_user_data_mut(context, &TRACE_ACTIVITIES_KEY)?;
    let activity_level = *Self::get_user_data(config, &ACTIVITY_LEVEL_KEY)? as usize;
    let activities_repr_source = *Self::get_user_data(config, &ACTIVITIES_REPR_SOURCE_KEY)?;

    Ok(ActivitiesVisualizationParams {
      common_vis_params,
      traces_activities,
      activity_level,
      activities_repr_source,
    })
  }

  fn create_activities_clustering_params<'a>(
    context: &'a mut PipelineContext,
    config: &'a UserDataImpl,
  ) -> Result<ActivitiesClusteringParams<'a, XesEventLogImpl>, PipelinePartExecutionError> {
    let vis_params = Self::create_activities_visualization_params(context, config)?;
    let tolerance = *Self::get_user_data(config, &TOLERANCE_KEY)?;
    let distance = *Self::get_user_data(config, &DISTANCE_KEY)?;

    if let Some(params) = ActivitiesClusteringParams::new(vis_params, tolerance, distance) {
      Ok(params)
    } else {
      let message = "Failed to create activities clustering params".to_owned();
      Err(PipelinePartExecutionError::Raw(RawPartExecutionError::new(message)))
    }
  }

  pub(super) fn clusterize_activities_from_traces_k_means_grid_search() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::CLUSTERIZE_ACTIVITIES_FROM_TRACES_KMEANS_GRID_SEARCH, &|context, _, config| {
      let learning_iterations_count = *Self::get_user_data(config, &LEARNING_ITERATIONS_COUNT_KEY)? as usize;
      let mut params = Self::create_activities_clustering_params(context, config)?;

      let labeled_dataset = match clusterize_activities_k_means_grid_search(&mut params, learning_iterations_count) {
        Ok(labeled_dataset) => labeled_dataset,
        Err(error) => return Err(error.into()),
      };

      context.put_concrete(LABELED_TRACES_ACTIVITIES_DATASET_KEY.key(), labeled_dataset);
      Ok(())
    })
  }

  pub(super) fn clusterize_activities_from_traces_dbscan() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::CLUSTERIZE_ACTIVITIES_FROM_TRACES_DBSCAN, &|context, _, config| {
      let min_points_in_cluster = *Self::get_user_data(config, &MIN_EVENTS_IN_CLUSTERS_COUNT_KEY)? as usize;
      let mut params = Self::create_activities_clustering_params(context, config)?;

      let labeled_dataset = match clusterize_activities_dbscan(&mut params, min_points_in_cluster) {
        Ok(labeled_dataset) => labeled_dataset,
        Err(error) => return Err(error.into()),
      };

      context.put_concrete(LABELED_TRACES_ACTIVITIES_DATASET_KEY.key(), labeled_dataset);
      Ok(())
    })
  }

  pub fn clusterize_log_by_traces_k_means_grid_search() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::CLUSTERIZE_LOG_TRACES_K_MEANS_GRID_SEARCH, &|context, infra, config| {
      let mut params = Self::create_traces_clustering_params(context, config)?;
      let learning_iterations_count = *Self::get_user_data(config, &LEARNING_ITERATIONS_COUNT_KEY)? as u64;

      let (logs, labeled_dataset) = match clusterize_log_by_traces_kmeans_grid_search(&mut params, learning_iterations_count) {
        Ok(new_logs) => new_logs,
        Err(error) => return Err(error.into()),
      };

      if let Some(after_clusterization_pipeline) = Self::get_user_data(config, &PIPELINE_KEY).ok() {
        for log in logs {
          let mut new_context = context.clone();
          new_context.put_concrete(EVENT_LOG_KEY.key(), log);

          after_clusterization_pipeline.execute(&mut new_context, infra)?;
        }
      }

      context.put_concrete(LABELED_LOG_TRACES_DATASET_KEY.key(), labeled_dataset);

      Ok(())
    })
  }

  pub(super) fn create_traces_activities_dataset() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::CREATE_TRACES_ACTIVITIES_DATASET, &|context, _, config| {
      let params = Self::create_activities_visualization_params(context, config)?;

      let (dataset, processed, classes) = match create_dataset(&params) {
        Ok((dataset, processed, classes)) => (dataset, processed, classes),
        Err(error) => return Err(error.into()),
      };

      let processed = processed.iter().map(|x| x.0.borrow().name().as_ref().as_ref().to_owned()).collect();
      let ficus_dataset = transform_to_ficus_dataset(&dataset, processed, classes);

      context.put_concrete(TRACES_ACTIVITIES_DATASET_KEY.key(), ficus_dataset);
      Ok(())
    })
  }

  pub(crate) fn create_traces_clustering_params<'a>(
    context: &'a mut PipelineContext,
    config: &'a UserDataImpl,
  ) -> Result<TracesClusteringParams<'a, XesEventLogImpl>, PipelinePartExecutionError> {
    let tolerance = *Self::get_user_data(config, &TOLERANCE_KEY)?;
    let distance = *Self::get_user_data(config, &DISTANCE_KEY)?;
    let repr_source = *Self::get_user_data(config, &TRACES_REPRESENTATION_SOURCE_KEY)?;
    let feature_count_kind = *Self::get_user_data(config, &FEATURE_COUNT_KIND_KEY)?;

    let feature_count_kind = match feature_count_kind {
      FeatureCountKindDto::One => FeatureCountKind::One,
      FeatureCountKindDto::Count => FeatureCountKind::Count,
      FeatureCountKindDto::OneIfMoreThanMaxFromAllFeatures => {
        let percent = *Self::get_user_data(config, &PERCENT_FROM_MAX_VALUE_KEY)?;
        FeatureCountKind::OneIfMoreThanMaxFromAllFeatures(percent)
      }
    };

    Ok(TracesClusteringParams {
      vis_params: Self::create_common_vis_params(context, config)?,
      distance,
      tolerance,
      repr_source,
      feature_count_kind,
    })
  }

  pub(super) fn clusterize_log_traces() -> (String, PipelinePartFactory) {
    Self::create_pipeline_part(Self::CLUSTERIZE_LOG_TRACES, &|context, infra, config| {
      let mut params = Self::create_traces_clustering_params(context, config)?;
      let after_clusterization_pipeline = Self::get_user_data(config, &PIPELINE_KEY)?;
      let min_points_in_cluster = *Self::get_user_data(config, &MIN_EVENTS_IN_CLUSTERS_COUNT_KEY)? as usize;

      let new_logs = match clusterize_log_by_traces_dbscan(&mut params, min_points_in_cluster) {
        Ok(new_logs) => new_logs,
        Err(error) => return Err(error.into()),
      };

      context.put_concrete(LABELED_LOG_TRACES_DATASET_KEY.key(), new_logs.1);
      for log in new_logs.0 {
        let mut new_context = context.clone();
        new_context.put_concrete(EVENT_LOG_KEY.key(), log);

        after_clusterization_pipeline.execute(&mut new_context, infra)?;
      }

      Ok(())
    })
  }
}