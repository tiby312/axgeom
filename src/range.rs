use core::convert::TryFrom;
use num_traits::Float;
use ordered_float::NotNan;
use primitive_from::PrimitiveFrom;

//TODO use this:
//https://doc.rust-lang.org/std/ops/struct.Range.html

///A 1D range. Internally represented as start and end. (as opposed to a start and length)
///This means that subdivision does not result in any floating point calculations.
///The left value must be <= the right value.
///There is no protection against "degenerate" Ranges where left>right.
///Behavior of any of the functions with degenrate Ranges is unspecified.
///
///
///A point is consindered inside of a range if the point is in [left,right), a semi-open interval.
///
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[must_use]
pub struct Range<T> {
    pub left: T,
    pub right: T,
}

impl<T> Range<T> {
    #[inline(always)]
    pub fn new(left: T, right: T) -> Range<T> {
        Range { left, right }
    }
}
impl<T: Copy + PartialOrd> Range<T> {
    ///If the pos is to the left of the range, return less.
    ///If the pos is to the right of the range, return greater.
    ///else, return equal.
    #[inline(always)]
    pub fn left_or_right_or_contain(&self, pos: &T) -> core::cmp::Ordering {
        if *pos < self.left {
            core::cmp::Ordering::Less
        } else if *pos >= self.right {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    ///Returns true if the point is inside of the range or on top of.
    #[inline(always)]
    pub fn contains(&self, pos: T) -> bool {
        self.left <= pos && pos < self.right
    }

    ///Subdivides the range.
    ///No floating point calculations are done.
    #[inline(always)]
    pub fn subdivide(&self, divider: T) -> (Range<T>, Range<T>) {
        debug_assert!(self.left <= divider);
        debug_assert!(divider < self.right);

        let l = Range {
            left: self.left,
            right: divider,
        };
        let r = Range {
            left: divider,
            right: self.right,
        };
        (l, r)
    }

    #[must_use]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.left <= self.right
    }

    #[inline(always)]
    pub fn grow_to_fit(&mut self, b: &Range<T>) {
        let a = self;
        if b.left < a.left {
            a.left = b.left;
        }
        if b.right > a.right {
            a.right = b.right;
        }
    }

    ///Returns true if self contains the specified range.
    #[inline(always)]
    pub fn contains_range(&self, val: &Range<T>) -> bool {
        self.left <= val.left && val.right <= self.right
    }

    ///Returns true if two ranges intersect.
    #[inline(always)]
    pub fn intersects(&self, val: &Range<T>) -> bool {
        self.contains(val.left) || val.contains(self.left)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_range() {
        let a = Range::new(0, 5);
        let b = Range::new(0, 5);
        assert!(a.contains_range(&b))
    }
}
impl<T: Copy + core::ops::Sub<Output = T>> Range<T> {
    #[inline(always)]
    pub fn distance(&self) -> T {
        self.right - self.left
    }
}

impl<T: Copy + core::ops::Sub<Output = T> + core::ops::Add<Output = T>> Range<T> {
    #[inline(always)]
    pub fn grow(&mut self, radius: T) -> &mut Self {
        self.right = self.right + radius;
        self.left = self.left - radius;
        self
    }
}

impl<N: Float> AsRef<Range<N>> for Range<NotNan<N>> {
    #[inline(always)]
    fn as_ref(&self) -> &Range<N> {
        unsafe { &*((self as *const Self) as *const Range<N>) }
    }
}

impl<N: Float> AsMut<Range<N>> for Range<NotNan<N>> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Range<N> {
        unsafe { &mut *((self as *mut Self) as *mut Range<N>) }
    }
}

impl<S: Copy> Range<S> {
    #[inline(always)]
    pub fn inner_as<B: PrimitiveFrom<S>>(&self) -> Range<B> {
        Range {
            left: PrimitiveFrom::from(self.left),
            right: PrimitiveFrom::from(self.right),
        }
    }

    #[inline(always)]
    pub fn inner_into<A: From<S>>(&self) -> Range<A> {
        let left = A::from(self.left);
        let right = A::from(self.right);
        Range { left, right }
    }

    #[inline(always)]
    pub fn inner_try_into<A: TryFrom<S>>(&self) -> Result<Range<A>, A::Error> {
        let left = A::try_from(self.left);
        let right = A::try_from(self.right);
        match (left, right) {
            (Ok(left), Ok(right)) => Ok(Range { left, right }),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(e1), Err(_)) => Err(e1),
        }
    }
}

impl<T: Copy + core::ops::Sub<Output = T> + core::ops::Add<Output = T>> Range<T> {
    ///Create a range from a point and radius.
    #[inline(always)]
    pub fn from_point(point: T, radius: T) -> Range<T> {
        Range {
            left: point - radius,
            right: point + radius,
        }
    }
}

impl<T: Copy + Ord> Range<T> {
    ///Creates a range that represents the intersection range.
    #[inline(always)]
    pub fn get_intersection(&self, val: &Range<T>) -> Option<Range<T>> {
        let a = self.left.max(val.left);
        let b = self.right.min(val.right);
        if a > b {
            None
        } else {
            Some(Range { left: a, right: b })
        }
    }
}
