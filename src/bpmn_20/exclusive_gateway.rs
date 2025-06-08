//! exclusive_gateway
#[allow(unused)]
#[allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ExclusiveGateway (Class : ExclusiveGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ExclusiveGateway",
///     name: "ExclusiveGateway",
///     is_abstract: false,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExclusiveGateway-default",
///                 name: "default",
///                 visibility: Public,
///                 simple_type: Some(
///                     "SequenceFlow",
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
///                 association: "A_default_exclusiveGateway",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct ExclusiveGateway<'a> {
    #[builder(setter(into, strip_option), default)]
    pub default: Option<&'a SequenceFlow<'a>>,
}

