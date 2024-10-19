pub trait SuffixTreeSlice<TElement>
where
    TElement: PartialEq,
{
    fn equals(&self, first: usize, second: usize) -> bool;
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<TElement>;
    fn sub_slice(&self, start: usize, end: usize) -> Option<&[TElement]>;
}

pub struct SingleWordSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq,
{
    pub slice: &'a [TElement],
}

impl<'a, TElement> SingleWordSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq,
{
    pub fn new(slice: &'a [TElement]) -> Self {
        Self { slice }
    }
}

impl<'a, TElement> SuffixTreeSlice<TElement> for SingleWordSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq + Copy,
{
    fn equals(&self, first: usize, second: usize) -> bool {
        if first >= self.slice.len() || second >= self.slice.len() {
            return false;
        }

        self.slice[first] == self.slice[second]
    }

    fn len(&self) -> usize {
        self.slice.len() + 1
    }

    fn get(&self, index: usize) -> Option<TElement> {
        if index >= self.slice.len() {
            None
        } else {
            Some(*self.slice.get(index).unwrap())
        }
    }

    fn sub_slice(&self, start: usize, end: usize) -> Option<&[TElement]> {
        Some(&self.slice[start..end])
    }
}

pub struct MultipleWordsSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq,
{
    words: Vec<&'a [TElement]>,
}

impl<'a, TElement> MultipleWordsSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq,
{
    pub fn new(words: Vec<&'a [TElement]>) -> Self {
        Self { words }
    }
}

impl<'a, TElement> SuffixTreeSlice<TElement> for MultipleWordsSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq + Copy,
{
    fn equals(&self, first: usize, second: usize) -> bool {
        self.get(first) == self.get(second)
    }

    fn len(&self) -> usize {
        let mut len = 0;
        for slice in &self.words {
            len += slice.len();
        }

        len += self.words.len();
        len
    }

    fn get(&self, index: usize) -> Option<TElement> {
        if let Some((slice_index, Some(index_in_slice))) = self.get_slice_info_for(index) {
            Some(self.words[slice_index][index_in_slice])
        } else {
            None
        }
    }

    fn sub_slice(&self, start: usize, end: usize) -> Option<&[TElement]> {
        if let Some((start_slice_index, Some(start_index_in_slice))) = self.get_slice_info_for(start) {
            if let Some((end_slice_index, end_index_in_slice)) = self.get_slice_info_for(end) {
                if start_slice_index != end_slice_index {
                    None
                } else {
                    let patched_end_index = if let Some(end_index_in_slice) = end_index_in_slice {
                        end_index_in_slice
                    } else {
                        self.words[start_slice_index].len()
                    };

                    Some(&self.words[start_slice_index][start_index_in_slice..patched_end_index])
                }
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl<'a, TElement> MultipleWordsSuffixTreeSlice<'a, TElement>
where
    TElement: PartialEq + Copy,
{
    pub fn get_slice_info_for(&self, index: usize) -> Option<(usize, Option<usize>)> {
        let mut next_word_border = 0;
        let mut slice_index = 0;
        for slice in &self.words {
            next_word_border += slice.len();
            if index < next_word_border {
                let index_in_slice = index - (next_word_border - slice.len());
                return Some((slice_index, Some(index_in_slice)));
            }

            if index == next_word_border {
                return Some((slice_index, None));
            }

            next_word_border += 1;
            slice_index += 1;
        }

        return None;
    }

    pub fn get_slice_part_len(&self, slice_part_index: usize) -> usize {
        self.words[slice_part_index].len()
    }
}
