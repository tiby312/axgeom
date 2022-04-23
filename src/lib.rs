//! A library that provides a way to easily extract 1d ranges from a 2d container based off of the x or y axis statically through
//! type parameters. This can help with performance in algorithms where you need to get values for a particular axis often.
//!

#![no_std]

mod range;
mod ray;
mod rect;
mod vec2;

#[cfg(feature = "std")]
pub use roots;

pub use num_traits;
pub use partial_min_max;

pub use self::range::range;
pub use self::range::Range;
pub use self::ray::ray;
pub use self::ray::CastResult;
pub use self::ray::Ray;
pub use self::rect::rect;
pub use self::rect::Rect;
pub use self::vec2::arr2_as;
pub use self::vec2::vec2;
pub use self::vec2::vec2same;
pub use self::vec2::Vec2;

///The x axis implementation of the Axis
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct XAXIS;
impl Axis for XAXIS {
    type Next = YAXIS;
    #[inline(always)]
    #[must_use]
    fn is_xaxis(&self) -> bool {
        true
    }
    #[inline(always)]
    #[must_use]
    fn next(&self) -> Self::Next {
        YAXIS
    }
}

///The y axis implementation of the Axis
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct YAXIS;
impl Axis for YAXIS {
    type Next = XAXIS;

    #[inline(always)]
    #[must_use]
    fn is_xaxis(&self) -> bool {
        false
    }

    #[inline(always)]
    #[must_use]
    fn next(&self) -> Self::Next {
        XAXIS
    }
}

///A dynamic axis as opposed to a statically known one via `impl Axis`.
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
#[must_use]
pub enum AxisDyn {
    X,
    Y,
}
impl AxisDyn {
    #[inline(always)]
    #[must_use]
    pub const fn is_equal_to(&self, other: AxisDyn) -> bool {
        use AxisDyn::*;
        matches!((self, other), (X, X) | (Y, Y))
    }

    #[inline(always)]
    #[must_use]
    pub const fn is_xaxis(self) -> bool {
        match self {
            AxisDyn::X => true,
            AxisDyn::Y => false,
        }
    }

    #[inline(always)]
    pub const fn next(&self) -> Self {
        use AxisDyn::*;
        match self {
            X => Y,
            Y => X,
        }
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

    ///Convert a statically known axis into a dynamic one.
    #[inline(always)]
    fn to_dyn(&self) -> AxisDyn {
        if self.is_xaxis() {
            AxisDyn::X
        } else {
            AxisDyn::Y
        }
    }

    #[inline(always)]
    #[must_use]
    fn is_equal_to<B: Axis>(&self, other: B) -> bool {
        self.to_dyn().is_equal_to(other.to_dyn())
    }
}
