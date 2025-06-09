//! DataOutputAssociation
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of DataOutputAssociation (Class : DataOutputAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataOutputAssociation",
///     name: "DataOutputAssociation",
///     is_abstract: false,
///     super_class: Some(
///         "DataAssociation",
///     ),
///     super_class_link: None,
///     owned_attribute: [],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct DataOutputAssociation {
}

