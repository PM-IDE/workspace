pub fn sort_by_first<TValue>(vec: &mut Vec<(&String, TValue)>) {
    vec.sort_by(|first, second| first.0.partial_cmp(second.0).unwrap());
}
