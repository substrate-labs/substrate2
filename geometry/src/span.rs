/// A one-dimensional span.
use serde::{Deserialize, Serialize};

use crate::sign::Sign;
use crate::snap::snap_to_grid;

#[derive(
    Debug, Default, Clone, Copy, Hash, Ord, PartialOrd, Serialize, Deserialize, PartialEq, Eq,
)]
pub struct Span {
    start: i64,
    stop: i64,
}

impl Span {
    /// Creates a new [`Span`] from 0 until the specified stop.
    ///
    /// # Panics
    ///
    /// This function panics if `stop` is less than 0.
    pub fn until(stop: i64) -> Self {
        debug_assert!(stop >= 0);
        Self { start: 0, stop }
    }

    /// Creates a new [`Span`] between two integers.
    ///
    /// The caller must ensure that `start` is less
    /// than or equal to `stop`.
    pub const fn new_unchecked(start: i64, stop: i64) -> Self {
        Self { start, stop }
    }

    /// Creates a new [`Span`] between two integers.
    pub fn new(start: i64, stop: i64) -> Self {
        use std::cmp::{max, min};
        let lower = min(start, stop);
        let upper = max(start, stop);
        Self {
            start: lower,
            stop: upper,
        }
    }

    /// Creates a span of zero length encompassing the given point.
    pub fn from_point(x: i64) -> Self {
        Self { start: x, stop: x }
    }

    pub fn with_start_and_length(start: i64, length: i64) -> Self {
        Self {
            stop: start + length,
            start,
        }
    }

    pub fn with_stop_and_length(stop: i64, length: i64) -> Self {
        Self {
            start: stop - length,
            stop,
        }
    }

    /// Creates a span with the given endpoint and length.
    ///
    /// If `sign` is [`Sign::Pos`], `point` is treated as the ending/stopping point of the span.
    /// If `sign` is [`Sign::Neg`], `point` is treated as the beginning/starting point of the span.
    pub fn with_point_and_length(sign: Sign, point: i64, length: i64) -> Self {
        match sign {
            Sign::Pos => Self::with_stop_and_length(point, length),
            Sign::Neg => Self::with_start_and_length(point, length),
        }
    }

    /// Creates a new [`Span`] expanded by `amount` in the direction indicated by `pos`.
    pub fn expand(mut self, pos: bool, amount: i64) -> Self {
        if pos {
            self.stop += amount;
        } else {
            self.start -= amount;
        }
        self
    }

    /// Creates a new [`Span`] expanded by `amount` in both directions.
    pub fn expand_all(mut self, amount: i64) -> Self {
        self.stop += amount;
        self.start -= amount;
        self
    }

    /// Gets the starting ([`Sign::Neg`]) or stopping ([`Sign::Pos`]) point of a span.
    #[inline]
    pub fn point(&self, sign: Sign) -> i64 {
        match sign {
            Sign::Neg => self.start(),
            Sign::Pos => self.stop(),
        }
    }

    /// Gets the shortest distance to a point.
    pub fn distance_to(&self, point: i64) -> i64 {
        std::cmp::min((point - self.start()).abs(), (point - self.stop()).abs())
    }

    /// Creates a new [`Span`] with center `center` and length `span`.
    pub fn from_center_span(center: i64, span: i64) -> Self {
        assert!(span >= 0);
        assert_eq!(span % 2, 0);

        Self::new(center - (span / 2), center + (span / 2))
    }

    /// Creates a new [`Span`] with center `center` and length `span` and snap the edges to the
    /// grid.
    pub fn from_center_span_gridded(center: i64, span: i64, grid: i64) -> Self {
        assert!(span >= 0);
        assert_eq!(span % 2, 0);
        assert_eq!(span % grid, 0);

        let start = snap_to_grid(center - (span / 2), grid);

        Self::new(start, start + span)
    }

