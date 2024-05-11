//! dc

/// Conversion of Boolean (PrimitiveType : Boolean)
pub use std::primitive::bool as Boolean;

/// Conversion of Integer (PrimitiveType : Integer)
pub use std::primitive::i64 as Integer;

/// Conversion of Real (PrimitiveType : Real)
pub use std::primitive::f64 as Real;

/// Conversion of String (PrimitiveType : String)
pub use std::string::String as String;

/// Conversion of Font (DataType : Font)
pub struct Font {
    pub name : String,
    pub size : Real,
    pub is_bold : Boolean,
    pub is_italic : Boolean,
    pub is_underline : Boolean,
    pub is_strike_through : Boolean,
}

/// Conversion of Point (DataType : Point)
pub struct Point {
    pub x : Real,
    pub y : Real,
}

/// Conversion of Bounds (DataType : Bounds)
pub struct Bounds {
    pub x : Real,
    pub y : Real,
    pub width : Real,
    pub height : Real,
}
