use core::convert::TryFrom;
use num_traits::Float;
use ordered_float::NotNan;
use primitive_from::PrimitiveFrom;

//Convenience function to create a Range.
#[inline(always)]
pub fn range<T>(start: T, end: T) -> Range<T> {
    Range { start, end }
}

///A 1D range. Internally represented as start and end. (as opposed to a start and length)
///This means that subdivision does not result in any floating point calculations.
///The start value must be <= the end value.
///There is no protection against "degenerate" Ranges where start>end.
///Behavior of any of the functions with degenrate Ranges is unspecified.
///
///
///A point is consindered inside of a range if the point is in [start,end), a semi-open interval.
///
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[must_use]
pub struct Range<T> {
    pub start: T,
    pub end: T,
}

impl<T> Range<T> {
    #[inline(always)]
    pub fn new(start: T, end: T) -> Range<T> {
        Range { start, end }
    }

    #[inline(always)]
    pub fn as_arr(&self)->&[T;2]{
        unsafe{&*(self as *const _ as *const _)}
    }


    #[inline(always)]
    pub fn from_ref(arr:&[T;2])->&Self{
        unsafe{&*(arr as *const _ as *const _)}
    }

    #[inline(always)]
    pub fn from_ref_mut(arr:&mut [T;2])->&mut Self{
        unsafe{&mut *(arr as *mut _ as *mut _)}
    }

}

impl<T: Copy + Ord + core::ops::Sub<Output = T> + num_traits::sign::Signed> Range<T> {
    #[inline(always)]
    pub fn distance_to_point(&self, pos: T) -> Option<T> {
        if self.contains(pos) {
            None
        } else {
            Some((pos - self.start).abs().min((pos - self.end).abs()))
        }
    }

    ///Positive if point is to the right of the range.
    ///Negative if point is to the left of range.
    #[inline(always)]
    pub fn difference_to_point(&self, pos: T) -> Option<T> {
        match self.contains_ext(pos) {
            core::cmp::Ordering::Less => Some(pos - self.start),
            core::cmp::Ordering::Greater => Some(pos - self.end),
            core::cmp::Ordering::Equal => None,
        }
    }
}
impl<T: Copy + PartialOrd> Range<T> {
    ///Like contains() but returns Ord.
    ///If the pos is stricly less than the range.start, return less.
    ///If the pos is greater of equal to the range.end, return greater.
    ///else, return equal.
    #[inline(always)]
    pub fn contains_ext(&self, pos: T) -> core::cmp::Ordering {
        if pos < self.start {
            core::cmp::Ordering::Less
        } else if pos >= self.end {
            core::cmp::Ordering::Greater
        } else {
            core::cmp::Ordering::Equal
        }
    }

    ///Returns true if the point is inside of the range or on top of.
    #[inline(always)]
    pub fn contains(&self, pos: T) -> bool {
        self.contains_ext(pos) == core::cmp::Ordering::Equal
        //self.start <= pos && pos < self.end
    }

    ///Subdivides the range.
    ///No floating point calculations are done.
    #[inline(always)]
    pub fn subdivide(&self, divider: T) -> (Range<T>, Range<T>) {
        debug_assert!(self.start <= divider);
        debug_assert!(divider < self.end);

        let l = Range {
            start: self.start,
            end: divider,
        };
        let r = Range {
            start: divider,
            end: self.end,
        };
        (l, r)
    }

    #[must_use]
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.start <= self.end
    }

    #[inline(always)]
    pub fn grow_to_fit(&mut self, b: &Range<T>) {
        let a = self;
        if b.start < a.start {
            a.start = b.start;
        }
        if b.end > a.end {
            a.end = b.end;
        }
    }

    ///Returns true if self contains the specified range.
    #[inline(always)]
    pub fn contains_range(&self, val: &Range<T>) -> bool {
        self.start <= val.start && val.end <= self.end
    }

    ///Returns true if two ranges intersect.
    #[inline(always)]
    pub fn intersects(&self, val: &Range<T>) -> bool {
        self.contains(val.start) || val.contains(self.start)
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
        self.end - self.start
    }
}

impl<T: Copy + core::ops::Sub<Output = T> + core::ops::Add<Output = T>> Range<T> {
    #[inline(always)]
    pub fn grow(&mut self, radius: T) -> &mut Self {
        self.end = self.end + radius;
        self.start = self.start - radius;
        self
    }
}

impl<N: Float> AsRef<Range<N>> for Range<NotNan<N>> {
    #[inline(always)]
    fn as_ref(&self) -> &Range<N> {
        unsafe { &*((self as *const Self) as *const Range<N>) }
    }
}

impl<S: Copy> Range<S> {
    #[inline(always)]
    pub fn inner_as<B: PrimitiveFrom<S>>(&self) -> Range<B> {
        Range {
            start: PrimitiveFrom::from(self.start),
            end: PrimitiveFrom::from(self.end),
        }
    }

    #[inline(always)]
    pub fn inner_into<A: From<S>>(&self) -> Range<A> {
        let start = A::from(self.start);
        let end = A::from(self.end);
        Range { start, end }
    }

    #[inline(always)]
    pub fn inner_try_into<A: TryFrom<S>>(&self) -> Result<Range<A>, A::Error> {
        let start = A::try_from(self.start);
        let end = A::try_from(self.end);
        match (start, end) {
            (Ok(start), Ok(end)) => Ok(Range { start, end }),
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
            start: point - radius,
            end: point + radius,
        }
    }
}

impl<T: Copy + Ord> Range<T> {
    ///Creates a range that represents the intersection range.
    #[inline(always)]
    pub fn get_intersection(&self, val: &Range<T>) -> Option<Range<T>> {
        let a = self.start.max(val.start);
        let b = self.end.min(val.end);
        if a > b {
            None
        } else {
            Some(Range { start: a, end: b })
        }
    }
}
