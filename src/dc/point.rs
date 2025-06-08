//! point
#[allow(unused)]
#[allow(unused_imports)]

use crate::dc::*;
use crate::Builder;

/// Conversion of Point (DataType : Point)
#[derive(Builder, Debug, Clone)]
pub struct Point {
    #[builder(setter(into), default = "0.0")]
    pub x: Real,
    #[builder(setter(into), default = "0.0")]
    pub y: Real,
}

