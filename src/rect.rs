use crate::range::Range;
use crate::vec2::vec2;
use crate::*;
use core::convert::TryFrom;
use num_traits::Float;
use ordered_float::NotNan;
use primitive_from::PrimitiveFrom;

#[inline(always)]
pub fn rect<T>(a:T,b:T,c:T,d:T)->Rect<T>{
    Rect::new(a,b,c,d)
}

///An axis aligned rectangle. Stored as two Ranges.
///It is a semi-closed rectangle. A point is considered inside the rectangle if it is in [start,end) for both x and y.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[must_use]
pub struct Rect<T> {
    pub x: Range<T>,
    pub y: Range<T>,
}

impl<N: Float> AsRef<Rect<N>> for Rect<NotNan<N>> {
    #[inline(always)]
    fn as_ref(&self) -> &Rect<N> {
        unsafe { &*((self as *const Self) as *const Rect<N>) }
    }
}

impl<N: Float> AsMut<Rect<N>> for Rect<NotNan<N>> {
    #[inline(always)]
    fn as_mut(&mut self) -> &mut Rect<N> {
        unsafe { &mut *((self as *mut Self) as *mut Rect<N>) }
    }
}

impl<S: Copy> Rect<S> {
    #[inline(always)]
    pub fn inner_into<A: From<S>>(&self) -> Rect<A> {
        let x = self.x.inner_into();
        let y = self.y.inner_into();

        Rect { x, y }
    }

    #[inline(always)]
    pub fn inner_try_into<A: TryFrom<S>>(&self) -> Result<Rect<A>, A::Error> {
        let x = self.x.inner_try_into();
        let y = self.y.inner_try_into();
        match (x, y) {
            (Ok(x), Ok(y)) => Ok(Rect { x, y }),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(e1), Err(_)) => Err(e1),
        }
    }
}

impl<T: Copy + core::ops::Sub<Output = T> + core::ops::Add<Output = T>> Rect<T> {
    ///Create a rectangle from a point and radius.
    #[inline(always)]
    pub fn from_point(point: Vec2<T>, radius: Vec2<T>) -> Rect<T> {
        let x = Range::from_point(point.x, radius.x);
        let y = Range::from_point(point.y, radius.y);
        Rect { x, y }
    }
}

impl<T> Rect<T> {
    ///Get the range of one axis.
    #[inline(always)]
    pub fn get_range(&self, axis: impl Axis) -> &Range<T> {
        if axis.is_xaxis() {
            &self.x
        } else {
            &self.y
        }
    }

    ///Get the mutable range of one axis.
    #[inline(always)]
    pub fn get_range_mut(&mut self, axis: impl Axis) -> &mut Range<T> {
        if axis.is_xaxis() {
            &mut self.x
        } else {
            &mut self.y
        }
    }
}


impl<T> Rect<T> {
    ///(a,b) is the x component range.
    ///(c,d) is the y component range.
    #[inline(always)]
    pub fn new(a: T, b: T, c: T, d: T) -> Rect<T> {
        Rect { x: Range { start: a, end: b }, y: Range { start: c, end: d } }
    }
}

impl<T: Copy> Rect<T> {
    #[inline(always)]
    pub fn top_start(&self)->Vec2<T>{
        vec2(self.x.start,self.y.start)
    }

    #[inline(always)]
    pub fn inner_as<B: PrimitiveFrom<T>>(&self) -> Rect<B> {
        Rect {
            x: self.x.inner_as(),
            y: self.y.inner_as(),
        }
    }

    ///(a,b) is the x component range.
    ///(c,d) is the y component range.
    #[inline(always)]
    pub fn get(&self) -> ((T, T), (T, T)) {
        let f = self;
        ((f.x.start, f.x.end), (f.y.start, f.y.end))
    }
}

impl<T: PartialOrd + Copy> Rect<T> {
    ///Returns true if the point is contained in the the ranges of both axis.
    #[inline(always)]
    pub fn contains_point(&self, a: Vec2<T>) -> bool {
        self.x.contains(a.x) && self.y.contains(a.y)
    }
}

impl<T: Copy + core::ops::Sub<Output = T> + core::ops::Add<Output = T>> Rect<T> {
    ///Grow a rectangle of a radius.
    #[inline(always)]
    pub fn grow(&mut self, radius: T) -> &mut Self {
        self.x.grow(radius);
        self.y.grow(radius);
        self
    }
}

