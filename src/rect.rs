use std::mem;

use range::Range;
use vec::PRIMT;
use vec::VecCont;
use vec::Vec2;
use vec::XAXIS;
use vec::YAXIS;
use vec::Axis;
//Need to use this version so that subdivide doesnt result in floating point rounding
//that may invalidate invariants


///Stored as two Ranges. 
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Rect{
    a:VecCont<Range>
}
impl Rect{

    #[inline(always)]
    pub fn new(a:PRIMT,b:PRIMT,c:PRIMT,d:PRIMT)->Rect{
        Rect{a:VecCont::new(Range{start:a,end:b},Range{start:c,end:d})}
    }

    #[inline(always)]
    pub fn from_pos_and_radius(pos:&Vec2,radius:PRIMT)->Rect{
        
        let rel=pos.get_axis(XAXIS);
        let a=Range{start:rel-radius,end:rel+radius};
        
        let rel=pos.get_axis(YAXIS);
        let b=Range{start:rel-radius,end:rel+radius};
        
        Rect{a:VecCont::new(a,b)}
    }

    #[inline(always)]
    pub fn midpoint(&self)->Vec2{
        let a=self.get_range2(XAXIS).midpoint();
        let b=self.get_range2(YAXIS).midpoint();
        Vec2::new(a,b)
    }

    #[inline(always)]
    pub fn get_range2(&self,axis:Axis)->&Range{
        self.a.get_axis(axis)
    }

    #[inline(always)]
    pub fn get_range2_mut(&mut self,axis:Axis)->&mut Range{
        self.a.get_axis_mut(axis)
    }

    #[inline(always)]
    pub fn grow(&mut self,val:PRIMT)->&mut Rect{
        for axis in Axis::get_axis_iter() {
            self.get_range2_mut(axis).grow(val);
        }
        self
    }

    #[inline(always)]
    pub fn contains_rect(&self,rect:&Rect)->bool{
        for axis in Axis::get_axis_iter() {
            if !self.get_range2(axis).contains_rang(&rect.get_range2(axis)) {
                return false;
            }        
        }
        true
    }

    #[inline(always)]
    pub fn grow_to_fit(&mut self,rect:&Rect){
        for axis in Axis::get_axis_iter() {
            let a=self.get_range2_mut(axis);
            let b=rect.get_range2(axis);
            
            if b.start<a.start{
                a.start=b.start;
            }
            if b.end>a.end{
                a.end=b.end;
            }
        }
    }

    #[inline(always)]
    pub fn get_longer_axis(&self)->Axis{
        if self.get_range2(XAXIS).len()>self.get_range2(YAXIS).len(){
            XAXIS
        }else{
            YAXIS
        }
    }

    #[inline(always)]
    pub fn get_intersect_rect(&self,rect:&Rect)->Option<Rect>{
        
        let mut rr:Rect=unsafe{mem::uninitialized()};
        for axis in Axis::get_axis_iter() {
            let a=self.get_range2(axis);
            let b=rect.get_range2(axis);

            let left=a.start.max(b.start);
            let right=a.end.min(b.end);
            rr.get_range2_mut(axis).start=left;
            rr.get_range2_mut(axis).end=right;
        
            if right<=left{
                return None;
            }
        }

        Some(rr)
    }
    
    #[inline(always)]
    pub fn intersects_rect(&self, rect: &Rect)->bool{
        for axis in Axis::get_axis_iter() {
            if !self.get_range2(axis).intersects(&rect.get_range2(axis)){
                return false;
            }
        }
        return true;
    }

    #[inline(always)]
    pub fn subdivide(&self, mut divider: PRIMT, axis: Axis) -> (Rect,Rect) {

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

        let mut left:Rect=unsafe{mem::uninitialized()};
        *left.a.get_axis_mut(ca)=l;
        *left.a.get_axis_mut(na)=*carry_thru;
        
        let mut right:Rect=unsafe{mem::uninitialized()};
        *right.a.get_axis_mut(ca)=r;
        *right.a.get_axis_mut(na)=*carry_thru;
        (left,right)
    }
}
