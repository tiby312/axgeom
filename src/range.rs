
use std;
use std::fmt::Debug;

///A 1d range. Internally represented as start and end. (not start and length)
///This means that subdivision does not result in any floating point calculations.
///There is no protection against "degenerate" Ranges where start>end.
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Range<T:Copy+Debug>{
    pub start:T,
    pub end:T
}
impl<T:Copy+Debug+Ord> Range<T>{

    ///If the pos is to the left of the range, return less.
    ///If the pos is to the right of the range, return greater.
    ///If the pos intersects with the range, return equal.
    #[inline(always)]
    pub fn left_or_right_or_contain(&self,pos:&T)->std::cmp::Ordering{
        
        if *pos<self.start{
            return std::cmp::Ordering::Less
        }else if *pos>self.end{
            return std::cmp::Ordering::Greater
        }else{
            return std::cmp::Ordering::Equal
        }
    }

    #[inline(always)]
    pub fn grow_to_fit(&mut self,b:&Range<T>){
        
            let a=self;
            
            if b.start<a.start{
                a.start=b.start;
            }
            if b.end>a.end{
                a.end=b.end;
            }
        
    }


    ///Returns true if the point is inside of the range or on top of.
    #[inline(always)]
    pub fn contains(&self, pos: T) -> bool {
        pos>=self.start&&pos<=self.end
    }

    ///Returns true if self contains the specified range.
    #[inline(always)]
    pub fn contains_range(&self, val: &Range<T>) -> bool {
        self.contains(val.start) && self.contains(val.end)
    }

    ///Creates a range that represents the intersection range.
    #[inline(always)]
    pub fn get_intersection(&self,val:&Range<T>)->Option<Range<T>>{
  
        let a=self.start.max(val.start);
        let b=self.end.min(val.end);
        if a>b{
            None
        }else{
            Some(Range{start:a,end:b})
        }
    }

    ///Returns true if two ranges intersect.
    #[inline(always)]
    pub fn intersects(&self, val: &Range<T>) -> bool {
        self.contains(val.start) || val.contains(self.start)
    }
}

impl Range<f32>{
    
    #[inline(always)]
    pub fn len(&self)->f32{
        self.end-self.start
    }

    ///Grow in both ends by the specified value.
    #[inline(always)]
    pub fn grow(&mut self,val:f32)->&mut Range<f32>{
        self.start-=val;
        self.end+=val;
        self
    }
    
    #[inline(always)]
    pub fn midpoint(&self)->f32{
        self.start+ self.len()/2.0
    }
}

impl<T:Copy+Debug> Range<T>{
    #[inline(always)]
    pub fn left(&self)->T{
        self.start
    }
    
    #[inline(always)]
    pub fn right(&self)->T{
        self.end
    }
}


impl<T:PartialEq+Copy+Debug> std::cmp::PartialEq for Range<T> {

    #[inline(always)]
    fn eq(&self, other: &Range<T>) -> bool {
        self.left() == other.left() && self.right()==other.right()
    }
}
