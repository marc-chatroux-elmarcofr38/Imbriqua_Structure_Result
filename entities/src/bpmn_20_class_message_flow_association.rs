//! MessageFlowAssociation
#![allow(unused_imports)]

/// Conversion of MessageFlowAssociation (Class : MessageFlowAssociation)
///
/// ```json
/// CMOFClass {
///     xmi_id: "MessageFlowAssociation",
///     name: "MessageFlowAssociation",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "MessageFlowAssociation-innerMessageFlowRef",
///                 name: "innerMessageFlowRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "MessageFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
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
///                 association: "A_innerMessageFlowRef_messageFlowAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "MessageFlowAssociation-outerMessageFlowRef",
///                 name: "outerMessageFlowRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "MessageFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
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
///                 association: "A_outerMessageFlowRef_messageFlowAssociation",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct MessageFlowAssociation {
    pub inner_message_flow_ref: MessageFlow,
    pub outer_message_flow_ref: MessageFlow,
}

