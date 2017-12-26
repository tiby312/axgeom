use std;
///The x axis is internally represented as 0.
pub const XAXIS: Axis = Axis(0);

///The y axis internally represented as 1.
pub const YAXIS: Axis = Axis(1);

///An Axis has a value of either X or Y.
///It is used to look up values in 2d containers.
#[derive(Copy,Clone,Debug,PartialEq)]
#[must_use]
pub struct Axis(usize);

impl Axis {
    
    ///Returns x axis if the num is even.
    ///Returns y axis if the num is odd.
    #[inline(always)]
    pub fn from_num_even_xaxis(num: usize) -> Axis {
        Axis(num % 2)
    }

    //TODO remove this and move to axis iter.
    ///Returns the other axis.
    #[inline(always)]
    pub fn next(&self) -> Axis {
        Axis(1 - self.0)
    }
}


///Iterator to iterate over the x and y axises.
pub struct AxisIter {
    val: usize,
}
impl AxisIter{
    #[inline(always)]
    pub fn new() -> AxisIter {
        AxisIter { val: 0 }
    }
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
pub struct VecCont<T>{
    raw:[T;2]
}

impl<T> VecCont<T>{

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
    pub fn get(&self) -> (&T,&T) {
        unsafe { (self.raw.get_unchecked(0),self.raw.get_unchecked(1)) }
    }

    #[inline(always)]
    pub fn get_mut(&mut self) -> (&mut T,&mut T) {
        let k=&mut self.raw as *mut [T;2];
        unsafe { ((*k).get_unchecked_mut(0),(*k).get_unchecked_mut(1)) }
    }


}



///A wrapper around a Vec2 with the f32 length and length sqr saved.
///Useful when you want to cache the length computation.
#[derive(Copy,Clone,Debug)]
pub struct ComputedVec2{
    vec:Vec2,
    len:f32
}
impl ComputedVec2{

    //Calculate the length and store it into the new object.
    #[inline(always)]
    pub fn new(vec:Vec2)->ComputedVec2{
        let len_sqr=vec.len_sqr();
        let len=len_sqr.sqrt();
        ComputedVec2{vec:vec,len:len}
    }

    //Retrieve the cached length.
    #[inline(always)]
    pub fn len(&self) -> f32 {
        self.len
    }

    ///Get a read only reference to the underlying vec.
    #[inline(always)]
    pub fn get_vec(&self)->&Vec2{
        &self.vec
    }
}

///A 2d point made up of f32's with a way to get the value on a particular axis easily.
#[derive(Copy,Clone,Debug)]
#[must_use]
pub struct Vec2 {
    raw: VecCont<f32>
}
impl Vec2 {
    #[inline(always)]
    pub fn new(x: f32, y: f32) -> Vec2 {
        Vec2 { raw: VecCont::new(x,y) }
    }

    #[inline(always)]
    pub fn set(&mut self, x: f32, y: f32) {
        let a=self.raw.get_mut();
        *a.0=x;
        *a.1=y;
    }

    #[inline(always)]
    pub fn get_axis_mut<'a>(&'a mut self, a: Axis) -> &'a mut f32 {
        self.raw.get_axis_mut(a)
    }

    #[inline(always)]
    pub fn get_axis(&self, a: Axis) -> &f32 {
        self.raw.get_axis(a)
    }
    
    pub fn get(&self)->(&f32,&f32){
        self.raw.get()
    }

    pub fn get_mut(&mut self)->(&mut f32,&mut f32){
        self.raw.get_mut()
    }

    ///Calculates the dot product.
    #[inline(always)]
    pub fn inner_product(&self, b: &Vec2) -> f32 {
        let a=self.get();
        let b=b.get();
        a.0 * b.0 + a.1 * b.1
    }

    ///Force the length of the vec to of max length nlen.
    ///If the length of the vec is zero, this will panic.
    #[inline(always)]
    pub fn truncate(&mut self, nlen: f32) {
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
        let a=self.get();
        let b=b.get();
        Vec2::new(a.0 * b.0 - a.1 * b.1,
                  a.0 * b.1 + a.1 * b.0)

    }

    ///Calculates len using sqrt().
    #[inline(always)]
    pub fn len(&self) -> f32 {
        self.len_sqr().sqrt()
    }

    #[inline(always)]
    pub fn len_sqr(&self) -> f32 {
        let a=self.get();
        a.0*a.0+a.1*a.1
    }
}

impl std::ops::Add for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn add(self, other: Vec2) -> Vec2 {
        let a=self.get();
        let b=other.get();
        Vec2::new(a.0+b.0,a.1+b.1)
    }
}

impl std::ops::Mul<f32> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn mul(self, other: f32) -> Vec2 {
        let a=self.get();
        Vec2::new(a.0*other,a.1*other)
    }
}

impl std::ops::Div<f32> for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn div(self, other: f32) -> Vec2 {
        let a=self.get();
        Vec2::new(a.0 / other, a.1 / other)
    }
}

impl std::ops::Neg for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn neg(self) -> Vec2 {
        let a=self.get();
        Vec2::new(-a.0, -a.1)
    }
}

impl std::ops::MulAssign<f32> for Vec2 {

    #[inline(always)]
    fn mul_assign(&mut self, rhs: f32) {
        let a=self.get_mut();
        *a.0*=rhs;
        *a.1*=rhs;
    }
}

impl std::ops::AddAssign for Vec2 {

    #[inline(always)]
    fn add_assign(&mut self, rhs: Vec2) {
        let a=self.get_mut();
        let b=rhs.get();
        *a.0+=*b.0;
        *a.1+=*b.1;
    }
}

impl std::ops::Sub for Vec2 {
    type Output = Vec2;

    #[inline(always)]
    fn sub(self, other: Vec2) -> Vec2 {
        let a=self.get();
        let b=other.get();
        Vec2::new(a.0-b.0,a.1-b.1)
    }
}
