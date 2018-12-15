//!	A library that provides a way to easily extract 1d ranges from a 2d container based off of the x or y axis statically through
//! type parameters. This is useful if you have a function that operates on an axis that recursively calls itself but at the same time alternates its axis. Also provides useful functions that operate on types that implement Ord such as grow_to_fit().


mod range;
mod rect;

pub use self::range::Range;
pub use self::rect::Rect;

///The x axis implementation of the AxisTrait
#[derive(Copy,Clone)]
pub struct XAXISS;
impl AxisTrait for XAXISS{
    type Next=YAXISS;
    #[inline(always)]
    fn is_xaxis(&self)->bool{
        true
    }
    #[inline(always)]
    fn next(&self)->Self::Next{
        YAXISS
    }
}

///The y axis implementation of the AxisTrait
#[derive(Copy,Clone)]
pub struct YAXISS;
impl AxisTrait for YAXISS{
    type Next=XAXISS;

    #[inline(always)]
    fn is_xaxis(&self)->bool{
        false
    }
    #[inline(always)]
    fn next(&self)->Self::Next{
        XAXISS
    }
}

///Axis trait can be used to extract the x or y portions of a container.
///when you know the axis as compile time.
///The X implementation of this trait's Next associated trait is the Y implementation.
///The Y implementation of this trait's Next associated trait is the X implementation. 
pub trait AxisTrait:Sync+Send+Copy+Clone{
    type Next:AxisTrait;
    fn is_xaxis(&self)->bool;
    fn next(&self)->Self::Next;

    #[inline(always)]
    fn is_equal_to<B:AxisTrait>(&self,other:B)->bool{
        if self.is_xaxis() && other.is_xaxis(){
            return true;
        }
        if !self.is_xaxis() && !other.is_xaxis(){
            return true;
        }
        false
    }
}


///A wrapper around an array that lets you extract the x and y components using the AxisTrait.
pub struct AxisWrapRef<'a,T:'a>(pub &'a [T;2]);
impl<'a,T:'a> AxisWrapRef<'a,T>{
    #[inline(always)]
    pub fn get<A:AxisTrait>(&self,axis:A)->&'a T{
        if axis.is_xaxis(){
            &self.0[0]
        }else{
            &self.0[1]
        }
    }
}


