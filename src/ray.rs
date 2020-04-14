use crate::*;
use core::cmp::Ordering;
use core::convert::TryFrom;
use primitive_from::PrimitiveFrom;

///Convenience function to create a ray.
#[must_use]
pub fn ray<N>(point: Vec2<N>, dir: Vec2<N>) -> Ray<N> {
    Ray { point, dir }
}

///A Ray.
#[derive(Debug, Copy, Clone)]
#[must_use]
pub struct Ray<N> {
    pub point: Vec2<N>,
    pub dir: Vec2<N>,
}

impl<B: Copy> Ray<B> {
    #[inline(always)]
    pub fn inner_as<A: PrimitiveFrom<B>>(&self) -> Ray<A> {
        Ray {
            point: self.point.inner_as(),
            dir: self.dir.inner_as(),
        }
    }
}

impl<N: Copy + core::ops::Add<Output = N> + core::ops::Mul<Output = N>> Ray<N> {
    #[inline(always)]
    pub fn point_at_tval(&self, tval: N) -> Vec2<N> {
        self.point + self.dir * tval
    }
}
impl<N> Ray<N> {
    #[inline(always)]
    pub fn inner_into<B: From<N>>(self) -> Ray<B> {
        let point = self.point.inner_into();
        let dir = self.dir.inner_into();
        Ray { point, dir }
    }
    #[inline(always)]
    pub fn inner_try_into<B: TryFrom<N>>(self) -> Result<Ray<B>, B::Error> {
        let point = self.point.inner_try_into();
        let dir = self.dir.inner_try_into();
        match (point, dir) {
            (Ok(point), Ok(dir)) => Ok(Ray { point, dir }),
            (Err(e), Ok(_)) => Err(e),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Err(_)) => Err(e),
        }
    }
}

impl<N: PartialOrd + Copy> Ray<N> {
    #[inline(always)]
    pub fn range_side(&self, axis: impl Axis, range: &Range<N>) -> Ordering {
        let v = if axis.is_xaxis() {
            self.point.x
        } else {
            self.point.y
        };

        range.contains_ext(v)
    }
}

///Describes if a ray hit a rectangle.
#[derive(Copy, Clone, Debug)]
#[must_use]
pub enum CastResult<N> {
    Hit(N),
    NoHit,
}

impl<N> CastResult<N> {
    #[inline(always)]
    pub fn map<X>(self, mut func: impl FnMut(N) -> X) -> CastResult<X> {
        match self {
            CastResult::Hit(a) => CastResult::Hit(func(a)),
            CastResult::NoHit => CastResult::NoHit,
        }
    }

    #[inline(always)]
    pub fn unwrap(self) -> N {
        match self {
            CastResult::Hit(a) => a,
            CastResult::NoHit => panic!("unwrapped a NoHit in CastResult"),
        }
    }
}

use roots;
use roots::*;
impl<N: num_traits::Float + roots::FloatType> Ray<N> {
    ///Checks if a ray intersects a circle.
    pub fn cast_to_circle(&self, center: Vec2<N>, radius: N) -> CastResult<N> {
        //https://math.stackexchange.com/questions/311921/get-location-of-vector-circle-intersection
        //circle
        //(x-center.x)^2+(y-center.y)^2=r2
        //ray
        //x(t)=ray.dir.x*t+ray.point.x
        //y(t)=ray.dir.y*t+ray.point.y
        //
        //solve for t.
        //
        //
        //we get:
        //
        //𝑎𝑡^2+𝑏𝑡+𝑐=0
        //
        //
        //
        //
        let ray = self;
        let zz = <N as FloatType>::zero();
        let two = <N as FloatType>::one()+<N as FloatType>::one();

        let a = ray.dir.x.powi(2) + ray.dir.y.powi(2);
        let b =
            two * ray.dir.x * (ray.point.x - center.x) + two * ray.dir.y * (ray.point.y - center.y);
        let c =
            (ray.point.x - center.x).powi(2) + (ray.point.y - center.y).powi(2) - radius.powi(2);

        match find_roots_quadratic(a, b, c) {
            Roots::No(_) => CastResult::NoHit,
            Roots::One([a]) => {
                if a < zz {
                    CastResult::NoHit
                } else {
                    CastResult::Hit(a)
                }
            }
            Roots::Two([a, b]) => {
                let (closer, further) = if a < b { (a, b) } else { (b, a) };

                if closer < zz && further < zz {
                    CastResult::NoHit
                } else if closer < zz && further > zz {
                    CastResult::Hit(<N as FloatType>::zero())
                } else {
                    CastResult::Hit(closer)
                }
            }
            _ => unreachable!(),
        }
    }
}

//TODO make a float specific one




impl<N: num_traits::float::Float + num_traits::Num + num_traits::Signed + PartialOrd + Copy  + core::fmt::Debug> Ray<N> {
    

    //if axis is x, then the line is top to bottom
    //if axis is y, then the line is left to right
    pub fn cast_to_aaline<A:Axis>(&self,a:A,line:N)->CastResult<N>{
        let ray=self;
        let  tval=if a.is_xaxis(){
            //ray.point.x+ray.dir.x*t=line
            //ray.point.x-line=ray.dir.x*t
            //

            
            (ray.point.x-line)/ray.dir.x
        }else{
            (ray.point.y-line)/ray.dir.y
        };

        if tval>N::zero() && !tval.is_nan(){
            CastResult::Hit(tval)
        }else{
            CastResult::NoHit
        }

    }


