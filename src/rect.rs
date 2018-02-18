use std::mem;
use range::Range;
use vec::VecCont;
use vec::Vec2;
use vec::XAXIS;
use vec::YAXIS;
use vec::Axis;
use vec::AxisIter;
use std::fmt::Debug;

use XAXIS_S;
use YAXIS_S;
///Stored as two Ranges. 
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Rect<T:Copy+Debug>{
    a:VecCont<Range<T>>
}

use AxisTrait;

impl<T:Copy+Debug> Rect<T>{

    #[inline(always)]
    pub fn new(a:T,b:T,c:T,d:T)->Rect<T>{
        Rect{a:VecCont::new(Range{start:a,end:b},Range{start:c,end:d})}
    }

    #[inline(always)]
    pub fn get_range2<X:AxisTrait>(&self)->&Range<T>{
        //TODO optimize furthur?
        self.a.get_axis(X::get())
    }

    #[inline(always)]
    pub fn get_range2_mut<X:AxisTrait>(&mut self)->&mut Range<T>{
        //TODO optimize furthur?
        self.a.get_axis_mut(X::get())
    }
    
    #[inline(always)]
    pub fn get_range(&self,axis:Axis)->&Range<T>{
        self.a.get_axis(axis)
    }

    #[inline(always)]
    pub fn get_range_mut(&mut self,axis:Axis)->&mut Range<T>{
        self.a.get_axis_mut(axis)
    }
    
}
impl Rect<f32>{
    ///Creates a Rect where the pos is in the center, had the edges are spaced a radius away.
    #[inline(always)]
    pub fn from_pos_and_radius(pos:&Vec2,radius:f32)->Rect<f32>{
        
        let rel=pos.get_axis(XAXIS);
        let a=Range{start:rel-radius,end:rel+radius};
        
        let rel=pos.get_axis(YAXIS);
        let b=Range{start:rel-radius,end:rel+radius};
        
        Rect{a:VecCont::new(a,b)}
    }

    //TODO GET RID OF. Require ORD trait
    #[inline(always)]
    pub fn contains_vec(&self,pos:&Vec2)->bool{
        let x=*(pos.get().0);
        let y=*(pos.get().1);
        let a=self.get_range2::<XAXIS_S>();
        let b=self.get_range2::<YAXIS_S>();
        x>=a.start&&x<=a.end && y>=b.start&&y<=b.end 
    }
    

    #[inline(always)]
    pub fn midpoint(&self)->Vec2{
        let a=self.get_range2::<XAXIS_S>().midpoint();
        let b=self.get_range2::<YAXIS_S>().midpoint();
        Vec2::new(a,b)
    }
    
    ///Grow in all directions by val.
    #[inline(always)]
    pub fn grow(&mut self,val:f32)->&mut Rect<f32>{
        self.get_range2_mut::<XAXIS_S>().grow(val);
        self.get_range2_mut::<YAXIS_S>().grow(val);
        
        self
    }
}

impl<T:PartialOrd+PartialEq+Copy+Debug> Rect<T>{
    ///Subdivides the rectangle.
    ///No floating point calculations are done (so no precision loss/rounding issues).
    #[inline(always)]
    pub fn subdivide(&self, mut divider: T, axis: Axis) -> (Rect<T>,Rect<T>) {

        let ca=axis;
        let na=axis.next();

        let rel=self.a.get_axis(ca);
        let carry_thru=self.a.get_axis(na);

        
        if divider<rel.start{
            divider=rel.start;
        }else if divider>rel.end{
            divider=rel.end;
        }
        //TODO move some of this code into Range.
        //TODO check algoritm is okay?
        //assert!(divider>=rel.start,"{:?}",(divider,rel));
        //assert!(divider<=rel.end,"{:?}",(divider,rel));
        

        let l=Range{start:rel.start,end:divider};
        let r=Range{start:divider,end:rel.end};

        let mut left:Rect<T>=unsafe{mem::uninitialized()};
        *left.a.get_axis_mut(ca)=l;
        *left.a.get_axis_mut(na)=*carry_thru;
        
        let mut right:Rect<T>=unsafe{mem::uninitialized()};
        *right.a.get_axis_mut(ca)=r;
        *right.a.get_axis_mut(na)=*carry_thru;
        (left,right)
    } 
}
impl<T:Ord+Copy+Debug> Rect<T>{


    #[inline(always)]
    pub fn contains_pos(&self,a:T,b:T)->bool{
        self.get_range2::<XAXIS_S>().contains(a) &&
        self.get_range2::<YAXIS_S>().contains(b)
    }

    ///Returns true if the specified rect is inside of this rect.
    #[inline(always)]
    pub fn contains_rect(&self,rect:&Rect<T>)->bool{

        //This seems like something a macro would be suited for.

        if !self.get_range2::<XAXIS_S>().contains_range(&rect.get_range2::<XAXIS_S>()) {
            return false;
        }
        if !self.get_range2::<YAXIS_S>().contains_range(&rect.get_range2::<YAXIS_S>()) {
            return false;
        }

        true
    }

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

    ///Get an intersecting rectangle.
    ///No floating point calculations as the new rectangle is made up of
    ///values from this rectangle and the specified rectangle.
    #[inline(always)]
    pub fn get_intersect_rect(&self,rect:&Rect<T>)->Option<Rect<T>>{
        
        let mut rr:Rect<T>=unsafe{mem::uninitialized()};
        for axis in AxisIter::new() {
            //TODO use range's methods
            let a=self.get_range(axis);
            let b=rect.get_range(axis);

            let left=a.start.max(b.start);
            let right=a.end.min(b.end);
            rr.get_range_mut(axis).start=left;
            rr.get_range_mut(axis).end=right;
        
            if right<=left{
                return None;
            }
        }

        Some(rr)
    }
    
    
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
    

    
}
