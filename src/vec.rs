use std;

///A typedef to a f32. //TODO get rid of this typedef. 
pub type PRIMT=f32;

pub const XAXIS: Axis = Axis(0);
pub const YAXIS: Axis = Axis(1);

///An Axis has a value of either X or Y.
///It is used to look up values in 2d containers.
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Axis(usize);

impl Axis {
    
    //Returns x axis if the num is even.
    //Returns y axis if the num is odd.
    #[inline(always)]
    pub fn from_num(num: usize) -> Axis {
        Axis(num % 2)
    }

    ///Returns the other axis.
    #[inline(always)]
    pub fn next(&self) -> Axis {
        Axis(1 - self.0)
    }
    

    #[inline(always)]
    pub fn get_axis_iter() -> AxisIter {
        AxisIter { val: 0 }
    }
}

impl std::cmp::PartialEq for Axis {

    #[inline(always)]
    fn eq(&self, other: &Axis) -> bool {
        self.0 == other.0
    }
}

///Iterator to iterate over the x and y axises.
pub struct AxisIter {
    val: usize,
}
impl std::iter::Iterator for AxisIter {
    type Item = Axis;

    #[inline(always)]
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

///A 2d generic container. Elements can be accessed using an Axis.
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct VecCont<T:Clone+Copy>{
    raw:[T;2]
}
impl<T:Clone+Copy> VecCont<T>{

    #[inline(always)]
    pub fn new(x: T, y: T) -> VecCont<T> {
        VecCont { raw: [x, y] }
    }

    #[inline(always)]
    pub fn get_axis_mut<'a>(&'a mut self, a: Axis) -> &'a mut T {
        unsafe { self.raw.get_unchecked_mut(a.0) }
    }

    #[inline(always)]
    pub fn get_axis(&self, a: Axis) -> &T {
        unsafe { self.raw.get_unchecked(a.0) }
    }
    
    #[inline(always)]
    pub fn x(&self) -> &T {
        unsafe { self.raw.get_unchecked(0) }
    }

    #[inline(always)]
    pub fn y(&self) -> &T {
        unsafe { self.raw.get_unchecked(1) }
    }

}


///A wrapper around a vec with the length and length sqr saved.
///Useful when you want to cache the length computation.
#[derive(Copy,Clone,Debug)]
pub struct ComputedVec2{
    vec:Vec2,
    len:PRIMT,
    len_sqr:PRIMT
}
impl ComputedVec2{

    //Calculate the length and store it into the new object.
    #[inline(always)]
    pub fn new(vec:Vec2)->ComputedVec2{
        let len_sqr=vec.len_sqr();
        let len=len_sqr.sqrt();
        ComputedVec2{vec:vec,len:len,len_sqr:len_sqr}
    }

    //Retrieve the cached length.
    #[inline(always)]
    pub fn len(&self) -> PRIMT {
        self.len
    }

    #[inline(always)]
    pub fn len_sqr(&self) -> PRIMT {
        self.len_sqr
    }

    ///Get a read only reference to the underlying vec.
    #[inline(always)]
    pub fn get_vec(&self)->&Vec2{
        &self.vec
    }
}

///A 2d point with a way to get the value on a particular axis easily.
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Vec2 {
    raw: [PRIMT; 2],
}
impl Vec2 {
    #[inline(always)]
    pub fn new(x: PRIMT, y: PRIMT) -> Vec2 {
        Vec2 { raw: [x, y] }
    }

    #[inline(always)]
    pub fn set(&mut self, x: PRIMT, y: PRIMT) {
        unsafe{
            *self.raw.get_unchecked_mut(0) = x;
            *self.raw.get_unchecked_mut(1) = y;
        }
    }

    #[inline(always)]
    pub fn zero(&mut self){
        self.raw=[0.0;2];
    }

    #[inline(always)]
    pub fn get_axis_mut<'a>(&'a mut self, a: Axis) -> &'a mut PRIMT {
        unsafe { self.raw.get_unchecked_mut(a.0) }
    }

    #[inline(always)]
    pub fn get_axis(&self, a: Axis) -> PRIMT {
        unsafe { *self.raw.get_unchecked(a.0) }
    }
    
    #[inline(always)]
    pub fn x(&self) -> PRIMT {
        unsafe { self.raw.get_unchecked(0).clone() }
    }

    #[inline(always)]
    pub fn y(&self) -> PRIMT {
        unsafe { self.raw.get_unchecked(1).clone() }
    }

    #[inline(always)]
    pub fn x_mut(&mut self) -> &mut PRIMT {
        unsafe { self.raw.get_unchecked_mut(0) }
    }

    #[inline(always)]
    pub fn y_mut(&mut self) -> &mut PRIMT {
        unsafe { self.raw.get_unchecked_mut(0) }
    }

    ///Calculates the dot product.
    #[inline(always)]
    pub fn inner_product(&self, b: &Vec2) -> PRIMT {
        self.x() * b.x() + self.y() * b.y()
    }

    ///Force the length of the vec to of max length nlen.
    ///If the length of the vec is zero, this will panic.
    #[inline(always)]
    pub fn truncate(&mut self, nlen: PRIMT) {
        if self.len_sqr()<nlen.powi(2){
            *self = *self / self.len();
            *self = *self * nlen;
        }
    }


    #[inline(always)]
    pub fn rotate90(&self) -> Vec2 {
        self.rotate_by(Vec2::new(0.0, 1.0))
    }

    #[inline(always)]
    pub fn rotate_by(&self, b: Vec2) -> Vec2 {
        Vec2::new(self.x() * b.x() - self.y() * b.y(),
                  self.x() * b.y() + self.y() * b.x())

    }

    ///Returns true if either element is nan.
    #[inline(always)]
    pub fn is_nan(&self) -> bool {
        self.x().is_nan()|self.y().is_nan()
    }

    ///Calculates len using sqrt().
    #[inline(always)]
    pub fn len(&self) -> PRIMT {
        self.len_sqr().sqrt()
    }

    #[inline(always)]
    pub fn len_sqr(&self) -> PRIMT {
        self.x()*self.x()+self.y()*self.y()
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 { raw: [self.raw[0] + other.raw[0], self.raw[1] + other.raw[1]] }
    }
}

impl std::ops::Mul<PRIMT> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, other: PRIMT) -> Vec2 {
        Vec2::new(self.raw[0] * other, self.raw[1] * other)
    }
}

impl std::ops::Div<PRIMT> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn div(self, other: PRIMT) -> Vec2 {
        Vec2::new(self.raw[0] / other, self.raw[1] / other)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn neg(self) -> Vec2 {
        Vec2::new(-self.raw[0], -self.raw[1])
    }
}

impl std::ops::MulAssign<PRIMT> for Vec2 {

    #[inline(always)]
    fn mul_assign(&mut self, rhs: PRIMT) {
        self.raw[0] *= rhs;
        self.raw[1] *= rhs;
    }
}

impl std::ops::AddAssign for Vec2 {

    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec2) {
        self.raw[0] += rhs.raw[0];
        self.raw[1] += rhs.raw[1];
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 { raw: [self.raw[0] - other.raw[0], self.raw[1] - other.raw[1]] }
    }
}
