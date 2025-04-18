use std::cmp::max;

pub fn find_longest_common_subsequence_length<T: PartialEq>(
  first: &Vec<T>,
  second: &Vec<T>,
  first_len: usize,
  second_len: usize,
) -> usize {
  build_longest_common_subsequence_matrix(first, second, first_len, second_len)[first_len][second_len] as usize
}

pub fn build_longest_common_subsequence_matrix<T: PartialEq>(
  first: &Vec<T>,
  second: &Vec<T>,
  first_len: usize,
  second_len: usize,
) -> Vec<Vec<i64>> {
  let mut dp = vec![vec![-1; second_len + 1]; first_len + 1];

  for i in 0..second_len + 1 { dp[0][i] = 0; }
  for i in 0..first_len + 1 { dp[i][0] = 0; }

  for i in 1..first_len + 1 {
    for j in 1..second_len + 1 {
      if first[i - 1] == second[j - 1] { dp[i][j] = 1 + dp[i - 1][j - 1]; } else { dp[i][j] = max(dp[i - 1][j], dp[i][j - 1]); }
    }
  }

  dp
}

pub struct LCSSearchResult<'a, T> {
  lcs: Vec<&'a T>,
  indices_in_first_sequence: Vec<usize>,
  indices_in_second_sequence: Vec<usize>,
}

impl<'a, T> LCSSearchResult<'a, T> {
  pub fn lcs(&self) -> &Vec<&'a T> { self.lcs.as_ref() }

  pub fn first_indices(&self) -> &Vec<usize> { self.indices_in_first_sequence.as_ref() }
  pub fn second_indices(&self) -> &Vec<usize> { self.indices_in_second_sequence.as_ref() }
}

pub fn find_longest_common_subsequence<'a, T: PartialEq + Clone>(
  first: &'a Vec<T>,
  second: &'a Vec<T>,
  first_len: usize,
  second_len: usize,
) -> LCSSearchResult<'a, T> {
  let dp = build_longest_common_subsequence_matrix(first, second, first_len, second_len);

  let mut indices_in_first_sequence = vec![];
  let mut indices_in_second_sequence = vec![];
  let mut lcs = vec![];

  let (mut i, mut j) = (first_len, second_len);
  while i > 0 && j > 0 {
    if first[i - 1].eq(&second[j - 1]) {
      lcs.push(first.get(i - 1).unwrap());
      indices_in_first_sequence.push(i - 1);
      indices_in_second_sequence.push(j - 1);

      i -= 1;
      j -= 1;
    } else if dp[i - 1][j] > dp[i][j - 1] {
      i -= 1;
    } else {
      j -= 1;
    }
  }

  LCSSearchResult {
    lcs: lcs.into_iter().rev().collect(),
    indices_in_first_sequence: indices_in_first_sequence.into_iter().rev().collect(),
    indices_in_second_sequence: indices_in_second_sequence.into_iter().rev().collect(),
  }
}