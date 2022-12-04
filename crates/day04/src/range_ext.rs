use std::ops::RangeInclusive;

pub trait Overlaps<Rhs=Self> {
    fn overlaps(&self, rhs: &Rhs) -> bool;
}

impl Overlaps for RangeInclusive<usize> {
    fn overlaps(&self, rhs: &RangeInclusive<usize>) -> bool {
        self.contains(rhs.start()) || self.contains(rhs.end())
    }
}

pub trait ContainsRange<Rhs=Self> {
    fn contains_range(&self, rhs: &Rhs) -> bool;
}

impl ContainsRange for RangeInclusive<usize> {
    fn contains_range(&self, rhs: &RangeInclusive<usize>) -> bool {
        self.contains(rhs.start()) && self.contains(rhs.end())
    }
}