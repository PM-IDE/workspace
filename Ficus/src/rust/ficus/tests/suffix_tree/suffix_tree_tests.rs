use std::{fmt::Debug, fs};

use ficus::{
    utils::suffix_tree::{
        suffix_tree_patterns::SuffixTree,
        suffix_tree_slice::{MultipleWordsSuffixTreeSlice, SingleWordSuffixTreeSlice, SuffixTreeSlice},
    },
    vecs,
};

use crate::test_core::{
    gold_based_test::execute_test_with_gold,
    simple_events_logs_provider::{
        create_max_repeats_trace_1, create_max_repeats_trace_2, create_max_repeats_trace_3, create_max_repeats_trace_4,
        create_max_repeats_trace_5,
    },
    test_paths::{create_suffix_tree_gold_file_path, get_paths_to_suffix_tree_string},
};

//ref impl: http://e-maxx.ru/algo/ukkonen
#[test]
fn test_suffix_tree_against_ref_impl() {
    for file_path in get_paths_to_suffix_tree_string() {
        let file_name = file_path.file_stem().unwrap().to_str().unwrap();
        execute_test_with_gold(create_suffix_tree_gold_file_path(file_name), || {
            let mut file_string = fs::read_to_string(file_path).ok().unwrap();

            //remove last symbol as it is non-existing symbol for ref impl, but our impl
            //adds at implicitly
            file_string.remove(file_string.len() - 1);
            let slice = SingleWordSuffixTreeSlice::new(file_string.as_bytes());
            let mut tree = SuffixTree::new(&slice);
            tree.build_tree();

            let mut test_value = String::new();
            for node in tree.dump_nodes() {
                let parent = match node.2 {
                    Some(value) => value as i64,
                    None => -1,
                };

                let link = match node.3 {
                    Some(value) => value as i64,
                    None => -1,
                };

                let serialized_node = format!("({} {} {} {})\n", node.0, node.1, parent, link);
                test_value.push_str(serialized_node.as_str());
            }

            test_value
        });
    }
}

fn dump_repeats_to_string(slice: &dyn SuffixTreeSlice<u8>, repeats: &Vec<(usize, usize)>) -> Vec<String> {
    let mut dump = vec![];

    for (left, right) in repeats {
        dump.push(
            String::from_utf8(slice.sub_slice(*left, *right).unwrap().iter().map(|e| *e).collect())
                .ok()
                .unwrap(),
        )
    }

    dump.sort();
    dump
}

#[test]
fn test_maximal_repeats() {
    execute_test_with_tuple_dump(
        "djksadlasdjaslkdj".as_bytes(),
        |_, tree| tree.find_maximal_repeats(),
        vec![(0, 1), (0, 2), (2, 3), (3, 4), (4, 5), (6, 7), (7, 9)],
    );
}

fn execute_test_with_tuple_dump<TFinder, TValue>(text: &[u8], finder: TFinder, expected: Vec<TValue>)
where
    TFinder: Fn(&dyn SuffixTreeSlice<u8>, &SuffixTree<u8>) -> Vec<TValue>,
    TValue: PartialEq + Debug,
{
    let slice = SingleWordSuffixTreeSlice::new(text);
    let mut tree = SuffixTree::new(&slice);
    tree.build_tree();

    assert_eq!(finder(&slice, &tree), expected)
}

#[test]
fn test_maximal_repeats_string() {
    execute_test_with_tuple_dump(
        "djksadlasdjaslkdj".as_bytes(),
        |slice, tree| dump_repeats_to_string(slice, &tree.find_maximal_repeats()),
        vecs!["a", "as", "d", "dj", "k", "l", "s"],
    );
}

#[test]
fn test_maximal_repeats2() {
    execute_test_with_tuple_dump("abcdxabcyabcz".as_bytes(), |_, tree| tree.find_maximal_repeats(), vec![(0, 3)]);
}

#[test]
fn test_maximal_repeats2_string() {
    execute_test_with_tuple_dump(
        "abcdxabcyabcz".as_bytes(),
        |slice, tree| dump_repeats_to_string(slice, &tree.find_maximal_repeats()),
        vecs!["abc"],
    );
}

