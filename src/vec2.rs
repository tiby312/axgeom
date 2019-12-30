use crate::Axis;
use core::convert::TryFrom;
use core::ops::*;
use num_traits::float::Float;
use num_traits::Zero;
use ordered_float::NotNan;
use primitive_from::PrimitiveFrom;

///Convenience function to create a vector.
#[inline(always)]
pub const fn vec2<N>(x: N, y: N) -> Vec2<N> {
    Vec2 { x, y }
}

///Convenience function to create a vector where both component are the same.
#[inline(always)]
pub fn vec2same<N: Copy>(a: N) -> Vec2<N> {
    Vec2 { x: a, y: a }
}

impl<N: Float> AsRef<Vec2<N>> for Vec2<NotNan<N>> {
    #[inline(always)]
    fn as_ref(&self) -> &Vec2<N> {
        unsafe { &*((self as *const Self) as *const Vec2<N>) }
    }
}

///A 2D vector.
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
#[must_use]
pub struct Vec2<N> {
    pub x: N,
    pub y: N,
}

#[inline(always)]
pub fn absdiff<T>(x: T, y: T) -> T
where
    T: Sub<Output = T> + PartialOrd,
{
    if x < y {
        y - x
    } else {
        x - y
    }
}

fn gen_abs<S: Neg<Output = S> + PartialOrd + Zero>(num: S) -> S {
    if num < S::zero() {
        -num
    } else {
        num
    }
}
impl<S: Copy + Neg<Output = S> + PartialOrd + Zero> Vec2<S> {
    #[inline(always)]
    pub fn abs(&self) -> Vec2<S> {
        vec2(gen_abs(self.x), gen_abs(self.y))
    }
    #[inline(always)]
    pub fn rotate_90deg_right(&self) -> Vec2<S> {
        vec2(-self.y, self.x)
    }
    #[inline(always)]
    pub fn rotate_90deg_left(&self) -> Vec2<S> {
        vec2(self.y, self.x)
    }

    #[inline(always)]
    pub fn split_into_components(&self) -> [Vec2<S>; 2] {
        [vec2(self.x, S::zero()), vec2(S::zero(), self.y)]
    }
}

impl<S: Add<Output = S> + Sub<Output = S> + PartialOrd + Copy> Vec2<S> {
    #[inline(always)]
    pub fn manhattan_dis(&self, other: Vec2<S>) -> S {
        (absdiff(self.x, other.x) + absdiff(self.y, other.y))
    }
}

impl<
        T: Copy
            + PartialOrd
            + core::ops::Sub<Output = T>
            + core::ops::Mul<Output = T>
            + core::ops::Add<Output = T>,
    > Vec2<T>
{
    ///If the point is outisde the rectangle, returns the squared distance from a point to a rectangle.
    ///If the point is inside the rectangle, it will return None.
    #[inline(always)]
    pub fn distance_squared_to_point(&self, point: Vec2<T>) -> T {
        (point.x - self.x) * (point.x - self.x) + (point.y - self.y) * (point.y - self.y)
    }
}

#[test]
fn test_rotate() {
    let b = vec2(1, 1).rotate_90deg_right();
    assert_eq!(b, vec2(-1, 1));

    let b = vec2(1, 0).rotate_90deg_right();
    assert_eq!(b, vec2(0, 1));
}

impl<S: Mul<Output = S> + Div<Output = S> + Add<Output = S> + Copy> Vec2<S> {
    #[inline(always)]
    pub fn scale(&self, other: Vec2<S>) -> Vec2<S> {
        vec2(self.x * other.x, self.y * other.y)
    }

    #[inline(always)]
    pub fn inv_scale(&self, other: Vec2<S>) -> Vec2<S> {
        vec2(self.x / other.x, self.y / other.y)
    }

    #[inline(always)]
    pub fn magnitude2(&self) -> S {
        self.x * self.x + self.y * self.y
    }
    #[inline(always)]
    pub fn dot(&self, other: Vec2<S>) -> S {
        self.x * other.x + self.y * other.y
    }
}
impl<S: Float> Vec2<S> {
    #[inline(always)]
    pub fn truncate_at(&self, mag: S) -> Vec2<S> {
        if self.magnitude() > mag {
            self.normalize_to(mag)
        } else {
            *self
        }
    }

