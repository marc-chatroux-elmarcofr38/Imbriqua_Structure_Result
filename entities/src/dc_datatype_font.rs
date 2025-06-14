//! Font
#![allow(unused_imports)]

/// Conversion of Font (DataType : Font)
#[derive(Debug, Clone)]
pub struct Font {
    pub name: Option<String>,
    pub size: Option<Real>,
    pub is_bold: Option<Boolean>,
    pub is_italic: Option<Boolean>,
    pub is_underline: Option<Boolean>,
    pub is_strike_through: Option<Boolean>,
}

