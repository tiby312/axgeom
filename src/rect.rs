
use crate::range::Range;
use crate::*;

///An axis aligned rectangle. Stored as two Ranges. 
///It is a fully closed rectangle. Points exactly along the border of the rectangle are considered inside the rectangle. 
#[repr(transparent)]
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Rect<T:Copy>(
    pub [Range<T>;2]
);


impl<T:Copy> Rect<T>{

    ///(a,b) is the x component range.
    ///(c,d) is the y component range.
    #[inline]
    pub fn new(a:T,b:T,c:T,d:T)->Rect<T>{
        let r1=Range{left:a,right:b};
        let r2=Range{left:c,right:d};
        Rect([r1,r2])
    }

    ///(a,b) is the x component range.
    ///(c,d) is the y component range.
    #[inline]
    pub fn get(&self)->((T,T),(T,T)){
        let f=&self.0;
        ((f[0].left,f[0].right),(f[1].left,f[1].right))
    }

    #[inline]
    pub fn get_range(&self,axis:impl AxisTrait)->&Range<T>{
        if axis.is_xaxis(){
            &self.0[0]
        }else{
            &self.0[1]
        }
    }
    #[inline]
    pub fn get_range_mut(&mut self,axis:impl AxisTrait)->&mut Range<T>{
        if axis.is_xaxis(){
            &mut self.0[0]
        }else{
            &mut self.0[1]
        }
    }
}

impl<T:PartialOrd+Copy> Rect<T>{

    #[inline]
    pub fn contains_point(&self,a:[T;2])->bool{
        self.get_range(XAXISS).contains(a[0]) &&
        self.get_range(YAXISS).contains(a[1])
    }
}


impl<T:Copy+std::ops::Sub<Output=T>+std::ops::Add<Output=T>> Rect<T>{
    #[inline]
    pub fn grow(&mut self,radius:T)->&mut Self{
        self.0[0].grow(radius);
        self.0[1].grow(radius);
        self
    }
}

impl<T:Ord+Copy> Rect<T>{
    #[inline]
    pub fn equals(&self,a:&Rect<T>)->bool{
        //TODO optimize
        let ((a1,b1),(c1,d1))=self.get();
        let ((a2,b2),(c2,d2))=a.get();

        (a1==a2)&&(b1==b2)&&(c1==c2)&&(d1==d2)
    }
    
    ///Subdivides the rectangle.
    ///No floating point calculations are done (so no precision loss/rounding issues).
    #[inline]
    pub fn subdivide<A:AxisTrait>(&self, axis:A,mut divider: T) -> (Rect<T>,Rect<T>) {
        
        let ca=axis;
        let na=axis.next();

        let rel=self.get_range(ca);
        let carry_thru=self.get_range(na);

        
        if divider<rel.left{
            divider=rel.left;
        }else if divider>rel.right{
            divider=rel.right;
        }
  
        let l=Range{left:rel.left,right:divider};
        let r=Range{left:divider,right:rel.right};

        let mut left:Rect<T>=unsafe{std::mem::uninitialized()};
        *left.get_range_mut(ca)=l;
        *left.get_range_mut(na)=*carry_thru;
        
        let mut right:Rect<T>=unsafe{std::mem::uninitialized()};
        *right.get_range_mut(ca)=r;
        *right.get_range_mut(na)=*carry_thru;
        (left,right)
        
    } 
    


    ///Returns true if the specified rect is inside of this rect.
    #[inline]
    pub fn contains_rect(&self,rect:&Rect<T>)->bool{

        //This seems like something a macro would be suited for.

        if !self.get_range(XAXISS).contains_range(rect.get_range(XAXISS)) {
            return false;
        }
        if !self.get_range(YAXISS).contains_range(rect.get_range(YAXISS)) {
            return false;
        }

        return true;
    }

    
    ///Grow the rectangle to fit the specified rectangle by replacing values
    ///with the specified rectangle. No floating point computations.
    #[inline]
    pub fn grow_to_fit(&mut self,rect:&Rect<T>)->&mut Self{
        {
            macro_rules! macro_axis
            {
                ($axis:ident)=>
                {
                    {
                        let sx=self.get_range_mut($axis);
                        let rx=rect.get_range($axis);
                        sx.grow_to_fit(rx);
                    }
                }
            }

            macro_axis!(XAXISS);
            macro_axis!(YAXISS);
        }
        self
    }
    
    #[inline]
    pub fn intersects_rect(&self,other:&Rect<T>)->bool{
        macro_rules! macro_axis{
            ($axis:ident)=>{
                {
                    let xr=other.get_range($axis);
                    let xf=self.get_range($axis);

                    if !xr.intersects(xf){
                        return false
                    }
                } 
            }
        }

        macro_axis!(XAXISS);
        macro_axis!(YAXISS);
        return true;
    }

    ///Get an intersecting rectangle.
    ///No floating point calculations as the new rectangle is made up of
    ///values from this rectangle and the specified rectangle.
    #[inline]
    pub fn get_intersect_rect(&self,other:&Rect<T>)->Option<Rect<T>>{
        
        macro_rules! macro_axis{
            ($axis:ident)=>{
                {
                    let xr=other.get_range($axis);
                    let xf=self.get_range($axis);

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
}
