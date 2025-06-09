//! EndEvent
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of EndEvent (Class : EndEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EndEvent",
///     name: "EndEvent",
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
pub struct EndEvent {
}

