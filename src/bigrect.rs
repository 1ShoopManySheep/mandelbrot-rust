use num::{BigRational};

#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub struct FloatRect {
    pub left: BigRational,
    pub top: BigRational,
    pub width: BigRational,
    pub height: BigRational
}

impl FloatRect {
    pub fn new(left: BigRational, top: BigRational, width: BigRational, height: BigRational) -> FloatRect {
        FloatRect{left: left, top: top, width: width, height: height}
    }
}