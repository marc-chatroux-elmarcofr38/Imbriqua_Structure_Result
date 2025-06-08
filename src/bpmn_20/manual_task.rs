//! manual_task
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ManualTask (Class : ManualTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ManualTask",
///     name: "ManualTask",
///     is_abstract: false,
///     super_class: Some(
///         "Task",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct ManualTask<'a> {
}