#[test]
fn test_maximal_repeats3() {
    execute_test_with_tuple_dump(
        "aaacdcdcbedbccbadbdebdc".as_bytes(),
        |_, tree| tree.find_maximal_repeats(),
        vec![
            (0, 1),
            (0, 2),
            (3, 4),
            (3, 6),
            (4, 5),
            (4, 6),
            (7, 9),
            (8, 9),
            (9, 10),
            (10, 12),
            (17, 19),
        ],
    );
}

#[test]
fn test_maximal_repeats3_string() {
    execute_test_with_tuple_dump(
        "aaacdcdcbedbccbadbdebdc".as_bytes(),
        |slice, tree| dump_repeats_to_string(slice, &tree.find_maximal_repeats()),
        vecs!["a", "aa", "b", "bd", "c", "cb", "cdc", "d", "db", "dc", "e"],
    );
}

#[test]
fn test_maximal_repeats4() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_1(),
        |_, tree| tree.find_maximal_repeats(),
        vec![(0, 1), (2, 3), (2, 5)],
    );
}

#[test]
fn test_maximal_repeats4_string() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_1(),
        |slice, tree| dump_repeats_to_string(slice, &tree.find_maximal_repeats()),
        vecs!["a", "b", "bcd"],
    );
}

#[test]
fn test_maximal_repeats5() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_2(),
        |_, tree| tree.find_maximal_repeats(),
        vec![(0, 4), (2, 3)],
    );
}

#[test]
fn test_maximal_repeats5_string() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_2(),
        |slice, tree| dump_repeats_to_string(slice, &tree.find_maximal_repeats()),
        vecs!["b", "dabc"],
    );
}

#[test]
fn test_super_maximal_repeats() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_1(),
        |_, tree| tree.find_super_maximal_repeats(),
        vec![(0, 1), (2, 5)],
    );
}

#[test]
fn test_super_maximal_repeats_string() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_1(),
        |slice, tree| dump_repeats_to_string(slice, &tree.find_super_maximal_repeats()),
        vecs!["a", "bcd"],
    );
}

#[test]
fn test_super_maximal_repeats3() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_3(),
        |_, tree| tree.find_super_maximal_repeats(),
        vec![(0, 4), (10, 11)],
    );
}

#[test]
fn test_super_maximal_repeats3_string() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_3(),
        |slice, tree| dump_repeats_to_string(slice, &tree.find_super_maximal_repeats()),
        vecs!["a", "bbbc"],
    );
}

#[test]
fn test_super_maximal_repeats4() {
    let slice = MultipleWordsSuffixTreeSlice::new(vec!["aaax".as_bytes(), "aaay".as_bytes()]);
    let mut tree = SuffixTree::new(&slice);
    tree.build_tree();

    assert_eq!(tree.find_maximal_repeats(), [(0, 2), (0, 3), (1, 2)])
}

#[test]
fn test_near_super_maximal_repeats() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_1(),
        |_, tree| tree.find_near_super_maximal_repeats(),
        vec![(0, 1), (2, 3), (2, 5)],
    );
}

#[test]
fn test_near_super_maximal_repeats_string() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_1(),
        |slice, tree| dump_repeats_to_string(slice, &tree.find_near_super_maximal_repeats()),
        vecs!["a", "b", "bcd"],
    );
}

#[test]
fn test_near_super_maximal_repeats2() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_2(),
        |_, tree| tree.find_near_super_maximal_repeats(),
        vec![(0, 4), (2, 3)],
    );
}

#[test]
fn test_near_super_maximal_repeats2_string() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_2(),
        |slice, tree| dump_repeats_to_string(slice, &tree.find_near_super_maximal_repeats()),
        vecs!["b", "dabc"],
    );
}

#[test]
fn test_near_super_maximal_repeats3() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_3(),
        |_, tree| tree.find_near_super_maximal_repeats(),
        vec![(0, 4), (3, 4), (10, 11)],
    );
}

#[test]
fn test_near_super_maximal_repeats3_string() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_3(),
        |slice, tree| dump_repeats_to_string(slice, &tree.find_near_super_maximal_repeats()),
        vecs!["a", "bbbc", "c"],
    );
}

#[test]
fn test_near_super_maximal_repeats4() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_4(),
        |_, tree| tree.find_near_super_maximal_repeats(),
        vec![(0, 1), (0, 2), (5, 6), (7, 9)],
    );
}

