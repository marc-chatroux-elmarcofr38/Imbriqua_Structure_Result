//! parallel_gateway

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ParallelGateway (Class : ParallelGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ParallelGateway",
///     name: "ParallelGateway",
///     is_abstract: false,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct ParallelGateway<'a> {
}

