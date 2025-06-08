//! cancel_event_definition

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of CancelEventDefinition (Class : CancelEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CancelEventDefinition",
///     name: "CancelEventDefinition",
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
pub struct CancelEventDefinition<'a> {
}

