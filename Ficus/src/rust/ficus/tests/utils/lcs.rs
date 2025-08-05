use ficus::utils::lcs::find_longest_common_subsequence;

#[test]
pub fn test_lcs_1() {
  execute_lcs_test("ABCDABDC", "XYZABCDZYZ", "ABCD");
}

#[test]
pub fn test_lcs_2() {
  execute_lcs_test("ABCBDAB", "BDCAB", "BDAB");
}

#[test]
pub fn test_lcs_3() {
  execute_lcs_test("", "ABCD", "");
}

#[test]
pub fn test_lcs_4() {
  execute_lcs_test("", "", "");
}

#[test]
pub fn test_lcs_5() {
  execute_lcs_test("XYZABCDZYZ", "XYZABCDZYZ", "XYZABCDZYZ");
}

fn execute_lcs_test(first: &str, second: &str, lcs: &str) {
  let first_bytes = first.as_bytes().to_vec();
  let second_bytes = second.as_bytes().to_vec();
  let found_lcs = find_longest_common_subsequence(&first_bytes, &second_bytes, first.len(), second.len());

  assert_eq!(found_lcs.lcs().clone(), lcs.as_bytes().iter().collect::<Vec<&u8>>());

  assert_eq!(found_lcs.first_indices().len(), found_lcs.second_indices().len());
  for (first_index, second_index) in found_lcs.first_indices().iter().zip(found_lcs.second_indices()) {
    assert_eq!(
      first.as_bytes().get(*first_index).unwrap(),
      second.as_bytes().get(*second_index).unwrap()
    )
  }
}
