use vec::PRIMT;
use std;

#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Range{
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

    #[inline(always)]
    pub fn contains_rang(&self, val: &Range) -> bool {
        self.contains(val.start) && self.contains(val.end)
    }

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

    #[inline(always)]
    pub fn intersects(&self, val: &Range) -> bool {
        self.contains(val.start) || val.contains(self.start)

    }
}
