use vec::PRIMT;
use std;


///A 1d range. Internally represented as start and end. (not start and length)
///This means that subdivision does not result in any floating point calculations.
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Range{
    //TODO make private
    pub start:PRIMT,
    pub end:PRIMT
}
impl Range{

    #[inline(always)]
    pub fn left(&self)->&PRIMT{
        &self.start
    }

    #[inline(always)]
    pub fn midpoint(&self)->PRIMT{
        self.start+ self.len()/2.0
    }
    
    #[inline(always)]
    pub fn right(&self)->&PRIMT{
        &self.end
    }

    ///If the pos is to the left of the range, return less.
    ///If the pos is to the right of the range, return greater.
    ///If the pos intersects with the range, return equal.
    #[inline(always)]
    pub fn left_or_right_or_contain(&self,pos:&PRIMT)->std::cmp::Ordering{
        
        if *pos<self.start{
            return std::cmp::Ordering::Less
        }else if *pos>self.end{
            return std::cmp::Ordering::Greater
        }else{
            return std::cmp::Ordering::Equal
        }
    }

    #[inline(always)]
    pub fn len(&self)->PRIMT{
        self.end-self.start
    }

    ///Grow in both ends by the specified value.
    #[inline(always)]
    pub fn grow(&mut self,val:PRIMT)->&mut Range{
        self.start-=val;
        self.end+=val;
        self
    }


    #[inline(always)]
    pub fn contains(&self, pos: PRIMT) -> bool {
        pos>=self.start&&pos<=self.end
    }

    ///Returns true if self contains the specified range.
    #[inline(always)]
    pub fn contains_rang(&self, val: &Range) -> bool {
        self.contains(val.start) && self.contains(val.end)
    }

    ///Creates a range that represents the intersection range.
    #[inline(always)]
    pub fn get_intersection(&self,val:&Range)->Option<Range>{
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
    pub fn intersects(&self, val: &Range) -> bool {
        self.contains(val.start) || val.contains(self.start)
    }
}
