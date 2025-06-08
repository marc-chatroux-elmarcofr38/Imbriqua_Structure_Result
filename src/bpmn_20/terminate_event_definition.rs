//! terminate_event_definition
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of TerminateEventDefinition (Class : TerminateEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "TerminateEventDefinition",
///     name: "TerminateEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct TerminateEventDefinition<'a> {
}

