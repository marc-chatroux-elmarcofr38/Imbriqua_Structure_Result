//! Group
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of Group (Class : Group)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Group",
///     name: "Group",
///     is_abstract: false,
///     super_class: Some(
///         "Artifact",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Group-categoryValueRef",
///                 name: "categoryValueRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CategoryValue",
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
///                 association: "A_categoryValueRef_categoryValueRef",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct Group {
    #[builder(setter(into, strip_option), default)]
    pub category_value_ref: Option<CategoryValue>,
}

