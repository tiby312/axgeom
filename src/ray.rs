use crate::*;
use core::cmp::Ordering;
use core::convert::TryInto;

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
    #[must_use]
    pub fn inner_as<C:'static+Copy>(&self) -> Ray<C> where B: num_traits::AsPrimitive<C>{
        ray(self.point.inner_as(),self.dir.inner_as())
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
    pub fn inner_try_into<B>(self) -> Result<Ray<B>, N::Error> where N:TryInto<B> {
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
#[derive(Copy, Clone, Debug,PartialEq,Eq)]
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


#[cfg(feature = "std")]
pub mod foo{
    use super::*;
    use roots;
    use roots::*;
    impl<N: num_traits::float::FloatCore + roots::FloatType> Ray<N> {
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
}



impl<N: num_traits::Num + num_traits::Signed + PartialOrd + Copy  + core::fmt::Debug> Ray<N> {
    
    //if axis is x, then the line is top to bottom
    //if axis is y, then the line is left to right
    pub fn cast_to_aaline<A:Axis>(&self,a:A,line:N)->CastResult<N>{
        let ray=self;
        let  tval=if a.is_xaxis(){
            if ray.dir.x==N::zero(){
                return CastResult::NoHit;
            }
            (line-ray.point.x)/ray.dir.x
        }else{
            if ray.dir.y==N::zero(){
                return CastResult::NoHit;
            }
            (line-ray.point.y)/ray.dir.y
        };

        if tval>N::zero() {
            CastResult::Hit(tval)
        }else{
            CastResult::NoHit
        }
    }    



    fn prune_rect_axis<A:Axis>(&self,tval:N,rect:&Rect<N>,axis:A)->CastResult<N>{
        use CastResult::*;
        
        if axis.is_xaxis(){
            let xx=self.point.x+self.dir.x*tval;
            if rect.x.contains(xx){
                Hit(tval)
            }else{
                NoHit
            }
        }else{
            let yy=self.point.y+self.dir.y*tval;
            if rect.y.contains(yy){
                Hit(tval)
            }else{
                NoHit
            }
        }
        
    }
    pub fn cast_to_rect(&self,rect:&Rect<N>)->CastResult<N>{
        
        if rect.contains_point(self.point){
            return CastResult::Hit(N::zero())
        }
        /*
        https://gamedev.stackexchange.com/questions/18436/most-efficient-aabb-vs-ray-collision-algorithms
        Nobody described the algorithm here, but the Graphics Gems algorithm is simply:
        Using your ray's direction vector, determine which 3 of the 6 candidate planes would be hit first. If your (unnormalized) ray direction vector is (-1, 1, -1), then the 3 planes that are possible to be hit are +x, -y, and +z.
        Of the 3 candidate planes, do find the t-value for the intersection for each. Accept the plane that gets the largest t value as being the plane that got hit, and check that the hit is within the box. The diagram in the text makes this clear:
        */
        let &Rect{x:Range{start:startx,end:endx},y:Range{start:starty,end:endy}}=rect;

        let x=if self.dir.x>=N::zero(){
            startx
        }else{
            endx
        };

        let y=if self.dir.y>=N::zero(){
            starty
        }else{
            endy
        };

        let tval1=self.cast_to_aaline(XAXIS,x);
        let tval2=self.cast_to_aaline(YAXIS,y);

        use CastResult::*;
        match (tval1,tval2){
            (Hit(a),Hit(b))=>{
                //xaxis hit
                if a>b{
                    self.prune_rect_axis(a,rect,YAXIS)
                }else{
                    self.prune_rect_axis(b,rect,XAXIS)
                }
            },
            (Hit(a),NoHit)=>{
                self.prune_rect_axis(a,rect,YAXIS)
            },
            (NoHit,Hit(b))=>{
                self.prune_rect_axis(b,rect,XAXIS)
            },
            (NoHit,NoHit)=>{
                NoHit
            }
        } 
    }

/*
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
*/
    
}
