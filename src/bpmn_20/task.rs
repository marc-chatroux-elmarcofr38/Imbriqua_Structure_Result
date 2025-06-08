//! task

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of Task (Class : Task)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Task",
///     name: "Task",
///     is_abstract: false,
///     super_class: Some(
///         "Activity InteractionNode",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct Task<'a> {
}