    /// Gets the center of the span.
    #[inline]
    pub fn center(&self) -> i64 {
        (self.start + self.stop) / 2
    }

    /// Gets the length of the span.
    #[inline]
    pub fn length(&self) -> i64 {
        self.stop - self.start
    }

    /// Gets the start of the span.
    #[inline]
    pub fn start(&self) -> i64 {
        self.start
    }

    /// Gets the stop of the span.
    #[inline]
    pub fn stop(&self) -> i64 {
        self.stop
    }

    /// Checks if the span intersects with the [`Span`] `other`.
    #[inline]
    pub fn intersects(&self, other: &Self) -> bool {
        !(other.stop < self.start || self.stop < other.start)
    }

    /// Creates a new minimal [`Span`] that contains all of the elements of `spans`.
    pub fn merge(spans: impl IntoIterator<Item = Self>) -> Self {
        use std::cmp::{max, min};
        let mut spans = spans.into_iter();
        let (mut start, mut stop) = spans
            .next()
            .expect("Span::merge requires at least one span")
            .into();

        for span in spans {
            start = min(start, span.start);
            stop = max(stop, span.stop);
        }

        debug_assert!(start <= stop);

        Span { start, stop }
    }

    /// Merges adjacent spans when `merge_fn` evaluates to true.
    pub fn merge_adjacent(
        spans: impl IntoIterator<Item = Self>,
        mut merge_fn: impl FnMut(Span, Span) -> bool,
    ) -> impl Iterator<Item = Span> {
        let mut spans: Vec<Span> = spans.into_iter().collect();
        spans.sort_by_key(|span| span.start());

        let mut merged_spans = Vec::new();

        let mut j = 0;
        while j < spans.len() {
            let mut curr_span = spans[j];
            j += 1;
            while j < spans.len() && merge_fn(curr_span, spans[j]) {
                curr_span = curr_span.union(spans[j]);
                j += 1;
            }
            merged_spans.push(curr_span);
        }

        merged_spans.into_iter()
    }

    pub fn union(self, other: Self) -> Self {
        use std::cmp::{max, min};
        Self {
            start: min(self.start, other.start),
            stop: max(self.stop, other.stop),
        }
    }

    pub fn contains(self, other: Self) -> bool {
        self.union(other) == self
    }

    /// Returns a new [`Span`] representing the union of the current span with the given point.
    pub fn add_point(self, pos: i64) -> Self {
        use std::cmp::{max, min};
        Self {
            start: min(self.start, pos),
            stop: max(self.stop, pos),
        }
    }

    /// Shrinks the given side by the given amount.
    ///
    /// Behavior is controlled by the given [`Sign`]:
    /// * If `side` is [`Sign::Pos`], shrinks from the positive end (ie. decreases the `stop`).
    /// * If `side` is [`Sign::Neg`], shrinks from the negative end (ie. increases the `start`).
    pub fn shrink(self, side: Sign, amount: i64) -> Self {
        assert!(self.length() >= amount);
        match side {
            Sign::Pos => Self::new(self.start, self.stop - amount),
            Sign::Neg => Self::new(self.start + amount, self.stop),
        }
    }

    pub fn shrink_all(self, amount: i64) -> Self {
        assert!(self.length() >= 2 * amount);
        Self {
            start: self.start + amount,
            stop: self.stop - amount,
        }
    }

    pub fn translate(self, amount: i64) -> Self {
        Self {
            start: self.start + amount,
            stop: self.stop + amount,
        }
    }

    pub fn min_distance(self, other: Span) -> i64 {
        std::cmp::max(
            0,
            self.union(other).length() - self.length() - other.length(),
        )
    }
}

impl From<(i64, i64)> for Span {
    #[inline]
    fn from(tup: (i64, i64)) -> Self {
        Self::new(tup.0, tup.1)
    }
}

impl From<Span> for (i64, i64) {
    #[inline]
    fn from(s: Span) -> Self {
        (s.start(), s.stop())
    }
}
