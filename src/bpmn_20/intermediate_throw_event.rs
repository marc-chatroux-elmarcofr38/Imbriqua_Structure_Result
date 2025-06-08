//! intermediate_throw_event

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of IntermediateThrowEvent (Class : IntermediateThrowEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "IntermediateThrowEvent",
///     name: "IntermediateThrowEvent",
///     is_abstract: false,
///     super_class: Some(
///         "ThrowEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct IntermediateThrowEvent<'a> {
}

