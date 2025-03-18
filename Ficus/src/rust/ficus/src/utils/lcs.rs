use std::cmp::max;

pub fn find_longest_common_subsequence_length<T: PartialEq>(
  first: &Vec<T>,
  second: &Vec<T>,
  first_len: usize,
  second_len: usize,
) -> i64 {
  build_longest_common_subsequence_matrix(first, second, first_len, second_len)[first_len][second_len]
}

pub fn build_longest_common_subsequence_matrix<T: PartialEq>(
  first: &Vec<T>,
  second: &Vec<T>,
  first_len: usize,
  second_len: usize
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

pub fn find_longest_common_subsequence<T: PartialEq + Clone>(first: &Vec<T>, second: &Vec<T>, first_len: usize, second_len: usize) -> Vec<T> {
  let dp = build_longest_common_subsequence_matrix(first, second, first_len, second_len);

  let mut lcs = vec![];
  let (mut i, mut j) = (first_len, second_len);
  while i > 0 && j > 0 {
    if first[i - 1].eq(&second[j - 1]) {
      lcs.push(first[i - 1].clone());
      i -= 1;
      j -= 1;
    } else if dp[i - 1][j] > dp[i][j - 1] {
      i -= 1;
    } else {
      j -= 1;
    }
  }

  lcs.into_iter().rev().collect()
}