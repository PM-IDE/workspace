use std::str::FromStr;

use linfa_nn::distance::{Distance, L1Dist, L2Dist};
use ndarray::{ArrayView, Dimension};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FicusDistance {
  Cosine,
  L1,
  L2,
  Levenshtein,
}

impl FromStr for FicusDistance {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    match s {
      "Cosine" => Ok(Self::Cosine),
      "L1" => Ok(Self::L1),
      "L2" => Ok(Self::L2),
      "Levenshtein" => Ok(Self::Levenshtein),
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
}

impl DistanceWrapper {
  pub fn new(ficus_distance: FicusDistance) -> DistanceWrapper {
    match ficus_distance {
      FicusDistance::Cosine => DistanceWrapper::Cosine(CosineDistance {}),
      FicusDistance::L1 => DistanceWrapper::L1(L1Dist {}),
      FicusDistance::L2 => DistanceWrapper::L2(L2Dist {}),
      FicusDistance::Levenshtein => DistanceWrapper::Levenshtein(LevenshteinDistance {}),
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
pub struct CosineDistance {}

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
pub struct LevenshteinDistance {}

impl Distance<f64> for LevenshteinDistance {
  fn distance<D: Dimension>(&self, a: ArrayView<f64, D>, b: ArrayView<f64, D>) -> f64 {
    let a_len = a.len() + 1;
    let b_len = b.len() + 1;

    let mut matrix = vec![vec![0f64]];
    for i in 0..a_len {
      matrix[0].push(i as f64);
    }

    for i in 1..b_len {
      matrix.push(vec![i as f64]);
    }

    let a_vec = a.iter().map(|x| *x).collect::<Vec<f64>>();
    let b_vec = b.iter().map(|x| *x).collect::<Vec<f64>>();

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

    matrix[a_len - 1][b_len - 1]
  }
}
