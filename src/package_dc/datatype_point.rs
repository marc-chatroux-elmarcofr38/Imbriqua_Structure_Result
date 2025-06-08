//! point
#![allow(unused_imports)]

use crate::package_dc::*;
use crate::Builder;

/// Conversion of Point (DataType : Point)
#[derive(Builder, Debug, Clone)]
pub struct Point {
    #[builder(setter(into), default = "0.0")]
    pub x: Real,
    #[builder(setter(into), default = "0.0")]
    pub y: Real,
}