impl<
        T: Copy
            + PartialOrd
            + core::ops::Sub<Output = T>
            + core::ops::Mul<Output = T>
            + core::ops::Add<Output = T>,
    > Rect<T>
{
    ///If the point is outisde the rectangle, returns the squared distance from a point to a rectangle.
    ///If the point is inside the rectangle, it will return None.
    #[inline(always)]
    pub fn distance_squared_to_point(&self, point: Vec2<T>) -> Option<T> {
        let (px, py) = (point.x, point.y);

        let ((a, b), (c, d)) = self.get();

        let xx = num_traits::clamp(px, a, b);
        let yy = num_traits::clamp(py, c, d);

        let dis = (xx - px) * (xx - px) + (yy - py) * (yy - py);

        //Then the point must be insert the rect.
        //In this case, lets return something negative.
        if xx > a && xx < b && yy > c && yy < d {
            None
        } else {
            Some(dis)
        }
    }
}

impl<T: num_traits::Num + Copy> Rect<T> {
    #[inline(always)]
    pub fn derive_center(&self) -> Vec2<T> {
        let two = T::one() + T::one();
        let ((a, b), (c, d)) = self.get();
        vec2(a + (b - a) / two, c + (d - c) / two)
    }
}

impl<T: PartialOrd + Copy> Rect<T> {
    ///Subdivides the rectangle.
    ///No floating point calculations are done.
    ///Important to note that a point that was in the original rectangle,
    ///could actually be inside both subdivded rectangles.
    ///This is because the ranges are inclusive on both sides [start,end].
    #[inline(always)]
    pub fn subdivide<A: Axis>(&self, axis: A, divider: T) -> (Rect<T>, Rect<T>) {
        let ca = axis;
        let na = axis.next();

        let rel = self.get_range(ca);
        let carry_thru = *self.get_range(na);

        let (l, r) = rel.subdivide(divider);

        if axis.is_xaxis() {
            (
                Rect {
                    x: l,
                    y: carry_thru,
                },
                Rect {
                    x: r,
                    y: carry_thru,
                },
            )
        } else {
            (
                Rect {
                    x: carry_thru,
                    y: l,
                },
                Rect {
                    x: carry_thru,
                    y: r,
                },
            )
        }
    }

    ///Returns true if the rectangle's ranges are not degenerate.
    #[inline(always)]
    pub fn is_valid(&self) -> bool {
        self.x.is_valid() && self.y.is_valid()
    }

    ///Returns true if the specified rect is inside of this rect.
    #[inline(always)]
    pub fn contains_rect(&self, rect: &Rect<T>) -> bool {
        self.x.contains_range(&rect.x) && self.y.contains_range(&rect.y)
    }

    ///Grow the rectangle to fit the specified rectangle by replacing values
    ///with the specified rectangle. No floating point computations.
    #[inline(always)]
    pub fn grow_to_fit(&mut self, rect: &Rect<T>) -> &mut Self {
        {
            macro_rules! macro_axis {
                ($axis:ident) => {{
                    let sx = self.get_range_mut($axis);
                    let rx = rect.get_range($axis);
                    sx.grow_to_fit(rx);
                }};
            }

            macro_axis!(XAXIS);
            macro_axis!(YAXIS);
        }
        self
    }

    #[inline(always)]
    pub fn intersects_rect(&self, other: &Rect<T>) -> bool {
        other.x.intersects(&self.x) && other.y.intersects(&self.y)
    }
}

impl<T: Ord + Copy> Rect<T> {
    ///Get an intersecting rectangle.
    ///No floating point calculations as the new rectangle is made up of
    ///values from this rectangle and the specified rectangle.
    #[inline(always)]
    pub fn get_intersect_rect(&self, other: &Rect<T>) -> Option<Rect<T>> {
        macro_rules! macro_axis {
            ($axis:ident) => {{
                let xr = other.get_range($axis);
                let xf = self.get_range($axis);

                let range = Range {
                    start: xr.start.max(xf.start),
                    end: xr.end.min(xf.end),
                };

                //TODO figure out inequality
                if range.end < range.start {
                    return None;
                }
                range
            }};
        }

        let x = macro_axis!(XAXIS);
        let y = macro_axis!(YAXIS);
        Some(Rect { x, y })
    }
}
