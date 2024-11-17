//! dc
use derive_builder::Builder;

/// Conversion of Boolean (PrimitiveType : Boolean)
pub use std::primitive::bool as Boolean;

/// Conversion of Integer (PrimitiveType : Integer)
pub use std::primitive::u64 as Integer;

/// Conversion of Real (PrimitiveType : Real)
pub use std::primitive::f64 as Real;

/// Conversion of String (PrimitiveType : String)
pub use std::string::String as String;

/// Conversion of Font (DataType : Font)
#[derive(Builder, Debug)]
#[builder(build_fn(validate = "Self::validate"))]
pub struct Font {
    #[builder(setter(into, strip_option), default)]
    pub name: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub size: Option<Real>,
    #[builder(setter(into, strip_option), default)]
    pub is_bold: Option<Boolean>,
    #[builder(setter(into, strip_option), default)]
    pub is_italic: Option<Boolean>,
    #[builder(setter(into, strip_option), default)]
    pub is_underline: Option<Boolean>,
    #[builder(setter(into, strip_option), default)]
    pub is_strike_through: Option<Boolean>,
}

impl FontBuilder {
    fn validate(&self) -> Result<(), String> {
        // Rule :  non_negative_size - Specification { xmi_type: "cmof:OpaqueExpression", xmi_id: "Font-non_negative_size-_specification", language: "OCL", body: "size >=  0" }
        let input = self.size;
        if input.is_some() {
            if input.unwrap().is_some() {
                if !(input.unwrap().unwrap() >= 0.0) {
                    return Err("size less that 0".to_string());
                };
            }
        }

        return Ok(());
    }
}

/// Conversion of Point (DataType : Point)
#[derive(Builder, Debug)]
pub struct Point {
    #[builder(setter(into), default = "0.0")]
    pub x: Real,
    #[builder(setter(into), default = "0.0")]
    pub y: Real,
}


/// Conversion of Bounds (DataType : Bounds)
#[derive(Builder, Debug)]
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

