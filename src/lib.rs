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
#[derive(Eq,PartialEq,Copy, Clone,Debug)]
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
#[derive(Eq,PartialEq,Copy, Clone,Debug)]
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


#[derive(Eq,PartialEq,Copy,Clone,Debug)]
pub enum AxisDyn{
    X(XAXIS),
    Y(YAXIS)
}
impl AxisDyn{

    #[inline(always)]
    pub fn is_axis(self)->bool{
        match self{
            AxisDyn::X(_)=>{
                true
            },
            AxisDyn::Y(_)=>{
                false
            }
        }
    }
    #[inline(always)]
    pub fn map_val<T>(self,val1:T,val2:T)->T{
        match self{
            AxisDyn::X(_)=>{
                val1
            },
            AxisDyn::Y(_)=>{
                val2
            }
        }
    }
    
    #[inline(always)]
    pub fn map<T>(self,func1:impl FnOnce(XAXIS)->T,func2:impl FnOnce(YAXIS)->T)->T{
        match self{
            AxisDyn::X(a)=>{
                func1(a)
            },
            AxisDyn::Y(b)=>{
                func2(b)
            }
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

    #[inline(always)]
    fn to_dyn(&self)->AxisDyn{
        if self.is_xaxis(){
            AxisDyn::X(XAXIS)
        }else{
            AxisDyn::Y(YAXIS)
        }
    }
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
