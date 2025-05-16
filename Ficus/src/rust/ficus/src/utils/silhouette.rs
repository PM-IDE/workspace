use std::collections::HashMap;
use std::fmt::{Display, Formatter};

#[derive(PartialEq, Debug)]
pub enum SilhouetteScoreError {
  NotEnoughSamples
}

impl Display for SilhouetteScoreError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      SilhouetteScoreError::NotEnoughSamples => f.write_str("Not enough samples for silhouette score")
    }
  }
}

pub fn silhouette_score(labels: &Vec<usize>, distance_func: impl Fn(usize, usize) -> f64) -> Result<f64, SilhouetteScoreError> {
  if labels.is_empty() {
    return Err(SilhouetteScoreError::NotEnoughSamples);
  }

  let mut clusters_to_indices: HashMap<usize, Vec<usize>> = HashMap::new();
  for i in 0..labels.len() {
    let label = *labels.get(i).unwrap();
    if let Some(indices) = clusters_to_indices.get_mut(&label) {
      indices.push(i);
    } else {
      clusters_to_indices.insert(label, vec![i]);
    }
  }

  if clusters_to_indices.len() == 1 {
    return Ok(1.);
  }

  let mut score = 0.;
  for (current_cluster_index, current_cluster_indices) in &clusters_to_indices {
    for current_label in current_cluster_indices {
      let mut a_x = 0.;
      for other_index_from_this_cluster in current_cluster_indices {
        a_x += distance_func(*current_label, *other_index_from_this_cluster);
      }

      a_x = match current_cluster_indices.len() {
        1 => 0.,
        len => a_x / (len - 1) as f64
      };

      let mut b_x = None;

      for (other_cluster_index, other_cluster_indices) in &clusters_to_indices {
        if *other_cluster_index == *current_cluster_index {
          continue;
        }

        let mut current_b_x = 0.;
        for other_label_from_other_cluster in other_cluster_indices {
          current_b_x += distance_func(*current_label, *other_label_from_other_cluster);
        }

        current_b_x /= other_cluster_indices.len() as f64;

        b_x = Some(if b_x.is_none() {
          current_b_x
        } else {
          current_b_x.min(b_x.unwrap())
        })
      }

      let b_x = b_x.unwrap_or_else(|| 0.);
      score += (b_x - a_x) / a_x.max(b_x);
    }
  }

  Ok(score / labels.len() as f64)
}