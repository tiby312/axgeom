use crate::Axis;
use core::convert::TryInto;
use core::ops::*;

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

impl<S: Add<Output = S> + Sub<Output = S> + PartialOrd + Copy> Vec2<S> {
    #[inline(always)]
    pub fn manhattan_dis(&self, other: Vec2<S>) -> S {
        absdiff(self.x, other.x) + absdiff(self.y, other.y)
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
    ///If the point is outside the rectangle, returns the squared distance from a point to a rectangle.
    ///If the point is inside the rectangle, it will return None.
    #[inline(always)]
    #[must_use]
    pub fn distance_squared_to_point(&self, point: Vec2<T>) -> T {
        (point.x - self.x) * (point.x - self.x) + (point.y - self.y) * (point.y - self.y)
    }
}

#[test]
fn test_rotate() {
    let b = vec2(1.0f64, 1.0).rotate_90deg_right();
    assert_eq!(b, vec2(-1.0, 1.0));

    let b = vec2(1.0f64, 0.0).rotate_90deg_right();
    assert_eq!(b, vec2(0.0, 1.0));
}

macro_rules! impl_float {
    ( $x:ty ) => {
        impl Vec2<$x> {
            #[inline(always)]
            pub fn zero() -> Vec2<$x> {
                vec2(0.0, 0.0)
            }

            #[inline(always)]
            #[must_use]
            pub fn is_zero(&self) -> bool {
                self.x == 0.0 && self.y == 0.0
            }

            #[inline(always)]
            pub fn scale(&self, other: Vec2<$x>) -> Vec2<$x> {
                vec2(self.x * other.x, self.y * other.y)
            }

            #[inline(always)]
            pub fn inv_scale(&self, other: Vec2<$x>) -> Vec2<$x> {
                vec2(self.x / other.x, self.y / other.y)
            }

            #[inline(always)]
            #[must_use]
            pub fn magnitude2(&self) -> $x {
                self.x * self.x + self.y * self.y
            }
            #[inline(always)]
            #[must_use]
            pub fn dot(&self, other: Vec2<$x>) -> $x {
                self.x * other.x + self.y * other.y
            }

            #[inline(always)]
            pub fn is_nan(&self) -> bool {
                self.x.is_nan() || self.y.is_nan()
            }
            #[inline(always)]
            pub fn abs(&self) -> Vec2<$x> {
                vec2(self.x.abs(), self.y.abs())
            }
            #[inline(always)]
            pub fn rotate_90deg_right(&self) -> Vec2<$x> {
                vec2(-self.y, self.x)
            }
            #[inline(always)]
            pub fn rotate_90deg_left(&self) -> Vec2<$x> {
                vec2(self.y, self.x)
            }

            #[inline(always)]
            pub fn split_into_components(&self) -> [Vec2<$x>; 2] {
                [vec2(self.x, 0.0), vec2(0.0, self.y)]
            }

            #[inline(always)]
            pub fn truncate_at(&self, mag: $x) -> Vec2<$x> {
                if self.magnitude() > mag {
                    self.normalize_to(mag)
                } else {
                    *self
                }
            }

            #[inline(always)]
            pub fn normalize_to(&self, mag: $x) -> Vec2<$x> {
                let l = self.magnitude2().sqrt();
                (*self) * (mag / l)
            }

            #[inline(always)]
            pub fn magnitude(&self) -> $x {
                self.magnitude2().sqrt()
            }
        }
    };
}

impl_float!(f32);
impl_float!(f64);

// ///Cast an array of 2 elements of primitive type to another primitive type using "as" on each element.
// #[must_use]
// pub fn arr2_as<B: 'static + Copy, A: num_traits::AsPrimitive<B>>(a: [A; 2]) -> [B; 2] {
//     let [a, b] = a;
//     [a.as_(), b.as_()]
// }

// impl<T> Vec2<T> {
//     #[inline(always)]
//     pub fn inner_as<B: 'static + Copy>(self) -> Vec2<B>
//     where
//         T: num_traits::AsPrimitive<B>,
//     {
//         vec2(self.x.as_(), self.y.as_())
//     }
// }

impl<'a, B: Copy> From<&'a [B; 2]> for Vec2<B> {
    fn from(a: &'a [B; 2]) -> Self {
        let [a, b] = *a;
        vec2(a, b)
    }
}

impl<B> From<[B; 2]> for Vec2<B> {
    fn from(a: [B; 2]) -> Self {
        let [a, b] = a;
        vec2(a, b)
    }
}

impl<B> From<Vec2<B>> for [B; 2] {
    fn from(a: Vec2<B>) -> Self {
        [a.x, a.y]
    }
}

impl<B> Vec2<B> {
    ///Get the range of one axis.
    #[inline(always)]
    #[must_use]
    pub fn get_axis(&self, axis: impl Axis) -> &B {
        if axis.is_xaxis() {
            &self.x
        } else {
            &self.y
        }
    }

    ///Get the mutable range of one axis.
    #[inline(always)]
    #[must_use]
    pub fn get_axis_mut(&mut self, axis: impl Axis) -> &mut B {
        if axis.is_xaxis() {
            &mut self.x
        } else {
            &mut self.y
        }
    }

    #[inline(always)]
    pub fn inner_into<A>(self) -> Vec2<A>
    where
        B: Into<A>,
    {
        let x = self.x.into();
        let y = self.y.into();
        vec2(x, y)
    }

    #[inline(always)]
    pub fn inner_try_into<A>(self) -> Result<Vec2<A>, B::Error>
    where
        B: TryInto<A>,
    {
        let x = self.x.try_into();
        let y = self.y.try_into();
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
