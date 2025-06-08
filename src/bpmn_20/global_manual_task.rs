//! global_manual_task
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of GlobalManualTask (Class : GlobalManualTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalManualTask",
///     name: "GlobalManualTask",
///     is_abstract: false,
///     super_class: Some(
///         "GlobalTask",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct GlobalManualTask<'a> {
}

