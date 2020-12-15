//!	A library that provides a way to easily extract 1d ranges from a 2d container based off of the x or y axis statically through
//! type parameters. This can help with performnace in algorithms where you need to get values for a particular axis often.
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
#[derive(Copy, Clone)]
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
#[derive(Copy, Clone)]
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

///Axis trait can be used to extract the x or y portions of a container.
///when you know the axis as compile time.
///The X implementation of this trait's Next associated trait is the Y implementation.
///The Y implementation of this trait's Next associated trait is the X implementation.
pub trait Axis: Sync + Send + Copy + Clone {
    type Next: Axis;
    fn is_xaxis(&self) -> bool;
    fn next(&self) -> Self::Next;

    #[inline(always)]
    #[must_use]
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

///Represents a Vec2 with the specified aspect ratio
///and the specified width. The height of the Vec2
///can be inferred by the aspect ratio.
#[derive(Copy, Clone, Debug)]
pub struct FixedAspectVec2 {
    pub ratio: AspectRatio,
    pub width: f64,
}
impl FixedAspectVec2 {
    #[inline(always)]
    #[must_use]
    pub fn as_vec(&self) -> Vec2<f64> {
        let height = self.ratio.height_over_width() * self.width;
        vec2(self.width, height)
    }
}

///An aspect ratio represented as a fraction
///so that there is no precision loss.
#[derive(Copy, Clone, Debug)]
pub struct AspectRatio(pub Vec2<f64>);

impl AspectRatio {
    #[inline(always)]
    #[must_use]
    pub fn width_over_height(&self) -> f64 {
        let v = self.0;
        v.x as f64 / v.y as f64
    }

    #[inline(always)]
    #[must_use]
    pub fn height_over_width(&self) -> f64 {
        let v = self.0;
        v.y as f64 / v.x as f64
    }
}
