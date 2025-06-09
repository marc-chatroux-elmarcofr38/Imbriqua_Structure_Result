//! IntermediateCatchEvent
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of IntermediateCatchEvent (Class : IntermediateCatchEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "IntermediateCatchEvent",
///     name: "IntermediateCatchEvent",
///     is_abstract: false,
///     super_class: Some(
///         "CatchEvent",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct IntermediateCatchEvent {
}

