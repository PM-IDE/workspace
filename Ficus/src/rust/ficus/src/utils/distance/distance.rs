use std::str::FromStr;

use crate::utils::lcs::find_longest_common_subsequence_length;
use linfa_nn::distance::{Distance, L1Dist, L2Dist};
use ndarray::{ArrayView, Dimension};
use num_traits::Zero;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FicusDistance {
  Cosine,
  L1,
  L2,
  Levenshtein,
  Length,
  LCS,
}

impl FromStr for FicusDistance {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Cosine" => Ok(Self::Cosine),
      "L1" => Ok(Self::L1),
      "L2" => Ok(Self::L2),
      "Levenshtein" => Ok(Self::Levenshtein),
      "Length" => Ok(Self::Length),
      "LCS" => Ok(Self::LCS),
      _ => Err(()),
    }
  }
}

#[derive(Clone)]
pub enum DistanceWrapper {
  Cosine(CosineDistance),
  L1(L1Dist),
  L2(L2Dist),
  Levenshtein(LevenshteinDistance),
  Length(LengthDistance),
  LCS(LCSDistance),
}

impl DistanceWrapper {
  pub fn new(ficus_distance: FicusDistance) -> DistanceWrapper {
    match ficus_distance {
      FicusDistance::Cosine => DistanceWrapper::Cosine(CosineDistance),
      FicusDistance::L1 => DistanceWrapper::L1(L1Dist),
      FicusDistance::L2 => DistanceWrapper::L2(L2Dist),
      FicusDistance::Levenshtein => DistanceWrapper::Levenshtein(LevenshteinDistance),
      FicusDistance::Length => DistanceWrapper::Length(LengthDistance),
      FicusDistance::LCS => DistanceWrapper::LCS(LCSDistance),
    }
  }
}

impl Distance<f64> for DistanceWrapper {
  fn distance<D: Dimension>(&self, a: ArrayView<f64, D>, b: ArrayView<f64, D>) -> f64 {
    match self {
      DistanceWrapper::Cosine(d) => d.distance(a, b),
      DistanceWrapper::L1(d) => d.distance(a, b),
      DistanceWrapper::L2(d) => d.distance(a, b),
      DistanceWrapper::Levenshtein(d) => d.distance(a, b),
      DistanceWrapper::Length(d) => d.distance(a, b),
      DistanceWrapper::LCS(d) => d.distance(a, b)
    }
  }

  fn rdistance<D: Dimension>(&self, a: ArrayView<f64, D>, b: ArrayView<f64, D>) -> f64 {
    self.distance(a, b)
  }

  fn rdist_to_dist(&self, rdist: f64) -> f64 {
    rdist
  }

  fn dist_to_rdist(&self, dist: f64) -> f64 {
    dist
  }
}

#[derive(Clone)]
pub struct CosineDistance;

impl Distance<f64> for CosineDistance {
  fn distance<D: Dimension>(&self, a: ArrayView<f64, D>, b: ArrayView<f64, D>) -> f64 {
    let mut sum = 0.0;
    let mut a_square = 0.0;
    let mut b_square = 0.0;

    for (a, b) in a.iter().zip(b.iter()) {
      sum += a * b;
      a_square += a * a;
      b_square += b * b;
    }

    1.0 - sum / (a_square.sqrt() * b_square.sqrt())
  }
}

#[derive(Clone)]
pub struct LevenshteinDistance;

impl Distance<f64> for LevenshteinDistance {
  fn distance<D: Dimension>(&self, a: ArrayView<f64, D>, b: ArrayView<f64, D>) -> f64 {
    let a_vec = a.iter().map(|x| *x).collect::<Vec<f64>>();
    let b_vec = b.iter().map(|x| *x).collect::<Vec<f64>>();

    let a_len = Self::get_levenshtein_matrix_dimension_length(&a_vec);
    let b_len = Self::get_levenshtein_matrix_dimension_length(&b_vec);

    let mut matrix = vec![vec![0f64]];
    for i in 0..a_len {
      matrix[0].push(i as f64);
    }

    for i in 1..b_len {
      matrix.push(vec![i as f64]);
    }

    for j in 1..b_len {
      for i in 1..a_len {
        let number = if a_vec.get(i - 1).unwrap() == b_vec.get(j - 1).unwrap() {
          matrix[j - 1][i - 1]
        } else {
          matrix[j - 1][i].min(matrix[j][i - 1]).min(matrix[j - 1][i - 1]) + 1.0
        };

        matrix[j].push(number);
      }
    }

    matrix[b_len - 1][a_len - 1]
  }
}

impl LevenshteinDistance {
  fn get_levenshtein_matrix_dimension_length(vec: &Vec<f64>) -> usize {
    find_first_zero_index(vec) + 2
  }
}

fn find_first_zero_index(vec: &Vec<f64>) -> usize {
  vec.iter().position(|x| x.is_zero()).unwrap_or(vec.len() - 1)
}

#[derive(Clone, Debug)]
pub struct LengthDistance;

impl Distance<f64> for LengthDistance {
  fn distance<D: Dimension>(&self, a: ArrayView<f64, D>, b: ArrayView<f64, D>) -> f64 {
    let a_len = find_first_zero_index(&a.into_iter().map(|x| *x).collect());
    let b_len = find_first_zero_index(&b.into_iter().map(|x| *x).collect());

    (a_len.max(b_len) - a_len.min(b_len)) as f64
  }
}

#[derive(Clone)]
pub struct LCSDistance;

impl Distance<f64> for LCSDistance {
  fn distance<D: Dimension>(&self, a: ArrayView<f64, D>, b: ArrayView<f64, D>) -> f64 {
    let a_vec = a.iter().map(|x| *x).collect::<Vec<f64>>();
    let b_vec = b.iter().map(|x| *x).collect::<Vec<f64>>();

    let a_len = find_first_zero_index(&a_vec) + 1;
    let b_len = find_first_zero_index(&b_vec) + 1;

    let lcp = find_longest_common_subsequence_length(&a_vec, &b_vec, a_len, b_len) as f64;

    1. - 2. * lcp / (a_len + b_len) as f64
  }
}
