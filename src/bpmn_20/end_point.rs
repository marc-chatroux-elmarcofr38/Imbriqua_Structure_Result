//! end_point
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of EndPoint (Class : EndPoint)
///
/// ```json
/// CMOFClass {
///     xmi_id: "EndPoint",
///     name: "EndPoint",
///     is_abstract: false,
///     super_class: Some(
///         "RootElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct EndPoint<'a> {
}

