//! Slices of bus signals.

use std::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

use crate::{Concat, SignalId};
use serde::{Deserialize, Serialize};

/// A single bit wire or a portion of a bus signal.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct Slice {
    signal: SignalId,
    range: Option<SliceRange>,
}

/// A single wire.
///
/// May be a single bit wire or a single bit taken from a bus.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
pub struct SliceOne {
    signal: SignalId,
    index: Option<usize>,
}

/// A range of bus indices.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct SliceRange {
    pub(crate) start: usize,
    pub(crate) end: usize,
}

impl SliceRange {
    #[inline]
    pub(crate) fn new(start: usize, end: usize) -> Self {
        assert!(end > start);
        Self { start, end }
    }

    pub(crate) fn from_index(start: usize) -> Self {
        Self::new(start, start + 1)
    }

    #[inline]
    pub(crate) fn with_width(end: usize) -> Self {
        assert!(end > 0);
        Self { start: 0, end }
    }

    /// The width of this slice.
    #[inline]
    pub const fn width(&self) -> usize {
        self.end - self.start
    }

    /// The start index (inclusive) of this range.
    #[inline]
    pub const fn start(&self) -> usize {
        self.start
    }

    /// The end index (**exclusive**) of this range.
    #[inline]
    pub const fn end(&self) -> usize {
        self.end
    }

    /// Iterate over the indices in this slice.
    #[inline]
    pub fn indices(&self) -> impl Iterator<Item = usize> {
        self.start..self.end
    }

    /// Returns if the this slice contains the given index.
    #[inline]
    pub const fn contains(&self, idx: usize) -> bool {
        idx >= self.start && idx < self.end
    }
}
impl IntoIterator for SliceRange {
    type Item = usize;
    type IntoIter = std::ops::Range<usize>;
    fn into_iter(self) -> Self::IntoIter {
        self.start..self.end
    }
}
impl Slice {
    #[inline]
    pub(crate) fn new(signal: SignalId, range: Option<SliceRange>) -> Self {
        Self { signal, range }
    }

    /// The range of indices indexed by this slice.
    ///
    /// Returns [`None`] if this slice represents a single bit wire.
    #[inline]
    pub fn range(&self) -> Option<SliceRange> {
        self.range
    }

    /// The width of this slice.
    ///
    /// Returns 1 if this slice represents a single bit wire.
    #[inline]
    pub fn width(&self) -> usize {
        self.range.map(|x| x.width()).unwrap_or(1)
    }

    /// The ID of the signal this slice indexes.
    #[inline]
    pub fn signal(&self) -> SignalId {
        self.signal
    }

    /// Returns `true` if this signal indexes into a bus.
    #[inline]
    pub fn is_bus(&self) -> bool {
        self.range.is_some()
    }

    #[inline]
    fn assert_bus_index(&self) {
        assert!(
            self.is_bus(),
            "attempted to index into a single-bit wire; only buses support indexing"
        );
    }

    /// If this slice contains one bit, returns `Some` with a [`SliceOne`].
    ///
    /// Otherwise, returns [`None`].
    pub fn slice_one(&self) -> Option<SliceOne> {
        SliceOne::try_from(self).ok()
    }
}

impl IndexOwned<usize> for Slice {
    type Output = SliceOne;
    fn index(&self, index: usize) -> Self::Output {
        self.assert_bus_index();
        SliceOne::new(self.signal, Some(self.range.unwrap().index(index)))
    }
}

impl IndexOwned<Range<usize>> for Slice {
    type Output = Self;
    fn index(&self, index: Range<usize>) -> Self::Output {
        self.assert_bus_index();
        Self::new(self.signal, Some(self.range.unwrap().index(index)))
    }
}

impl IndexOwned<RangeFrom<usize>> for Slice {
    type Output = Self;
    fn index(&self, index: RangeFrom<usize>) -> Self::Output {
        self.assert_bus_index();
        Self::new(self.signal, Some(self.range.unwrap().index(index)))
    }
}

impl IndexOwned<RangeFull> for Slice {
    type Output = Self;
    fn index(&self, index: RangeFull) -> Self::Output {
        self.assert_bus_index();
        Self::new(self.signal, Some(self.range.unwrap().index(index)))
    }
}

impl IndexOwned<RangeInclusive<usize>> for Slice {
    type Output = Self;
    fn index(&self, index: RangeInclusive<usize>) -> Self::Output {
        self.assert_bus_index();
        Self::new(self.signal, Some(self.range.unwrap().index(index)))
    }
}

impl IndexOwned<RangeTo<usize>> for Slice {
    type Output = Self;
    fn index(&self, index: RangeTo<usize>) -> Self::Output {
        self.assert_bus_index();
        Self::new(self.signal, Some(self.range.unwrap().index(index)))
    }
}