    #[inline(always)]
    pub fn normalize_to(&self, mag: S) -> Vec2<S> {
        let l = self.magnitude2().sqrt();
        (*self) * (mag / l)
    }

    #[inline(always)]
    pub fn magnitude(&self) -> S {
        self.magnitude2().sqrt()
    }
}

///Cast an array of 2 elements of primitive type to another primitive type using "as" on each element.
pub fn arr2_as<B: Copy, A: PrimitiveFrom<B>>(a: [B; 2]) -> [A; 2] {
    [PrimitiveFrom::from(a[0]), PrimitiveFrom::from(a[1])]
}

impl<B: Copy> Vec2<B> {
    pub fn inner_as<A: PrimitiveFrom<B>>(&self) -> Vec2<A> {
        vec2(PrimitiveFrom::from(self.x), PrimitiveFrom::from(self.y))
    }
}

impl<B> Vec2<B> {
    ///Get the range of one axis.
    #[inline(always)]
    pub fn get_axis(&self, axis: impl Axis) -> &B {
        if axis.is_xaxis() {
            &self.x
        } else {
            &self.y
        }
    }

    ///Get the mutable range of one axis.
    #[inline(always)]
    pub fn get_axis_mut(&mut self, axis: impl Axis) -> &mut B {
        if axis.is_xaxis() {
            &mut self.x
        } else {
            &mut self.y
        }
    }

    #[inline(always)]
    pub fn inner_into<A: From<B>>(self) -> Vec2<A> {
        let x = A::from(self.x);
        let y = A::from(self.y);
        vec2(x, y)
    }

    #[inline(always)]
    pub fn inner_try_into<A: TryFrom<B>>(self) -> Result<Vec2<A>, A::Error> {
        let x = A::try_from(self.x);
        let y = A::try_from(self.y);
        match (x, y) {
            (Ok(x), Ok(y)) => Ok(vec2(x, y)),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(e), Err(_)) => Err(e),
        }
    }
}

impl<S: Add<Output = S> + Copy> Add<Self> for Vec2<S> {
    type Output = Self;
    #[inline(always)]
    fn add(self, rhs: Self) -> Self {
        vec2(self.x + rhs.x, self.y + rhs.y)
    }
}

impl<S: Sub<Output = S> + Copy> Sub<Self> for Vec2<S> {
    type Output = Self;
    #[inline(always)]
    fn sub(self, rhs: Self) -> Self {
        vec2(self.x - rhs.x, self.y - rhs.y)
    }
}

impl<S: Mul<Output = S> + Copy> Mul<S> for Vec2<S> {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: S) -> Self {
        vec2(self.x * rhs, self.y * rhs)
    }
}

impl<S: Div<Output = S> + Copy> Div<S> for Vec2<S> {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: S) -> Self {
        vec2(self.x / rhs, self.y / rhs)
    }
}

impl<S: DivAssign<S> + Copy> DivAssign<S> for Vec2<S> {
    #[inline(always)]
    fn div_assign(&mut self, scalar: S) {
        self.x /= scalar;
        self.y /= scalar;
    }
}
impl<S: MulAssign<S> + Copy> MulAssign<S> for Vec2<S> {
    #[inline(always)]
    fn mul_assign(&mut self, scalar: S) {
        self.x *= scalar;
        self.y *= scalar;
    }
}

impl<S: AddAssign<S> + Copy> AddAssign<Self> for Vec2<S> {
    #[inline(always)]
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}
impl<S: SubAssign<S> + Copy> SubAssign<Self> for Vec2<S> {
    #[inline(always)]
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl<S: Neg<Output = S>> Neg for Vec2<S> {
    type Output = Vec2<S>;

    #[inline]
    fn neg(self) -> Vec2<S> {
        vec2(-self.x, -self.y)
    }
}

impl<S: Zero + Eq + Copy> Zero for Vec2<S> {
    #[inline(always)]
    fn zero() -> Vec2<S> {
        vec2(S::zero(), S::zero())
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        *self == Vec2::zero()
    }
}
