//! ComplexBehaviorDefinition
#![allow(unused_imports)]

use crate::bpmn_20::*;
use crate::Builder;

/// Conversion of ComplexBehaviorDefinition (Class : ComplexBehaviorDefinition)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ComplexBehaviorDefinition",
///     name: "ComplexBehaviorDefinition",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ComplexBehaviorDefinition-condition",
///                 name: "condition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FormalExpression",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
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
///                 association: "A_condition_complexBehaviorDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ComplexBehaviorDefinition-event",
///                 name: "event",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ImplicitThrowEvent",
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
///                 association: "A_event_complexBehaviorDefinition",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Builder, Debug, Clone)]
pub struct ComplexBehaviorDefinition {
    #[builder(setter(into))]
    pub condition: FormalExpression,
    #[builder(setter(into, strip_option), default)]
    pub event: Option<ImplicitThrowEvent>,
}

