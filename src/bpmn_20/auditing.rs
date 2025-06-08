//! auditing

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of Auditing (Class : Auditing)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Auditing",
///     name: "Auditing",
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
pub struct Auditing<'a> {
}

