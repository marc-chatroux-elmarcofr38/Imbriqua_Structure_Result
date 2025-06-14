//! Shape
#![allow(unused_imports)]

/// Conversion of Shape (Class : Shape)
///
/// ```json
/// CMOFClass {
///     xmi_id: "Shape",
///     name: "Shape",
///     is_abstract: true,
///     super_class: Some(
///         "Node",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "Shape-bounds",
///                 name: "bounds",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     DataTypeLink(
///                         DataTypeLink {
///                             href: "DC.cmof#Bounds",
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
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct Shape {
    pub bounds: i8,
}

