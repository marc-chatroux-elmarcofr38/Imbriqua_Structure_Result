//! Lane
#![allow(unused_imports)]

/// Conversion of Lane (Class : Lane)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Lane",
///     name: "Lane",
///     is_abstract: false,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Lane-name",
///                 name: "name",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     PrimitiveTypeLink(
///                         PrimitiveTypeLink {
///                             href: "DC.cmof#String",
///                         },
///                     ),
///                 ),
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
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Lane-childLaneSet",
///                 name: "childLaneSet",
///                 visibility: Public,
///                 simple_type: Some(
///                     "LaneSet",
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
///                 association: "A_childLaneSet_parentLane",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Lane-partitionElementRef",
///                 name: "partitionElementRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BaseElement",
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
///                 association: "A_partitionElementRef_lane",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Lane-flowNodeRefs",
///                 name: "flowNodeRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FlowNode",
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
///                 association: "A_flowNodeRefs_lanes",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Lane-partitionElement",
///                 name: "partitionElement",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BaseElement",
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
///                 association: "A_partitionElement_lane",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct Lane {
    pub name: dc::String,
    pub child_lane_set: Option<LaneSet>,
    pub partition_element_ref: Option<BaseElement>,
    pub flow_node_refs: Option<Vec<FlowNode>>,
    pub partition_element: Option<BaseElement>,
}

