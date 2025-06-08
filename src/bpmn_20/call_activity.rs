//! call_activity

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of CallActivity (Class : CallActivity)
///
/// ```json
/// CMOFClass {
///     xmi_id: "CallActivity",
///     name: "CallActivity",
///     is_abstract: false,
///     super_class: Some(
///         "Activity",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "CallActivity-calledElementRef",
///                 name: "calledElementRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "CallableElement",
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
///                 association: "A_calledElementRef_callActivity",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct CallActivity<'a> {
    #[builder(setter(into, strip_option), default)]
    pub called_element_ref: Option<&'a CallableElement<'a>>,
}

