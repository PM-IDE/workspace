use std::hash::Hash;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Interval<TElement, TData>
where
    TElement: PartialEq + Ord + Copy + Hash,
    TData: PartialEq + Eq + Copy,
{
    pub left: TElement,
    pub right: TElement,
    pub data: Option<TData>,
}

impl<TElement, TData> Hash for Interval<TElement, TData>
where
    TElement: PartialEq + Ord + Copy + Hash,
    TData: PartialEq + Eq + Copy,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.left.hash(state);
        self.right.hash(state);
    }
}

impl<TElement, TData> Interval<TElement, TData>
where
    TElement: PartialEq + Ord + Copy + Hash,
    TData: PartialEq + Eq + Copy,
{
    pub fn new(left: TElement, right: TElement) -> Interval<TElement, TData> {
        Interval { left, right, data: None }
    }

    pub fn new_with_data(left: TElement, right: TElement, data: Option<TData>) -> Interval<TElement, TData> {
        Interval { left, right, data }
    }
}

impl<TElement, TData> Interval<TElement, TData>
where
    TElement: PartialEq + Ord + Copy + Hash,
    TData: PartialEq + Eq + Copy,
{
    pub fn contains(&self, point: TElement) -> bool {
        self.left <= point && point <= self.right
    }
}
