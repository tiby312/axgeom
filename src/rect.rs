use std::mem;
a
use Range2;
use PRIMT;
use VecCont;
use Vec2;
use XAXIS;
use YAXIS;
use Axis;
//Need to use this version so that subdivide doesnt result in floating point rounding
//that may invalidate invariants

#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct RectAbsolute{
    a:VecCont<Range2>
}
impl RectAbsolute{

    pub fn new(a:PRIMT,b:PRIMT,c:PRIMT,d:PRIMT)->RectAbsolute{
        RectAbsolute{a:VecCont::new(Range2{start:a,end:b},Range2{start:c,end:d})}
    }

    pub fn from_pos_and_radius(pos:&Vec2,radius:PRIMT)->RectAbsolute{
        
        let rel=pos.get_axis(XAXIS);
        let a=Range2{start:rel-radius,end:rel+radius};
        
        let rel=pos.get_axis(YAXIS);
        let b=Range2{start:rel-radius,end:rel+radius};
        
        RectAbsolute{a:VecCont::new(a,b)}
    }

    pub fn midpoint(&self)->Vec2{
        let a=self.get_range2(XAXIS).midpoint();
        let b=self.get_range2(YAXIS).midpoint();
        Vec2::new(a,b)
    }

    pub fn get_range2(&self,axis:Axis)->&Range2{
        self.a.get_axis(axis)
    }

    pub fn get_range2_mut(&mut self,axis:Axis)->&mut Range2{
        self.a.get_axis_mut(axis)
    }

    pub fn grow(&mut self,val:PRIMT)->&mut RectAbsolute{
        for axis in Axis::get_axis_iter() {
            self.get_range2_mut(axis).grow(val);
        }
        self
    }

    pub fn contains_rect(&self,rect:&RectAbsolute)->bool{
        for axis in Axis::get_axis_iter() {
            if !self.get_range2(axis).contains_rang(&rect.get_range2(axis)) {
                return false;
            }        
        }
        true
    }

    pub fn grow_to_fit(&mut self,rect:&RectAbsolute){
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

    pub fn get_longer_axis(&self)->Axis{
        if self.get_range2(XAXIS).len()>self.get_range2(YAXIS).len(){
            XAXIS
        }else{
            YAXIS
        }
    }

    pub fn get_intersect_rect(&self,rect:&RectAbsolute)->Option<RectAbsolute>{
        
        let mut rr:RectAbsolute=unsafe{mem::uninitialized()};
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
    
    pub fn intersects_rect(&self, rect: &RectAbsolute)->bool{
        for axis in Axis::get_axis_iter() {
            if !self.get_range2(axis).intersects(&rect.get_range2(axis)){
                return false;
            }
        }
        return true;
    }


    pub fn subdivide(&self, mut divider: PRIMT, axis: Axis) -> (RectAbsolute,RectAbsolute) {

        let ca=axis;
        let na=axis.next2();

        let rel=self.a.get_axis(ca);
        let carry_thru=self.a.get_axis(na);

        
        if divider<rel.start{
            divider=rel.start;
        }else if divider>rel.end{
            divider=rel.end;
        }
        //TODO check algoritm is okay?
        //assert!(divider>=rel.start,"{:?}",(divider,rel));
        //assert!(divider<=rel.end,"{:?}",(divider,rel));
        

        let l=Range2{start:rel.start,end:divider};
        let r=Range2{start:divider,end:rel.end};

        let mut left:RectAbsolute=unsafe{mem::uninitialized()};
        *left.a.get_axis_mut(ca)=l;
        *left.a.get_axis_mut(na)=*carry_thru;
        
        let mut right:RectAbsolute=unsafe{mem::uninitialized()};
        *right.a.get_axis_mut(ca)=r;
        *right.a.get_axis_mut(na)=*carry_thru;
        (left,right)
    }
}
