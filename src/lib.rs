//!	A 2D geometry library. It provides a way to easily extract 1d ranges from a 2d Rectangle based off of the x or y axis.
//!	Also provides functions that operate on types that implement Ord, as well has hard coded ones for f32.
//!
//!## Example
//!
//!```
//!extern crate axgeom;
//!use axgeom::{Rect,Range,Axis,AxisIter,XAXIS,YAXIS};
//!fn main(){
//!let rect = Rect::new(30.0,40.0,30.0,40.0);
//!
//!for k in AxisIter::new(){
//!	let r=rect.get_range(k);
//!	assert!(r.len()==10.0);
//!}
//!
//!let (r1,r2)=rect.subdivide(35.0,XAXIS);
//!assert!(*r1.get_range(XAXIS)==Range{start:30.0,end:35.0});
//!assert!(*r1.get_range(YAXIS)==Range{start:30.0,end:40.0});
//!	      
//!assert!(*r2.get_range(XAXIS)==Range{start:35.0,end:40.0});
//!assert!(*r2.get_range(YAXIS)==Range{start:30.0,end:40.0});
//!}
//!```
//!

mod range;
mod rect;

pub use self::range::Range;
pub use self::rect::Rect;


///The x axis implementation of the AxisTrait
#[derive(Copy,Clone)]
pub struct XAXISS;
impl AxisTrait for XAXISS{
    type Next=YAXISS;

    fn is_xaxis(&self)->bool{
        true
    }
    fn next(&self)->Self::Next{
        YAXISS
    }
}

///The y axis implementation of the AxisTrait
#[derive(Copy,Clone)]
pub struct YAXISS;
impl AxisTrait for YAXISS{
    type Next=XAXISS;

    fn is_xaxis(&self)->bool{
        false
    }
    fn next(&self)->Self::Next{
        XAXISS
    }
}

///Axis trait can be used to extract the x or y axis out of a vector
///when you know the axis as compile time.
pub trait AxisTrait:Sync+Send+Copy+Clone{
    type Next:AxisTrait;
    fn is_xaxis(&self)->bool;
    fn next(&self)->Self::Next;

    fn is_equal_to<B:AxisTrait>(&self,other:B)->bool{
        if self.is_xaxis() && other.is_xaxis(){
            return true;
        }
        if !self.is_xaxis() && !other.is_xaxis(){
            return true;
        }

        return false;
    }
}



pub struct AxisWrap<T>(pub [T;2]);
impl<T> AxisWrap<T>{
    pub fn get<A:AxisTrait>(&self,axis:A)->&T{
        if axis.is_xaxis(){
            &self.0[0]
        }else{
            &self.0[1]
        }
    }
    pub fn get_mut<A:AxisTrait>(&mut self,axis:A)->&mut T{
        if axis.is_xaxis(){
            &mut self.0[0]
        }else{
            &mut self.0[1]
        }
    }
    pub fn set<A:AxisTrait>(&mut self,axis:A,a:T){
        if axis.is_xaxis(){
            self.0[0]=a;
        }else{
            self.0[1]=a;
        }
    }
}

pub struct AxisWrapRef<'a,T:'a>(pub &'a [T;2]);
impl<'a,T:'a> AxisWrapRef<'a,T>{
    pub fn get<A:AxisTrait>(&self,axis:A)->&'a T{
        if axis.is_xaxis(){
            &self.0[0]
        }else{
            &self.0[1]
        }
    }
}


pub struct AxisWrapRefMut<'a,T:'a>(pub &'a mut [T;2]);
impl<'a,T:'a> AxisWrapRefMut<'a,T>{
    pub fn get<A:AxisTrait>(&self,axis:A)->&T{
        if axis.is_xaxis(){
            &self.0[0]
        }else{
            &self.0[1]
        }
    }
    pub fn get_mut <A:AxisTrait>(&mut self,axis:A)->&mut T{
        if axis.is_xaxis(){
            &mut self.0[0]
        }else{
            &mut self.0[1]
        }
    }
    
    pub fn set<A:AxisTrait>(&mut self,axis:A,a:T){
        if axis.is_xaxis(){
            self.0[0]=a;
        }else{
            self.0[1]=a;
        }
    }
}






#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test1() {
        let rect = Rect::new(30.0,40.0,30.0,40.0);
        for k in AxisIter::new(){
            let r=rect.get_range(k);
            assert!(r.len()==10.0);
        }

        let (r1,r2)=rect.subdivide(35.0,XAXIS);
        assert_eq!(*r1.get_range(XAXIS),Range{start:30.0,end:35.0});
        assert_eq!(*r1.get_range(YAXIS),Range{start:30.0,end:40.0});
        
        assert_eq!(*r2.get_range(XAXIS),Range{start:35.0,end:40.0});
        assert_eq!(*r2.get_range(YAXIS),Range{start:30.0,end:40.0});
    }
}
