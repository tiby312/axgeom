use std;

pub type PRIMT=f32;

pub const XAXIS: Axis = Axis(0);
pub const YAXIS: Axis = Axis(1);

#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Axis(usize);

impl Axis {
    //todo this is a tree specific property. should not be here.
    pub fn from_depth(depth: usize) -> Axis {
        Axis(depth % 2)
    }
  
    pub fn next2(&self) -> Axis {
        Axis(1 - self.0)
    }
  
    pub fn get_axis_iter() -> AxisIter {
        AxisIter { val: 0 }
    }
}

impl std::cmp::PartialEq for Axis {
    fn eq(&self, other: &Axis) -> bool {
        self.0 == other.0
    }
}

pub struct AxisIter {
    val: usize,
}
impl std::iter::Iterator for AxisIter {
    type Item = Axis;
    fn next(&mut self) -> Option<Axis> {
        if self.val < 2 {
            let k = Some(Axis(self.val));
            self.val += 1;
            k
        } else {
            None
        }
    }
}

#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Range2{
    pub start:PRIMT,
    pub end:PRIMT
}
impl Range2{
    
    pub fn left(&self)->&PRIMT{
        &self.start
    }
    pub fn midpoint(&self)->PRIMT{
        self.start+ self.len()/2.0
    }

    pub fn right(&self)->&PRIMT{
        &self.end
    }
    pub fn left_or_right_or_contain(&self,pos:&PRIMT)->std::cmp::Ordering{
        
        if *pos<self.start{
            return std::cmp::Ordering::Less
        }else if *pos>self.end{
            return std::cmp::Ordering::Greater
        }else{
            return std::cmp::Ordering::Equal
        }
    }
    pub fn len(&self)->PRIMT{
        self.end-self.start
    }
    pub fn grow(&mut self,val:PRIMT)->&mut Range2{
        self.start-=val;
        self.end+=val;
        self
    }

    pub fn contains(&self, pos: PRIMT) -> bool {
        pos>=self.start&&pos<=self.end
    }

    pub fn contains_rang(&self, val: &Range2) -> bool {
        self.contains(val.start) && self.contains(val.end)
    }

    pub fn get_intersection(&self,val:&Range2)->Option<Range2>{
        let a=self.start.max(val.start);
        let b=self.end.min(val.end);
        if a>b{
            None
        }else{
            Some(Range2{start:a,end:b})
        }
    }
    pub fn intersects(&self, val: &Range2) -> bool {
        self.contains(val.start) || val.contains(self.start)

    }
}


#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct VecCont<T:Clone+Copy>{
    raw:[T;2]
}
impl<T:Clone+Copy> VecCont<T>{
    pub fn new(x: T, y: T) -> VecCont<T> {
        VecCont { raw: [x, y] }
    }
    pub fn get_axis_mut<'a>(&'a mut self, a: Axis) -> &'a mut T {
        unsafe { self.raw.get_unchecked_mut(a.0) }
    }
    pub fn get_axis(&self, a: Axis) -> &T {
        unsafe { self.raw.get_unchecked(a.0) }
    }
    
    pub fn x(&self) -> &T {
        unsafe { self.raw.get_unchecked(0) }
    }
    pub fn y(&self) -> &T {
        unsafe { self.raw.get_unchecked(1) }
    }

}


///A wrapper around a vec with the length and length sqr saved.
#[derive(Copy,Clone,Debug)]
pub struct ComputedVec2{
    vec:Vec2,
    len:PRIMT,
    len_sqr:PRIMT
}
impl ComputedVec2{
    pub fn new(vec:Vec2)->ComputedVec2{
        let len_sqr=vec.len_sqr();
        let len=len_sqr.sqrt();
        ComputedVec2{vec:vec,len:len,len_sqr:len_sqr}
    }
    pub fn len(&self) -> PRIMT {
        self.len
    }
    pub fn len_sqr(&self) -> PRIMT {
        self.len_sqr
    }
    pub fn get_vec(&self)->&Vec2{
        &self.vec
    }
}


#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Vec2 {
    raw: [PRIMT; 2],
}
impl Vec2 {
    pub fn new(x: PRIMT, y: PRIMT) -> Vec2 {
        Vec2 { raw: [x, y] }
    }

    
    pub fn set(&mut self, x: PRIMT, y: PRIMT) {
        unsafe{
            *self.raw.get_unchecked_mut(0) = x;
            *self.raw.get_unchecked_mut(1) = y;
        }
    }
    pub fn zero(&mut self){
        self.raw=[0.0;2];
    }

    pub fn get_axis_mut<'a>(&'a mut self, a: Axis) -> &'a mut PRIMT {
        unsafe { self.raw.get_unchecked_mut(a.0) }
    }
    pub fn get_axis(&self, a: Axis) -> PRIMT {
        unsafe { *self.raw.get_unchecked(a.0) }
    }
    
    pub fn x(&self) -> PRIMT {
        unsafe { self.raw.get_unchecked(0).clone() }
    }
    pub fn y(&self) -> PRIMT {
        unsafe { self.raw.get_unchecked(1).clone() }
    }
    pub fn x_mut(&mut self) -> &mut PRIMT {
        unsafe { self.raw.get_unchecked_mut(0) }
    }
    pub fn y_mut(&mut self) -> &mut PRIMT {
        unsafe { self.raw.get_unchecked_mut(0) }
    }

    pub fn inner_product(&self, b: Vec2) -> PRIMT {
        self.x() * b.x() + self.y() * b.y()
    }
    pub fn truncate(&mut self, nlen: PRIMT) {
        if self.len_sqr()<nlen.powi(2){
            *self = *self / self.len();
            *self = *self * nlen;
        }
    }


    pub fn rotate90(&self) -> Vec2 {
        self.rotate_by(Vec2::new(0.0, 1.0))
    }
    pub fn rotate_by(&self, b: Vec2) -> Vec2 {
        Vec2::new(self.x() * b.x() - self.y() * b.y(),
                  self.x() * b.y() + self.y() * b.x())

    }

    pub fn is_nan(&self) -> bool {
        self.x().is_nan()|self.y().is_nan()
    }

    pub fn len(&self) -> PRIMT {
        self.len_sqr().sqrt()
    }
    pub fn len_sqr(&self) -> PRIMT {
        self.x()*self.x()+self.y()*self.y()
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { raw: [self.raw[0] + other.raw[0], self.raw[1] + other.raw[1]] }
    }
}

impl std::ops::Mul<PRIMT> for Vec2 {
    type Output = Vec2;
    fn mul(self, other: PRIMT) -> Vec2 {
        Vec2::new(self.raw[0] * other, self.raw[1] * other)
    }
}

impl std::ops::Div<PRIMT> for Vec2 {
    type Output = Vec2;
    fn div(self, other: PRIMT) -> Vec2 {
        Vec2::new(self.raw[0] / other, self.raw[1] / other)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;
    fn neg(self) -> Vec2 {
        Vec2::new(-self.raw[0], -self.raw[1])
    }
}

impl std::ops::MulAssign<PRIMT> for Vec2 {
    fn mul_assign(&mut self, rhs: PRIMT) {
        self.raw[0] *= rhs;
        self.raw[1] *= rhs;
    }
}

impl std::ops::AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Vec2) {
        self.raw[0] += rhs.raw[0];
        self.raw[1] += rhs.raw[1];
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 { raw: [self.raw[0] - other.raw[0], self.raw[1] - other.raw[1]] }
    }
}
