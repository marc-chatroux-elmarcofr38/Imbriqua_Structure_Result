//! Style
#![allow(unused_imports)]

use crate::di::*;
use crate::Builder;

/// Conversion of Style (Class : Style)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Style",
///     name: "Style",
///     is_abstract: true,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct Style {
}

