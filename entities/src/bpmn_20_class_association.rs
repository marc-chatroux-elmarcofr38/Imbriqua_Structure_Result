//! Association
#![allow(unused_imports)]

/// Conversion of Association (Class : Association)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Association",
///     name: "Association",
///     is_abstract: false,
///     super_class: Some(
///         "Artifact",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Association-associationDirection",
///                 name: "associationDirection",
///                 visibility: Public,
///                 simple_type: Some(
///                     "AssociationDirection",
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
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Association-sourceRef",
///                 name: "sourceRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BaseElement",
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
///                 association: "A_sourceRef_outgoing_association",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "Association-targetRef",
///                 name: "targetRef",
///                 visibility: Public,
///                 simple_type: Some(
///                     "BaseElement",
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
///                 association: "A_targetRef_incoming_association",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct Association {
    pub association_direction: AssociationDirection,
    pub source_ref: BaseElement,
    pub target_ref: BaseElement,
}

