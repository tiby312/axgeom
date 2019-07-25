
use crate::range::Range;
use crate::*;
use ordered_float::*;

///An axis aligned rectangle. Stored as two Ranges. 
///It is a fully closed rectangle. Points exactly along the border of the rectangle are considered inside the rectangle. 
#[repr(transparent)]
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Rect<T:Copy>(
    pub [Range<T>;2]
);




impl<T:num_traits::float::Float> AsRef<Rect<T>> for Rect<NotNan<T>>{
    #[inline(always)]
    fn as_ref(&self)->&Rect<T>{
       unsafe{&*( self as *const Rect<NotNan<T>> as *const Rect<T> )}
    }
}

impl<T:num_traits::float::Float> AsMut<Rect<T>> for Rect<NotNan<T>>{
    #[inline(always)]
    fn as_mut(&mut self)->&mut Rect<T>{
       unsafe{&mut *( self as *mut Rect<NotNan<T>> as *mut Rect<T> )}
    }
}

///Thrown if unable to convert rectangle of floats to NotNan.
#[derive(Debug)]
pub struct RectNanErr;

impl<T:num_traits::float::Float> Rect<T>{
    ///Convert a ractangle of floats to a rectangle of NotNan floats.
    #[inline(always)]
    pub fn into_notnan(self)->Result<Rect<NotNan<T>>,RectNanErr>{

        let a=self.get_range(XAXISS);
        let b=self.get_range(YAXISS);
        
        let floats=[NotNan::new(a.left),NotNan::new(a.right),NotNan::new(b.left),NotNan::new(b.right)];

        match floats{
            [Ok(a),Ok(b),Ok(c),Ok(d)]=>{
                Ok(Rect::new(a,b,c,d))
            },
            _=>{
                Err(RectNanErr)
            }
        }
    }
}

impl<T:num_traits::float::Float> Rect<NotNan<T>>{
    ///Convert a rectangle of NotNan floats to primitive floats.
    #[inline(always)]
    pub fn into_inner(self)->Rect<T>{
        let ((x1,x2),(y1,y2))=self.get();
        Rect::new(x1.into_inner(),x2.into_inner(),y1.into_inner(),y2.into_inner())
    }
}


impl<T:Copy+core::ops::Sub<Output=T>+core::ops::Add<Output=T>> Rect<T>{
    ///Create a rectangle from a point and radius.
    #[inline(always)]
    pub fn from_point(point:[T;2],radius:[T;2])->Rect<T>{
        Rect::new(point[0]-radius[0],point[0]+radius[0],point[1]-radius[1],point[1]+radius[1])
    }  
}

impl<T:Copy> Rect<T>{

    ///(a,b) is the x component range.
    ///(c,d) is the y component range.
    #[inline(always)]
    pub fn new(a:T,b:T,c:T,d:T)->Rect<T>{
        let r1=Range{left:a,right:b};
        let r2=Range{left:c,right:d};
        Rect([r1,r2])
    }

    ///(a,b) is the x component range.
    ///(c,d) is the y component range.
    #[inline(always)]
    pub fn get(&self)->((T,T),(T,T)){
        let f=&self.0;
        ((f[0].left,f[0].right),(f[1].left,f[1].right))
    }

    ///Get the range of one axis.
    #[inline(always)]
    pub fn get_range(&self,axis:impl AxisTrait)->&Range<T>{
        if axis.is_xaxis(){
            &self.0[0]
        }else{
            &self.0[1]
        }
    }
    
    ///Get the mutable range of one axis.
    #[inline(always)]
    pub fn get_range_mut(&mut self,axis:impl AxisTrait)->&mut Range<T>{
        if axis.is_xaxis(){
            &mut self.0[0]
        }else{
            &mut self.0[1]
        }
    }
}

impl<T:PartialOrd+Copy> Rect<T>{

    ///Returns true if the point is contained in the the ranges of both axis.
    #[inline(always)]
    pub fn contains_point(&self,a:[T;2])->bool{
        self.get_range(XAXISS).contains(a[0]) &&
        self.get_range(YAXISS).contains(a[1])
    }
}


impl<T:Copy+core::ops::Sub<Output=T>+core::ops::Add<Output=T>> Rect<T>{
    ///Grow a rectangle of a radius.
    #[inline(always)]
    pub fn grow(&mut self,radius:T)->&mut Self{
        self.0[0].grow(radius);
        self.0[1].grow(radius);
        self
    }
}

impl<T:Ord+Copy> Rect<T>{

    ///Returns true if two rectangles has the same values.
    #[inline(always)]
    pub fn equals(&self,a:&Rect<T>)->bool{
        //TODO optimize
        let ((a1,b1),(c1,d1))=self.get();
        let ((a2,b2),(c2,d2))=a.get();

        (a1==a2)&&(b1==b2)&&(c1==c2)&&(d1==d2)
    }
    
    ///Subdivides the rectangle.
    ///No floating point calculations are done (so no precision loss/rounding issues).
    #[inline(always)]
    pub fn subdivide<A:AxisTrait>(&self, axis:A,mut divider: T) -> (Rect<T>,Rect<T>) {
        
        let ca=axis;
        let na=axis.next();

        let rel=self.get_range(ca);
        let carry_thru=*self.get_range(na);

        
        if divider<rel.left{
            divider=rel.left;
        }else if divider>rel.right{
            divider=rel.right;
        }
  
        let l=Range{left:rel.left,right:divider};
        let r=Range{left:divider,right:rel.right};


        if axis.is_xaxis(){
            (Rect([l,carry_thru]),Rect([r,carry_thru]))
        }else{
             (Rect([carry_thru,l]),Rect([carry_thru,r]))
        }
    } 
    
    ///Returns true if the rectangle's ranges are not degenerate.
    #[inline(always)]
    pub fn is_valid(&self)->bool{
        self.0[0].is_valid() &&
        self.0[0].is_valid()
    }


    ///Returns true if the specified rect is inside of this rect.
    #[inline(always)]
    pub fn contains_rect(&self,rect:&Rect<T>)->bool{

        if !self.get_range(XAXISS).contains_range(rect.get_range(XAXISS)) {
            return false;
        }
        if !self.get_range(YAXISS).contains_range(rect.get_range(YAXISS)) {
            return false;
        }
        true
    }

    
    ///Grow the rectangle to fit the specified rectangle by replacing values
    ///with the specified rectangle. No floating point computations.
    #[inline(always)]
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
    
    #[inline(always)]
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
        true
    }

    ///Get an intersecting rectangle.
    ///No floating point calculations as the new rectangle is made up of
    ///values from this rectangle and the specified rectangle.
    #[inline(always)]
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
