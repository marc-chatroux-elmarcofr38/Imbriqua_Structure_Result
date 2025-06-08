//! rendering

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of Rendering (Class : Rendering)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Rendering",
///     name: "Rendering",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct Rendering<'a> {
}

