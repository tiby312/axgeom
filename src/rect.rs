use crate::range::Range;
use crate::vec2::vec2;
use crate::*;
use core::convert::TryInto;

use crate::vec2::*;

///Convenience function to create a Rect.
#[inline(always)]
pub fn rect<T>(xstart: T, xend: T, ystart: T, yend: T) -> Rect<T> {
    Rect::new(xstart, xend, ystart, yend)
}

///An axis aligned rectangle. Stored as two Ranges.
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[must_use]
pub struct Rect<T> {
    pub x: Range<T>,
    pub y: Range<T>,
}

impl<S> Rect<S> {
    #[inline(always)]
    pub fn inner_into<A>(self) -> Rect<A>
    where
        S: Into<A>,
    {
        let x = self.x.inner_into();
        let y = self.y.inner_into();

        Rect { x, y }
    }

    #[inline(always)]
    pub fn inner_try_into<A>(self) -> Result<Rect<A>, S::Error>
    where
        S: TryInto<A>,
    {
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

impl<B> From<[B; 4]> for Rect<B> {
    #[inline(always)]
    fn from(a: [B; 4]) -> Self {
        let [a, b, c, d] = a;
        Rect::new(a, b, c, d)
    }
}

impl<B> From<Rect<B>> for [B; 4] {
    #[inline(always)]
    fn from(a: Rect<B>) -> Self {
        [a.x.start, a.x.end, a.y.start, a.y.end]
    }
}

impl<B: Copy> From<&Rect<B>> for [B; 4] {
    #[inline(always)]
    fn from(a: &Rect<B>) -> Self {
        [a.x.start, a.x.end, a.y.start, a.y.end]
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
    ///Constructor.
    ///(xstart,xend) is the x component range.
    ///(ystart,yend) is the y component range.
    #[inline(always)]
    pub fn new(xstart: T, xend: T, ystart: T, yend: T) -> Rect<T> {
        Rect {
            x: Range {
                start: xstart,
                end: xend,
            },
            y: Range {
                start: ystart,
                end: yend,
            },
        }
    }
}

impl<T: Copy> Rect<T> {
    #[inline(always)]
    pub fn top_left(&self) -> Vec2<T> {
        vec2(self.x.start, self.y.start)
    }

    ///Returns each corner in this order:
    ///topleft
    ///topright
    ///bottomright
    ///bottomleft
    pub fn get_corners(&self) -> [Vec2<T>; 4] {
        [
            vec2(self.x.start, self.y.start),
            vec2(self.x.end, self.y.start),
            vec2(self.x.end, self.y.end),
            vec2(self.x.start, self.y.end),
        ]
    }

    // #[inline(always)]
    // pub fn inner_as<B: 'static + Copy>(&self) -> Rect<B>
    // where
    //     T: num_traits::AsPrimitive<B>,
    // {
    //     Rect {
    //         x: self.x.inner_as(),
    //         y: self.y.inner_as(),
    //     }
    // }

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
    pub fn grow(self, radius: T) -> Self {
        Rect {
            x: self.x.grow(radius),
            y: self.y.grow(radius),
        }
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
    ///If the point is outside the rectangle, returns the squared distance from a point to the furthest corner
    ///of the rectangle.
    #[inline(always)]
    pub fn furthest_distance_squared_to_point(&self, point: Vec2<T>) -> T {
        let (px, py) = (point.x, point.y);

        let ((a, b), (c, d)) = self.get();

        fn reverse_clamp<N: PartialOrd + core::ops::Sub<Output = N> + Copy>(
            px: N,
            a: N,
            b: N,
        ) -> N {
            let aa = px - a;
            let bb = b - px;
            if bb > aa {
                b
            } else {
                a
            }
        }
        let xx = reverse_clamp(px, a, b);
        let yy = reverse_clamp(py, c, d);

        (xx - px) * (xx - px) + (yy - py) * (yy - py)
    }
}

macro_rules! impl_float {
    ( $x:ty ) => {
        impl Rect<$x> {
            ///If the point is outside the rectangle, returns the squared distance from the closest corner of the rectangle.
            ///If the point is inside the rectangle, it will return None.
            #[inline(always)]
            pub fn distance_squared_to_point(&self, point: Vec2<$x>) -> Option<$x> {
                let (px, py) = (point.x, point.y);

                let ((a, b), (c, d)) = self.get();

                let xx = px.clamp(a, b);
                let yy = py.clamp(c, d);

                let dis = (xx - px) * (xx - px) + (yy - py) * (yy - py);

                //Then the point must be insert the rect.
                //In this case, lets return something negative.
                if xx > a && xx < b && yy > c && yy < d {
                    None
                } else {
                    Some(dis)
                }
            }

            #[inline(always)]
            pub fn derive_center(&self) -> Vec2<$x> {
                let two = 2.0;
                let ((a, b), (c, d)) = self.get();
                vec2(a + (b - a) / two, c + (d - c) / two)
            }
        }
    };
}
impl_float!(f32);
impl_float!(f64);

impl<T: PartialOrd + Copy> Rect<T> {
    ///Subdivides the rectangle.
    ///No floating point calculations are done.
    ///Important to note that a point that was in the original rectangle,
    ///could actually be inside both subdivded rectangles.
    ///This is because the ranges are inclusive on both sides `[start,end]`.
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

    #[inline(always)]
    pub fn grow_to_fit_point(&mut self, point: Vec2<T>) -> &mut Self {
        //TODO simplify using range.
        if point.x < self.x.start {
            self.x.start = point.x
        } else if self.x.end < point.x {
            self.x.end = point.x
        }
        if point.y < self.y.start {
            self.y.start = point.y
        } else if self.y.end < point.y {
            self.y.end = point.y
        }
        self
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

// impl<T: PartialOrd + Copy> Rect<T> {
//     ///Get an intersecting rectangle.
//     ///No floating point calculations as the new rectangle is made up of
//     ///values from this rectangle and the specified rectangle.
//     #[inline(always)]
//     pub fn get_intersect_rect(&self, other: &Rect<T>) -> Option<Rect<T>> {
//         macro_rules! macro_axis {
//             ($axis:ident) => {{
//                 let xr = other.get_range($axis);
//                 let xf = self.get_range($axis);

//                 let range = Range {
//                     start: partial_min_max::max(xr.start, xf.start),
//                     end: partial_min_max::min(xr.end, xf.end),
//                 };

//                 if range.end <= range.start {
//                     return None;
//                 }
//                 range
//             }};
//         }

//         let x = macro_axis!(XAXIS);
//         let y = macro_axis!(YAXIS);
//         Some(Rect { x, y })
//     }
// }