impl IndexOwned<RangeToInclusive<usize>> for Slice {
    type Output = Self;
    fn index(&self, index: RangeToInclusive<usize>) -> Self::Output {
        self.assert_bus_index();
        Self::new(self.signal, Some(self.range.unwrap().index(index)))
    }
}

impl IndexOwned<usize> for SliceRange {
    type Output = usize;
    fn index(&self, index: usize) -> Self::Output {
        let idx = self.start + index;
        assert!(idx < self.end, "index out of bounds");
        idx
    }
}

impl IndexOwned<Range<usize>> for SliceRange {
    type Output = Self;
    fn index(&self, index: Range<usize>) -> Self::Output {
        assert!(self.start + index.end <= self.end, "index out of bounds");
        Self::new(self.start + index.start, self.start + index.end)
    }
}

impl IndexOwned<RangeFrom<usize>> for SliceRange {
    type Output = Self;
    fn index(&self, index: RangeFrom<usize>) -> Self::Output {
        assert!(self.start + index.start <= self.end, "index out of bounds");
        Self::new(self.start + index.start, self.end)
    }
}

impl IndexOwned<RangeFull> for SliceRange {
    type Output = Self;
    fn index(&self, _index: RangeFull) -> Self::Output {
        *self
    }
}

impl IndexOwned<RangeInclusive<usize>> for SliceRange {
    type Output = Self;
    fn index(&self, index: RangeInclusive<usize>) -> Self::Output {
        assert!(self.start + index.end() < self.end, "index out of bounds");
        Self::new(self.start + index.start(), self.start + index.end() + 1)
    }
}

impl IndexOwned<RangeTo<usize>> for SliceRange {
    type Output = Self;
    fn index(&self, index: RangeTo<usize>) -> Self::Output {
        assert!(self.start + index.end <= self.end, "index out of bounds");
        Self::new(self.start, self.start + index.end)
    }
}

impl IndexOwned<RangeToInclusive<usize>> for SliceRange {
    type Output = Self;
    fn index(&self, index: RangeToInclusive<usize>) -> Self::Output {
        assert!(self.start + index.end < self.end, "index out of bounds");
        Self::new(self.start, self.start + index.end + 1)
    }
}

impl IndexOwned<usize> for Concat {
    type Output = SliceOne;

    fn index(&self, mut index: usize) -> Self::Output {
        for part in self.parts.iter() {
            let width = part.width();
            if index < width {
                return part.slice_one().unwrap_or_else(|| part.index(index));
            }
            index -= width;
        }
        panic!("index {index} out of bounds for signal");
    }
}

/// Index into an object.
///
///
/// Unlike [`std::ops::Index`], allows implementors
/// to return ownership of data, rather than just a reference.
pub trait IndexOwned<Idx>
where
    Idx: ?Sized,
{
    /// The result of the indexing operation.
    type Output;

    /// Indexes the given object, returning owned data.
    fn index(&self, index: Idx) -> Self::Output;
}

/// The error type returned when converting
/// [`Slice`]s to [`SliceOne`]s.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct SliceWidthNotOne;

impl TryFrom<Slice> for SliceOne {
    type Error = SliceWidthNotOne;
    fn try_from(value: Slice) -> Result<Self, Self::Error> {
        if value.width() == 1 {
            Ok(Self {
                signal: value.signal,
                index: value.range.map(|s| s.start()),
            })
        } else {
            Err(SliceWidthNotOne)
        }
    }
}

impl TryFrom<&Slice> for SliceOne {
    type Error = SliceWidthNotOne;
    fn try_from(value: &Slice) -> Result<Self, Self::Error> {
        Self::try_from(*value)
    }
}

impl From<SliceOne> for Slice {
    fn from(value: SliceOne) -> Self {
        Self {
            signal: value.signal,
            range: value.range(),
        }
    }
}

impl From<&SliceOne> for Slice {
    fn from(value: &SliceOne) -> Self {
        Self::from(*value)
    }
}

impl SliceOne {
    #[inline]
    pub(crate) fn new(signal: SignalId, index: Option<usize>) -> Self {
        Self { signal, index }
    }

    /// The range of indices indexed by this slice.
    ///
    /// Returns [`None`] if this slice represents a single bit wire.
    #[inline]
    pub fn range(&self) -> Option<SliceRange> {
        self.index.map(SliceRange::from_index)
    }

    /// The width of this slice.
    #[inline]
    pub const fn width(&self) -> usize {
        1
    }

    /// The ID of the signal this slice indexes.
    #[inline]
    pub fn signal(&self) -> SignalId {
        self.signal
    }

    /// Returns `true` if this signal indexes into a bus.
    #[inline]
    pub fn is_bus(&self) -> bool {
        self.index.is_some()
    }

    /// The index this single-bit slice contains.
    #[inline]
    pub fn index(&self) -> Option<usize> {
        self.index
    }
}
