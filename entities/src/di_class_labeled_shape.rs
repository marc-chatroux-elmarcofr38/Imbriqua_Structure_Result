//! LabeledShape
#![allow(unused_imports)]

/// Conversion of LabeledShape (Class : LabeledShape)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LabeledShape",
///     name: "LabeledShape",
///     is_abstract: true,
///     super_class: Some(
///         "Shape",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "LabeledShape-ownedLabel",
///                 name: "ownedLabel",
///                 visibility: Public,
///                 simple_type: Some(
///                     "Label",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 0,
///                 upper: Infinity,
///                 default: None,
///                 is_read_only: true,
///                 is_composite: true,
///                 is_unique: false,
///                 is_ordered: false,
///                 is_abstract: None,
///                 is_derived: true,
///                 is_derived_union: true,
///                 subsetted_property: Some(
///                     "DiagramElement-ownedElement",
///                 ),
///                 owning_association: "",
///                 association: "A_ownedLabel_owningShape",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct LabeledShape {
    pub owned_label: Option<Vec<Label>>,
}

