mod vec;
mod rect;

pub use self::vec::Vec2;
pub use self::vec::ComputedVec2;
pub use self::vec::VecCont;
pub use self::vec::Range2;
pub use self::vec::Axis;
pub use self::rect::RectAbsolute;
pub use self::vec::PRIMT;
pub use self::vec::XAXIS;
pub use self::vec::YAXIS;


#[derive(Copy,Clone,Debug)]
pub struct RadiusProp {
    radius: PRIMT,
    radius2: PRIMT,
    radius2_squared: PRIMT,
    radius_x_root_2_inv: PRIMT,
}
impl RadiusProp {
    pub fn create(radius: PRIMT) -> RadiusProp {
        let k = radius * 2.0;

        RadiusProp {
            radius: radius,
            radius2: k,
            radius2_squared: k.powi(2),
            radius_x_root_2_inv: radius * (1.0 / 1.4142),
        }
    }

    pub fn radius(&self) -> PRIMT {
        self.radius
    }
    pub fn radius2(&self) -> PRIMT {
        self.radius2
    }
    
    pub fn radius2_squared(&self) -> PRIMT {
        self.radius2_squared
    }
    pub fn radius_x_root_2_inv(&self) -> PRIMT {
        self.radius_x_root_2_inv
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
