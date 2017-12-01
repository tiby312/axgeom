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

#![feature(ord_max_min)]
mod vec;
mod range;
mod rect;

pub use self::vec::Vec2;
pub use self::vec::ComputedVec2;
pub use self::vec::VecCont;
pub use self::range::Range;
pub use self::vec::Axis;
pub use self::vec::AxisIter;
pub use self::rect::Rect;
pub use self::vec::XAXIS;
pub use self::vec::YAXIS;

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
