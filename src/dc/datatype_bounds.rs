//! Bounds
#![allow(unused_imports)]

use crate::dc::*;
use crate::Builder;

/// Conversion of Bounds (DataType : Bounds)
#[derive(Builder, Debug, Clone)]
pub struct Bounds {
    #[builder(setter(into), default = "0.0")]
    pub x: Real,
    #[builder(setter(into), default = "0.0")]
    pub y: Real,
    #[builder(setter(into))]
    pub width: Real,
    #[builder(setter(into))]
    pub height: Real,
}

