//! SubProcess
#![allow(unused_imports)]

/// Conversion of SubProcess (Class : SubProcess)
///
/// ```json
/// CMOFClass {
///     xmi_id: "SubProcess",
///     name: "SubProcess",
///     is_abstract: false,
///     super_class: Some(
///         "Activity FlowElementsContainer",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "SubProcess-triggeredByEvent",
///                 name: "triggeredByEvent",
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
///                 xmi_id: "SubProcess-artifacts",
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
///                 association: "A_artifacts_subProcess",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct SubProcess {
    pub triggered_by_event: dc::Boolean,
    pub artifacts: Option<Vec<Artifact>>,
}

