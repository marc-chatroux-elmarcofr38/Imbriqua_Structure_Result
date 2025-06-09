//! GlobalTask
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of GlobalTask (Class : GlobalTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "GlobalTask",
///     name: "GlobalTask",
///     is_abstract: false,
///     super_class: Some(
///         "CallableElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "GlobalTask-resources",
///                 name: "resources",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ResourceRole",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_resources_globalTask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct GlobalTask {
    #[builder(setter(into, strip_option), default)]
    pub resources: Option<Vec<ResourceRole>>,
}

