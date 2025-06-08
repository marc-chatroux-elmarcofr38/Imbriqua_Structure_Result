//! performer

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of Performer (Class : Performer)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Performer",
///     name: "Performer",
///     is_abstract: false,
///     super_class: Some(
///         "ResourceRole",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct Performer<'a> {
}