    pub fn find_candidate_planes(&self,rect:&Rect<N>)->[bool;4]{
        //In cases where the ray is directly vertical or horizant, 
        //we technically only need to check one side of the rect.
        //but these cases are so rare, and it doesnt hurt much to check
        //one exra side. So we condense these cases into cases
        //where we check two sides.

        let x=self.dir.x>N::zero();
        let y=self.dir.y>N::zero();
        
        match (x,y){
            (true,true)=>{
                //left top 
            },
            (true,false)=>{
                //left bottom
            },
            (false,true)=>{
                //right top
            },
            (false,false)=>{
                //right bottom
            }
        }

        //Observation to make is that in each case, there was on x and one y coordinate.

        todo!()

    }

    pub fn cast_to_rect2(&self,rect:&Rect<N>)->CastResult<N>{
        let &Rect{x:Range{start:startx,end:endx},y:Range{start:starty,end:endy}}=rect;


        let x=if self.dir.x>N::zero(){
            startx
        }else{
            endx
        };
        let y=if self.dir.y>N::zero(){
            starty
        }else{
            endy
        };

        let tval1=self.cast_to_aaline(XAXIS,x);
        let tval2=self.cast_to_aaline(YAXIS,y);


        use CastResult::*;
        //return tval2;
        match (tval1,tval2){
            (Hit(a),Hit(b))=>{

                //xaxis hit
                if a>b{
                    let yy=self.point.y+self.dir.y*a;
                    if rect.y.contains(yy){
                        Hit(a)
                    }else{
                        NoHit
                    }
                }else{
                    let xx=self.point.x+self.dir.x*b;
                    if rect.x.contains(xx){
                        Hit(b)
                    }else{
                        NoHit
                    }
                }
            },
            (Hit(a),NoHit)=>{
                let yy=self.point.y+self.dir.y*a;
                if rect.y.contains(yy){
                    Hit(a)
                }else{
                    NoHit
                }
            },
            (NoHit,Hit(b))=>{
                let xx=self.point.x+self.dir.x*b;
                if rect.x.contains(xx){
                    Hit(b)
                }else{
                    NoHit
                }
            },
            (NoHit,NoHit)=>{
                NoHit
            }

        }


        
    }

    /*
    ///Returns if a ray intersects a box.
    
    pub fn cast_to_rect(&self, rect: &Rect<N>) -> CastResult<N> {
        let ray = self;

        //Find the corner that the ray will hit one of its sides with.
        let next_grid_pos = {
            vec2(
                if ray.dir.x < N::zero() {
                    rect.x.end
                } else if ray.dir.x > N::zero() {
                    rect.x.start
                } else {
                    if rect.x.contains(ray.point.x) {
                        let diff = rect.y.difference_to_point(ray.point.y);
                        match diff {
                            Some(diff) => {
                                if diff.signum() == -ray.dir.y.signum() {
                                    return CastResult::Hit(diff.abs());
                                } else {
                                    return CastResult::NoHit;
                                }
                            }
                            None => return CastResult::Hit(N::zero()),
                        }
                    } else {
                        return CastResult::NoHit;
                    }
                },
                if ray.dir.y < N::zero() {
                    rect.y.end
                } else if ray.dir.y > N::zero() {
                    rect.y.start
                } else {
                    if rect.y.contains(ray.point.y) {
                        let diff = rect.x.difference_to_point(ray.point.x);
                        match diff {
                            Some(diff) => {
                                if diff.signum() == -ray.dir.x.signum() {
                                    return CastResult::Hit(diff.abs());
                                } else {
                                    return CastResult::NoHit;
                                }
                            }
                            None => return CastResult::Hit(N::zero()),
                        }
                    } else {
                        return CastResult::NoHit;
                    }
                },
            )
        };

        //Compute the tval of hitting both the x and y axis.
        let tvalx = (next_grid_pos.x - ray.point.x) / ray.dir.x;
        let tvaly = (next_grid_pos.y - ray.point.y) / ray.dir.y;

        fn as_positive<N: PartialOrd + num_traits::Signed>(a: N) -> Option<N> {
            if a > N::zero() {
                Some(a)
            } else {
                None
            }
        }

        match (as_positive(tvalx), as_positive(tvaly)) {
            (Some(x), Some(y)) => {
                let x = if rect.y.contains(ray.point_at_tval(x).y) {
                    Some(x)
                } else {
                    None
                };

                let y = if rect.x.contains(ray.point_at_tval(y).x) {
                    Some(y)
                } else {
                    None
                };

                match (x, y) {
                    (Some(x), Some(y)) => CastResult::Hit(x.min(y)),
                    (Some(x), None) => CastResult::Hit(x),
                    (None, Some(y)) => CastResult::Hit(y),
                    (None, None) => CastResult::NoHit,
                }
            }
            (Some(x), None) => {
                if rect.y.contains(ray.point_at_tval(x).y) {
                    CastResult::Hit(x)
                } else {
                    CastResult::NoHit
                }
            }
            (None, Some(y)) => {
                if rect.x.contains(ray.point_at_tval(y).x) {
                    CastResult::Hit(y)
                } else {
                    CastResult::NoHit
                }
            }
            (None, None) => {
                if rect.contains_point(ray.point) {
                    CastResult::Hit(N::zero())
                } else {
                    CastResult::NoHit
                }
            }
        }
    }*/
}
