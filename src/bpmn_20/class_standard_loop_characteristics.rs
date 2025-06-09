//! StandardLoopCharacteristics
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of StandardLoopCharacteristics (Class : StandardLoopCharacteristics)
///
/// ```json
/// CMOFClass {
///     xmi_id: "StandardLoopCharacteristics",
///     name: "StandardLoopCharacteristics",
///     is_abstract: false,
///     super_class: Some(
///         "LoopCharacteristics",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "StandardLoopCharacteristics-testBefore",
///                 name: "testBefore",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     PrimitiveTypeLink(
///                         PrimitiveTypeLink {
///                             href: "DC.cmof#Boolean",
///                         },
///                     ),
///                 ),
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     1,
///                 ),
///                 default: Some(
///                     "false",
///                 ),
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "StandardLoopCharacteristics-loopCondition",
///                 name: "loopCondition",
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
///                 association: "A_loopCondition_standardLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "StandardLoopCharacteristics-loopMaximum",
///                 name: "loopMaximum",
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
///                 association: "A_loopMaximum_standardLoopCharacteristics",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct StandardLoopCharacteristics {
    #[builder(setter(into), default = "false")]
    pub test_before: dc::Boolean,
    #[builder(setter(into, strip_option), default)]
    pub loop_condition: Option<Expression>,
    #[builder(setter(into, strip_option), default)]
    pub loop_maximum: Option<Expression>,
}

