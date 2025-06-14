//! ChoreographyTask
#![allow(unused_imports)]

/// Conversion of ChoreographyTask (Class : ChoreographyTask)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ChoreographyTask",
///     name: "ChoreographyTask",
///     is_abstract: false,
///     super_class: Some(
///         "ChoreographyActivity",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ChoreographyTask-messageFlowRef",
///                 name: "messageFlowRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "MessageFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
///                 upper: Finite(
///                     2,
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
///                 association: "A_messageFlowRef_choreographyTask",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct ChoreographyTask {
    pub message_flow_ref: Vec<MessageFlow>,
}

