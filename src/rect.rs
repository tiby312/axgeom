
use range::Range;
use std::fmt::Debug;

use *;

///Stored as two Ranges. 
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Rect<T:Copy>(
    pub [Range<T>;2]
);


impl<T:Copy> Rect<T>{
    #[inline(always)]
    pub fn new(a:T,b:T,c:T,d:T)->Rect<T>{
        let r1=Range{left:a,right:b};
        let r2=Range{left:c,right:d};
        Rect([r1,r2])
    }
    #[inline(always)]
    pub fn get(&self)->((T,T),(T,T)){
        let f=&self.0;
        ((f[0].left,f[0].right),(f[1].left,f[1].right))
    }
    #[inline(always)]
    pub fn as_axis(&self)->AxisWrapRef<Range<T>>{
        AxisWrapRef(&self.0)
    }
    #[inline(always)]
    pub fn as_axis_mut(&mut self)->AxisWrapRefMut<Range<T>>{
        AxisWrapRefMut(&mut self.0)
    }
}

impl<T:Ord+Copy> Rect<T>{

    
    ///Subdivides the rectangle.
    ///No floating point calculations are done (so no precision loss/rounding issues).
    #[inline(always)]
    pub fn subdivide<A:AxisTrait>(&self, axis:A,mut divider: T) -> (Rect<T>,Rect<T>) {
        
        let ca=axis;
        let na=axis.next();

        let rel=self.as_axis().get(ca);
        let carry_thru=self.as_axis().get(na);

        
        if divider<rel.left{
            divider=rel.left;
        }else if divider>rel.right{
            divider=rel.right;
        }
  
        let l=Range{left:rel.left,right:divider};
        let r=Range{left:divider,right:rel.right};

        let mut left:Rect<T>=unsafe{std::mem::uninitialized()};
        *left.as_axis_mut().get_mut(ca)=l;
        *left.as_axis_mut().get_mut(na)=*carry_thru;
        
        let mut right:Rect<T>=unsafe{std::mem::uninitialized()};
        *right.as_axis_mut().get_mut(ca)=r;
        *right.as_axis_mut().get_mut(na)=*carry_thru;
        (left,right)
        
    } 
    

    #[inline(always)]
    pub fn contains_pos(&self,a:T,b:T)->bool{
        self.as_axis().get(XAXISS).contains(a) &&
        self.as_axis().get(YAXISS).contains(b)
    }

    ///Returns true if the specified rect is inside of this rect.
    #[inline(always)]
    pub fn contains_rect(&self,rect:&Rect<T>)->bool{

        //This seems like something a macro would be suited for.

        if !self.as_axis().get(XAXISS).contains_range(rect.as_axis().get(XAXISS)) {
            return false;
        }
        if !self.as_axis().get(YAXISS).contains_range(rect.as_axis().get(YAXISS)) {
            return false;
        }

        return true;
    }

    /*
    ///Grow the rectangle to fit the specified rectangle by replacing values
    ///with the specified rectangle. No floating point computations.
    #[inline(always)]
    pub fn grow_to_fit(&mut self,rect:&Rect<T>){
        for axis in AxisIter::new() {
            let a=self.get_range_mut(axis);
            let b=rect.get_range(axis);
            
            if b.start<a.start{
                a.start=b.start;
            }
            if b.end>a.end{
                a.end=b.end;
            }
        }
    }
    */

    ///Get an intersecting rectangle.
    ///No floating point calculations as the new rectangle is made up of
    ///values from this rectangle and the specified rectangle.
    #[inline(always)]
    pub fn get_intersect_rect(&self,other:&Rect<T>)->Option<Rect<T>>{
        
        macro_rules! macro_axis{
            ($axis:ident)=>{
                {
                    let xr=other.as_axis().get($axis);
                    let xf=self.as_axis().get($axis);

                    let range=Range{left:xr.left.max(xf.left),right:xr.right.min(xf.right)};
                    
                    //TODO figure out inequality
                    if range.right<range.left{
                        return None
                    }  
                    range
                } 
            }
        }

        let r1=macro_axis!(XAXISS);
        let r2=macro_axis!(YAXISS);
        Some(Rect([r1,r2]))
    }
    
    /*
    ///Faster than using get_intersect_rect() and checking is_some().
    #[inline(always)]
    pub fn intersects_rect(&self, rect: &Rect<T>)->bool{
        for axis in AxisIter::new() {
            if !self.get_range(axis).intersects(&rect.get_range(axis)){
                return false;
            }
        }
        return true;
    }
    */

    
}
