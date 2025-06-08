//! implicit_throw_event
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ImplicitThrowEvent (Class : ImplicitThrowEvent)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ImplicitThrowEvent",
///     name: "ImplicitThrowEvent",
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
pub struct ImplicitThrowEvent<'a> {
}

