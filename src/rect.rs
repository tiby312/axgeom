
use crate::range::Range;
use crate::*;

use crate::num_traits::NumCast;

use cgmath::prelude::*;
use cgmath::vec2;
use cgmath::Vector2;
use cgmath::BaseFloat;

///An axis aligned rectangle. Stored as two Ranges. 
///It is a fully closed rectangle. Points exactly along the border of the rectangle are considered inside the rectangle. 
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Rect<T>{
    pub x:Range<T>,
    pub y:Range<T>
}



impl<S: NumCast + Copy> Rect<S> {
    /// Component-wise casting to another type.
    #[inline(always)]
    pub fn cast<T: NumCast>(&self) -> Option<Rect<T>> {
        let a=self.x.cast();
        let b=self.y.cast();
        match (a,b){
            (Some(x),Some(y))=>{
                Some(Rect{x,y})
            },
            _=>{
                None
            }
        }
    }
}


impl<T:Copy+core::ops::Sub<Output=T>+core::ops::Add<Output=T>> Rect<T>{
    ///Create a rectangle from a point and radius.
    #[inline(always)]
    pub fn from_point(point:[T;2],radius:[T;2])->Rect<T>{
        let x=Range::from_point(point[0],radius[0]);
        let y=Range::from_point(point[1],radius[1]);
        Rect{x,y}
    }  
}

impl<T:Copy> Rect<T>{

    ///(a,b) is the x component range.
    ///(c,d) is the y component range.
    #[inline(always)]
    pub fn new(a:T,b:T,c:T,d:T)->Rect<T>{
        let x=Range{left:a,right:b};
        let y=Range{left:c,right:d};
        Rect{x,y}
    }

    ///(a,b) is the x component range.
    ///(c,d) is the y component range.
    #[inline(always)]
    pub fn get(&self)->((T,T),(T,T)){
        let f=self;
        ((f.x.left,f.x.right),(f.y.left,f.y.right))
    }

    ///Get the range of one axis.
    #[inline(always)]
    pub fn get_range(&self,axis:impl AxisTrait)->&Range<T>{
        if axis.is_xaxis(){
            &self.x
        }else{
            &self.y
        }
    }
    
    ///Get the mutable range of one axis.
    #[inline(always)]
    pub fn get_range_mut(&mut self,axis:impl AxisTrait)->&mut Range<T>{
        if axis.is_xaxis(){
            &mut self.x
        }else{
            &mut self.y
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
        self.x.grow(radius);
        self.y.grow(radius);
        self
    }
}

impl<T:Copy+PartialOrd+core::ops::Sub<Output=T>+core::ops::Mul<Output=T>+core::ops::Add<Output=T>> Rect<T>{

    ///If the point is outisde the rectangle, returns the squared distance from a point to a rectangle.
    ///If the point is inside the rectangle, it will return None.
    pub fn distance_squared_to_point(
        &self,
        point: [T; 2],
    ) -> Option<T> {
        let (px, py) = (point[0], point[1]);

        let ((a, b), (c, d)) = self.get();

        let xx = num_traits::clamp(px, a, b);
        let yy = num_traits::clamp(py, c, d);

        let dis = (xx - px) * (xx - px) + (yy - py) * (yy - py);

        //Then the point must be insert the rect.
        //In this case, lets return something negative.
        if xx > a && xx < b && yy > c && yy < d {
            None
        } else {
            Some(dis)
        }
    }



}

impl<T:num_traits::Num+Copy> Rect<T>{

    pub fn derive_center(&self) -> Vector2<T> {
        let two = T::one() + T::one();
        let ((a, b), (c, d)) = self.get();
        vec2(a + (b - a) / two, c + (d - c) / two)
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
            (Rect{x:l,y:carry_thru},Rect{x:r,y:carry_thru})
        }else{
             (Rect{x:carry_thru,y:l},Rect{x:carry_thru,y:r})
        }
    } 
    
    ///Returns true if the rectangle's ranges are not degenerate.
    #[inline(always)]
    pub fn is_valid(&self)->bool{
        self.x.is_valid() &&
        self.y.is_valid()
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
        if other.x.intersects(&self.x) && other.y.intersects(&self.y){
            true
        }else{
            false
        }
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

        let x=macro_axis!(XAXISS);
        let y=macro_axis!(YAXISS);
        Some(Rect{x,y})
    }    
}
