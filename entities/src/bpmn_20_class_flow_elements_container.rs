//! FlowElementsContainer
#![allow(unused_imports)]

/// Conversion of FlowElementsContainer (Class : FlowElementsContainer)
///
/// ```json
/// CMOFClass {
///     xmi_id: "FlowElementsContainer",
///     name: "FlowElementsContainer",
///     is_abstract: true,
///     super_class: Some(
///         "BaseElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowElementsContainer-flowElements",
///                 name: "flowElements",
///                 visibility: Public,
///                 simple_type: Some(
///                     "FlowElement",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
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
///                 association: "A_flowElements_container",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "FlowElementsContainer-laneSets",
///                 name: "laneSets",
///                 visibility: Public,
///                 simple_type: Some(
///                     "LaneSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
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
///                 association: "A_laneSets_flowElementsContainer",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct FlowElementsContainer {
    pub flow_elements: Option<Vec<FlowElement>>,
    pub lane_sets: Option<Vec<LaneSet>>,
}

