//! PotentialOwner
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of PotentialOwner (Class : PotentialOwner)
///
/// ```json
/// CMOFClass {
///     xmi_id: "PotentialOwner",
///     name: "PotentialOwner",
///     is_abstract: false,
///     super_class: Some(
///         "HumanPerformer",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct PotentialOwner {
}

