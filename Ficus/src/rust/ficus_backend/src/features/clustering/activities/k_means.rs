use std::{cell::RefCell, collections::HashMap, rc::Rc};

use linfa::{
    metrics::SilhouetteScore,
    traits::{Fit, Predict},
};
use linfa_clustering::KMeans;

use crate::{
    event_log::core::event_log::EventLog,
    features::{
        analysis::patterns::repeat_sets::ActivityNode,
        clustering::{
            common::{create_colors_vector, transform_to_ficus_dataset, ClusteredDataset, MyDataset},
            error::{ClusteringError, ClusteringResult},
        },
    },
    utils::{
        colors::ColorsHolder,
        dataset::dataset::LabeledDataset,
        distance::distance::{DistanceWrapper, FicusDistance},
    },
};

use super::{activities_common::create_dataset, activities_params::ActivitiesClusteringParams, merging::merge_activities};

pub fn clusterize_activities_k_means<TLog: EventLog>(
    params: &mut ActivitiesClusteringParams<TLog>,
    clusters_count: usize,
    iterations_count: usize,
) -> ClusteringResult {
    let (dataset, processed, classes_names) = create_dataset(&params.vis_params)?;
    let model = create_k_means_model(clusters_count, iterations_count as u64, params.tolerance, &dataset, params.distance);

    let clustered_dataset = model.predict(dataset.clone());
    merge_activities(
        params.vis_params.common_vis_params.log,
        params.vis_params.traces_activities,
        &processed.iter().map(|x| x.0.clone()).collect(),
        &clustered_dataset.targets.map(|x| Some(*x)),
    );

    let holder = &mut params.vis_params.common_vis_params.colors_holder;
    Ok(create_labeled_dataset_from_k_means(
        &dataset,
        &clustered_dataset,
        &processed,
        classes_names,
        holder,
    ))
}

fn create_labeled_dataset_from_k_means(
    dataset: &MyDataset,
    clustered_dataset: &ClusteredDataset,
    processed: &Vec<(Rc<RefCell<ActivityNode>>, HashMap<String, usize>)>,
    classes_names: Vec<String>,
    colors_holder: &mut ColorsHolder,
) -> LabeledDataset {
    let ficus_dataset = transform_to_ficus_dataset(
        dataset,
        processed.iter().map(|x| x.0.borrow().name.to_owned()).collect(),
        classes_names,
    );

    let labels = clustered_dataset.targets.clone().into_raw_vec();
    let colors = create_colors_vector(&labels, colors_holder);

    LabeledDataset::new(ficus_dataset, labels, colors)
}

fn create_k_means_model(
    clusters_count: usize,
    iterations_count: u64,
    tolerance: f64,
    dataset: &MyDataset,
    distance: FicusDistance,
) -> KMeans<f64, DistanceWrapper> {
    KMeans::params_with(clusters_count, rand::thread_rng(), DistanceWrapper::new(distance))
        .max_n_iterations(iterations_count)
        .tolerance(tolerance)
        .fit(&dataset)
        .expect("KMeans fitted")
}

pub fn clusterize_activities_k_means_grid_search<TLog: EventLog>(
    params: &mut ActivitiesClusteringParams<TLog>,
    iterations_count: usize,
) -> ClusteringResult {
    let (dataset, processed, classes_names) = create_dataset(&params.vis_params)?;

    let mut best_metric = -1f64;
    let mut best_labels = None;

    for clusters_count in 2..processed.len() {
        let model = create_k_means_model(clusters_count, iterations_count as u64, params.tolerance, &dataset, params.distance);

        let clustered_dataset = model.predict(dataset.clone());
        let score = match clustered_dataset.silhouette_score() {
            Ok(score) => score,
            Err(_) => return Err(ClusteringError::FailedToCalculateSilhouetteScore),
        };

        if score > best_metric {
            best_labels = Some(clustered_dataset.targets);
            best_metric = score;
        }
    }

    if let Some(best_labels) = best_labels.as_ref() {
        merge_activities(
            params.vis_params.common_vis_params.log,
            params.vis_params.traces_activities,
            &processed.iter().map(|x| x.0.clone()).collect(),
            &best_labels.map(|x| Some(*x)),
        );

        let ficus_dataset = transform_to_ficus_dataset(
            &dataset,
            processed.iter().map(|x| x.0.borrow().name.to_owned()).collect(),
            classes_names,
        );

        let colors = create_colors_vector(&best_labels.to_vec(), params.vis_params.common_vis_params.colors_holder);
        Ok(LabeledDataset::new(ficus_dataset, best_labels.clone().into_raw_vec(), colors))
    } else {
        Err(ClusteringError::RawError(
            "Failed to find best labels in K-means grid search".to_owned(),
        ))
    }
}
