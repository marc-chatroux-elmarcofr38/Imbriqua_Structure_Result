//! monitoring
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of Monitoring (Class : Monitoring)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Monitoring",
///     name: "Monitoring",
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
pub struct Monitoring<'a> {
}

