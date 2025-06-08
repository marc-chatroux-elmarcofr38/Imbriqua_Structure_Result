//! complex_gateway

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ComplexGateway (Class : ComplexGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ComplexGateway",
///     name: "ComplexGateway",
///     is_abstract: false,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ComplexGateway-activationCondition",
///                 name: "activationCondition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Expression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Finite(
///                     1,
///                 ),
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
///                 association: "A_activationCondition_complexGateway",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ComplexGateway-default",
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
///                 association: "A_default_complexGateway",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct ComplexGateway<'a> {
    #[builder(setter(into, strip_option), default)]
    pub activation_condition: Option<&'a Expression<'a>>,
    #[builder(setter(into, strip_option), default)]
    pub default: Option<&'a SequenceFlow<'a>>,
}

