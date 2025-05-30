use std::ops::{Bound, Range, RangeBounds};

pub trait RangeIndex: Clone {
    fn next(&self) -> Self;
}

impl RangeIndex for usize {
    fn next(&self) -> Self {
        self + 1
    }
}

pub trait IntoRange<Idx> {
    fn into_range(self, unbounded: Range<Idx>) -> Range<Idx>;
}

impl<Idx: RangeIndex, T: RangeBounds<Idx>> IntoRange<Idx> for T {
    fn into_range(self, unbounded: Range<Idx>) -> Range<Idx> {
        let start = match self.start_bound() {
            Bound::Included(start) => start.clone(),
            Bound::Excluded(start) => start.next(),
            Bound::Unbounded => unbounded.start,
        };

        let end = match self.end_bound() {
            Bound::Included(end) => end.next(),
            Bound::Excluded(end) => end.clone(),
            Bound::Unbounded => unbounded.end,
        };

        start..end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn range_eq<L, R>(left: L, right: R) -> bool
    where
        L: IntoRange<usize>,
        R: IntoRange<usize>,
    {
        left.into_range(0..10) == right.into_range(0..10)
    }

    #[test]
    fn into_range() {
        assert!(range_eq(3..7, 3..7));
        assert!(range_eq(3..=7, 3..8));
        assert!(range_eq(3.., 3..10));
        assert!(range_eq(..7, 0..7));
        assert!(range_eq(.., 0..10));
    }
}