#[test]
fn test_near_super_maximal_repeats4_string() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_4(),
        |slice, tree| dump_repeats_to_string(slice, &tree.find_near_super_maximal_repeats()),
        vecs!["a", "aa", "b", "cc"],
    );
}

#[test]
fn test_near_super_maximal_repeats6() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_5(),
        |_, tree| tree.find_near_super_maximal_repeats(),
        vec![(0, 1), (0, 2), (3, 4), (3, 6), (4, 6), (7, 9), (9, 10), (10, 12), (17, 19)],
    );
}

#[test]
fn test_near_super_maximal_repeats6_string() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_5(),
        |slice, tree| dump_repeats_to_string(slice, &tree.find_near_super_maximal_repeats()),
        vecs!["a", "aa", "bd", "c", "cb", "cdc", "db", "dc", "e"],
    );
}

#[test]
fn test_multiple_words_suffix_tree_slice() {
    let slices = vec!["abc".as_bytes(), "fsd".as_bytes()];
    let slice = MultipleWordsSuffixTreeSlice::new(slices);
    let mut tree = SuffixTree::new(&slice);
    tree.build_tree();

    assert_eq!(tree.find_patterns("abc".as_bytes()).unwrap(), [(0, 3)]);
    assert_eq!(tree.find_patterns("fsd".as_bytes()).unwrap(), [(4, 7)]);
    assert_eq!(tree.find_patterns("f".as_bytes()).unwrap(), [(4, 5)]);
}

#[test]
fn test_patterns_search() {
    execute_test_with_tuple_dump(
        "abcdxabcyabcz".as_bytes(),
        |_, tree| tree.find_patterns("abc".as_bytes()).unwrap(),
        vec![(0, 3), (5, 8), (9, 12)],
    );
}

#[test]
fn test_patterns_search2() {
    execute_test_with_tuple_dump(
        create_max_repeats_trace_5(),
        |_, tree| tree.find_patterns("badb".as_bytes()).unwrap(),
        vec![(14, 18)],
    );
}

#[test]
fn test_patterns_search3() {
    execute_test_with_tuple_dump(
        "abcdxabcyabcz".as_bytes(),
        |_, tree| tree.find_patterns("a".as_bytes()).unwrap(),
        vec![(0, 1), (5, 6), (9, 10)],
    );
}

#[test]
fn test_patterns_search4() {
    execute_test_with_tuple_dump(
        "xabxac".as_bytes(),
        |_, tree| tree.find_patterns("xa".as_bytes()).unwrap(),
        vec![(0, 2), (3, 5)],
    );
}

#[test]
fn test_patterns_search5() {
    execute_test_with_tuple_dump(
        "bbbcdbbbccaa".as_bytes(),
        |_, tree| tree.find_patterns("bb".as_bytes()).unwrap(),
        vec![(0, 2), (1, 3), (5, 7), (6, 8)],
    );
}

#[test]
pub fn test_suffix_tree_nodes() {
    execute_test_with_tuple_dump(
        "xabxac".as_bytes(),
        |_, tree| tree.dump_nodes(),
        vec![
            (0, 0, None, None),
            (2, 7, Some(4), None),
            (2, 7, Some(6), None),
            (2, 7, Some(0), None),
            (0, 2, Some(0), Some(6)),
            (5, 7, Some(4), None),
            (1, 2, Some(0), Some(0)),
            (5, 7, Some(6), None),
            (5, 7, Some(0), None),
            (6, 7, Some(0), None),
        ],
    );
}

