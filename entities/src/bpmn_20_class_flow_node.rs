//! FlowNode
#![allow(unused_imports)]

/// Conversion of FlowNode (Class : FlowNode)
///
/// ```json
/// CMOFClass {
///     xmi_id: "FlowNode",
///     name: "FlowNode",
///     is_abstract: true,
///     super_class: Some(
///         "FlowElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowNode-outgoing",
///                 name: "outgoing",
///                 visibility: Public,
///                 simple_type: Some(
///                     "SequenceFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: true,
///                 is_abstract: None,
///                 is_derived: false,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_sourceRef_outgoing_flow",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowNode-incoming",
///                 name: "incoming",
///                 visibility: Public,
///                 simple_type: Some(
///                     "SequenceFlow",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
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
///                 association: "A_targetRef_incoming_flow",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowNode-lanes",
///                 name: "lanes",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Lane",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: false,
///                 is_composite: false,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: false,
///                 subsetted_property: None,
///                 owning_association: "",
///                 association: "A_flowNodeRefs_lanes",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct FlowNode {
    pub outgoing: Option<Vec<SequenceFlow>>,
    pub incoming: Option<Vec<SequenceFlow>>,
    pub lanes: Option<Vec<Lane>>,
}

