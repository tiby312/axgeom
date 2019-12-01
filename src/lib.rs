//!	A library that provides a way to easily extract 1d ranges from a 2d container based off of the x or y axis statically through
//! type parameters. This can help with performnace in algorithms where you need to get values for a particular axis often.

#![no_std]

mod range;
mod rect;
mod vec2;
mod ray;

pub use self::ray::Ray;
pub use self::range::Range;
pub use self::rect::Rect;
pub use self::rect::rect;
pub use self::vec2::arr2_as;
pub use self::vec2::vec2;
pub use self::vec2::vec2same;
pub use self::vec2::Vec2;

///The x axis implementation of the Axis
#[derive(Copy, Clone)]
pub struct XAXISS;
impl Axis for XAXISS {
    type Next = YAXISS;
    #[inline(always)]
    fn is_xaxis(&self) -> bool {
        true
    }
    #[inline(always)]
    fn next(&self) -> Self::Next {
        YAXISS
    }
}

///The y axis implementation of the Axis
#[derive(Copy, Clone)]
pub struct YAXISS;
impl Axis for YAXISS {
    type Next = XAXISS;

    #[inline(always)]
    fn is_xaxis(&self) -> bool {
        false
    }
    #[inline(always)]
    fn next(&self) -> Self::Next {
        XAXISS
    }
}

///Axis trait can be used to extract the x or y portions of a container.
///when you know the axis as compile time.
///The X implementation of this trait's Next associated trait is the Y implementation.
///The Y implementation of this trait's Next associated trait is the X implementation.
pub trait Axis: Sync + Send + Copy + Clone {
    type Next: Axis;
    fn is_xaxis(&self) -> bool;
    fn next(&self) -> Self::Next;

    #[inline(always)]
    fn is_equal_to<B: Axis>(&self, other: B) -> bool {
        if self.is_xaxis() && other.is_xaxis() {
            return true;
        }
        if !self.is_xaxis() && !other.is_xaxis() {
            return true;
        }
        false
    }
}
