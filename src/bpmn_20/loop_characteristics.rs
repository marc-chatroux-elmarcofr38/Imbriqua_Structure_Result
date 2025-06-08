//! loop_characteristics
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of LoopCharacteristics (Class : LoopCharacteristics)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LoopCharacteristics",
///     name: "LoopCharacteristics",
///     is_abstract: true,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct LoopCharacteristics<'a> {
}

