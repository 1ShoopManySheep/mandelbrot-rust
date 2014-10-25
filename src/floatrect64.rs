#[deriving(Clone, PartialEq, PartialOrd, Show)]
pub struct FloatRect {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64
}

impl FloatRect {
    pub fn new(left: f64, top: f64, width: f64, height: f64) -> FloatRect {
        FloatRect{left: left, top: top, width: width, height: height}
    }
}

