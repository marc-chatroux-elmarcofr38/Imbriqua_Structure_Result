//! SubChoreography
#![allow(unused_imports)]

/// Conversion of SubChoreography (Class : SubChoreography)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SubChoreography",
///     name: "SubChoreography",
///     is_abstract: false,
///     super_class: Some(
///         "ChoreographyActivity FlowElementsContainer",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "SubChoreography-artifacts",
///                 name: "artifacts",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Artifact",
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
///                 association: "A_artifacts_subChoreography",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct SubChoreography {
    pub artifacts: Option<Vec<Artifact>>,
}

