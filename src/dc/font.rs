//! font
#[allow(unused)]
#[allow(unused_imports)]

use crate::dc::*;
use crate::Builder;

/// Conversion of Font (DataType : Font)
#[derive(Builder, Debug, Clone)]
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
    // Rule :  non_negative_size - OpaqueExpression(CMOFOpaqueExpression { xmi_id: "Font-non_negative_size-_specification", body: "size >=  0", language: "OCL" })
    pub fn non_negative_size(self) -> Result<(), String> {
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

    fn validate(&self) -> Result<(), String> {
        // Rule :  non_negative_size - OpaqueExpression(CMOFOpaqueExpression { xmi_id: "Font-non_negative_size-_specification", body: "size >=  0", language: "OCL" })
        &self.non_negative_size()?;

        return Ok(());
    }
}
