//! artifact

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of Artifact (Class : Artifact)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Artifact",
///     name: "Artifact",
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
pub struct Artifact<'a> {
}

