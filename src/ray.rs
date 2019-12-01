use crate::*;
use core::convert::TryFrom;
use core::cmp::Ordering;

///A Ray.
#[derive(Debug, Copy, Clone)]
pub struct Ray<N> {
    pub point: Vec2<N>,
    pub dir: Vec2<N>,
}

impl<N> Ray<N>{
    

    #[inline(always)]
    pub fn inner_into<B:From<N>>(self)->Ray<B>{
        let point=self.point.inner_into();
        let dir=self.dir.inner_into();
        Ray{point,dir}
    }
    #[inline(always)]
    pub fn inner_try_into<B:TryFrom<N>>(self)->Result<Ray<B>,B::Error>{
        let point=self.point.inner_try_into();
        let dir=self.dir.inner_try_into();
        match(point,dir){
            (Ok(point),Ok(dir))=>{
                Ok(Ray{point,dir})
            },
            (Err(e),Ok(_))=>{
                Err(e)
            },
            (Ok(_),Err(e))=>{
                Err(e)
            },
            (Err(e),Err(_))=>{
                Err(e)
            }
        }
    }
}

impl<N:PartialOrd + Copy> Ray<N>{
    pub fn range_side(&self,axis:impl Axis,range:&Range<N>)->Ordering{
        
        let v=if axis.is_xaxis(){
            self.point.x
        }else{
            self.point.y
        };

        range.contains_ext(v)
    }
}