#[test]
pub fn test_suffix_tree_nodes2() {
    execute_test_with_tuple_dump(
        "dasdasdasasasdasdasasd".as_bytes(),
        |_, tree| tree.dump_nodes(),
        vec![
            (0, 0, None, None),
            (6, 23, Some(4), None),
            (11, 23, Some(20), None),
            (11, 23, Some(22), None),
            (3, 6, Some(10), Some(6)),
            (11, 23, Some(24), None),
            (4, 6, Some(36), Some(8)),
            (11, 23, Some(26), None),
            (4, 6, Some(38), Some(10)),
            (11, 23, Some(28), None),
            (1, 3, Some(40), Some(12)),
            (11, 23, Some(30), None),
            (1, 3, Some(0), Some(14)),
            (11, 23, Some(16), None),
            (2, 3, Some(0), Some(0)),
            (11, 23, Some(18), None),
            (9, 11, Some(12), Some(18)),
            (14, 23, Some(32), None),
            (9, 11, Some(14), Some(12)),
            (14, 23, Some(34), None),
            (6, 11, Some(6), Some(22)),
            (21, 23, Some(20), None),
            (6, 11, Some(8), Some(24)),
            (21, 23, Some(22), None),
            (9, 11, Some(4), Some(26)),
            (21, 23, Some(24), None),
            (9, 11, Some(6), Some(28)),
            (21, 23, Some(26), None),
            (9, 11, Some(8), Some(30)),
            (21, 23, Some(28), None),
            (9, 11, Some(10), Some(16)),
            (21, 23, Some(30), None),
            (13, 14, Some(16), Some(34)),
            (22, 23, Some(32), None),
            (13, 14, Some(18), Some(36)),
            (22, 23, Some(34), None),
            (3, 4, Some(12), Some(38)),
            (22, 23, Some(36), None),
            (3, 4, Some(14), Some(40)),
            (22, 23, Some(38), None),
            (0, 1, Some(0), Some(0)),
            (22, 23, Some(40), None),
            (22, 23, Some(0), None),
        ],
    );
}

#[test]
pub fn test_suffix_tree_nodes3() {
    execute_test_with_tuple_dump(
        "asjkldhoiufjaksdjkasfgahabvasdrfaoasdfuabjikdu".as_bytes(),
        |_, tree| tree.dump_nodes(),
        vec![
            (0, 0, None, None),
            (2, 47, Some(25), None),
            (2, 47, Some(18), None),
            (4, 47, Some(22), None),
            (4, 47, Some(16), None),
            (4, 47, Some(0), None),
            (6, 47, Some(20), None),
            (7, 47, Some(32), None),
            (8, 47, Some(44), None),
            (9, 47, Some(58), None),
            (10, 47, Some(51), None),
            (11, 47, Some(28), None),
            (2, 3, Some(0), Some(0)),
            (12, 47, Some(12), None),
            (0, 1, Some(0), Some(0)),
            (13, 47, Some(14), None),
            (3, 4, Some(0), Some(0)),
            (14, 47, Some(16), None),
            (1, 2, Some(0), Some(0)),
            (16, 47, Some(38), None),
            (5, 6, Some(0), Some(0)),
            (16, 47, Some(20), None),
            (3, 4, Some(12), Some(16)),
            (18, 47, Some(22), None),
            (18, 47, Some(16), None),
            (1, 2, Some(14), Some(18)),
            (20, 47, Some(25), None),
            (20, 47, Some(18), None),
            (10, 11, Some(0), Some(0)),
            (21, 47, Some(28), None),
            (21, 47, Some(0), None),
            (23, 47, Some(14), None),
            (6, 7, Some(0), Some(0)),
            (24, 47, Some(32), None),
            (26, 47, Some(53), None),
            (26, 47, Some(55), None),
            (26, 47, Some(0), None),
            (30, 47, Some(46), None),
            (15, 16, Some(18), Some(20)),
            (30, 47, Some(38), None),
            (30, 47, Some(20), None),
            (30, 47, Some(0), None),
            (32, 47, Some(28), None),
            (33, 47, Some(14), None),
            (7, 8, Some(0), Some(0)),
            (34, 47, Some(44), None),
            (29, 30, Some(25), Some(38)),
            (37, 47, Some(46), None),
            (37, 47, Some(38), None),
            (37, 47, Some(20), None),
            (38, 47, Some(28), None),
            (9, 10, Some(0), Some(0)),
            (39, 47, Some(51), None),
            (25, 26, Some(14), Some(55)),
            (41, 47, Some(53), None),
            (25, 26, Some(0), Some(0)),
            (41, 47, Some(55), None),
            (42, 47, Some(12), None),
            (8, 9, Some(0), Some(0)),
            (43, 47, Some(58), None),
            (44, 47, Some(16), None),
            (45, 47, Some(20), None),
            (46, 47, Some(51), None),
            (46, 47, Some(0), None),
        ],
    );
}
