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
  let found_lcs = find_longest_common_subsequence(&first.as_bytes().to_vec(), &second.as_bytes().to_vec(), first.len(), second.len());
  assert_eq!(found_lcs, lcs.as_bytes().to_vec());
}