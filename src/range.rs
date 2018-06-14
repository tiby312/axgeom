
use std;
use std::fmt::Debug;


///A 1d range. Internally represented as start and end. (not start and length)
///This means that subdivision does not result in any floating point calculations.
///There is no protection against "degenerate" Ranges where start>end.
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
#[must_use]
pub struct Range<T:Copy+Debug>{
    pub left:T,
    pub right:T
}
impl<T:Copy+Debug+Ord> Range<T>{

    ///If the pos is to the left of the range, return less.
    ///If the pos is to the right of the range, return greater.
    ///If the pos intersects with the range, return equal.
    #[inline(always)]
    pub fn left_or_right_or_contain(&self,pos:&T)->std::cmp::Ordering{
        
        if *pos<self.left{
            return std::cmp::Ordering::Less
        }else if *pos>self.right{
            return std::cmp::Ordering::Greater
        }else{
            return std::cmp::Ordering::Equal
        }
    }

    #[inline(always)]
    pub fn grow_to_fit(&mut self,b:&Range<T>){
        
            let a=self;
            
            if b.left<a.left{
                a.left=b.left;
            }
            if b.right>a.right{
                a.right=b.right;
            }
        
    }


    ///Returns true if the point is inside of the range or on top of.
    #[inline(always)]
    pub fn contains(&self, pos: T) -> bool {
        pos>=self.left&&pos<=self.right
    }

    ///Returns true if self contains the specified range.
    #[inline(always)]
    pub fn contains_range(&self, val: &Range<T>) -> bool {
        self.contains(val.left) && self.contains(val.right)
    }

    ///Creates a range that represents the intersection range.
    #[inline(always)]
    pub fn get_intersection(&self,val:&Range<T>)->Option<Range<T>>{
  
        let a=self.left.max(val.left);
        let b=self.right.min(val.right);
        if a>b{
            None
        }else{
            Some(Range{left:a,right:b})
        }
    }

    ///Returns true if two ranges intersect.
    #[inline(always)]
    pub fn intersects(&self, val: &Range<T>) -> bool {
        //TODO double check this?
        self.contains(val.left) || val.contains(self.left)
    }
}


/*
impl<T:PartialEq+Copy+Debug> std::cmp::PartialEq for Range<T> {

    #[inline(always)]
    fn eq(&self, other: &Range<T>) -> bool {
        self.left() == other.left() && self.right()==other.right()
    }
}
*/