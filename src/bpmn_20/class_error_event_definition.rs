//! ErrorEventDefinition
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ErrorEventDefinition (Class : ErrorEventDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ErrorEventDefinition",
///     name: "ErrorEventDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "EventDefinition",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ErrorEventDefinition-errorRef",
///                 name: "errorRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Error",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_errorRef_errorEventDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct ErrorEventDefinition {
    #[builder(setter(into, strip_option), default)]
    pub error_ref: Option<Error>,
}

