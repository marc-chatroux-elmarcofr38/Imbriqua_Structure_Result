//! human_performer
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of HumanPerformer (Class : HumanPerformer)
///
/// ```json
/// CMOFClass {
///     xmi_id: "HumanPerformer",
///     name: "HumanPerformer",
///     is_abstract: false,
///     super_class: Some(
///         "Performer",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct HumanPerformer<'a> {
}

