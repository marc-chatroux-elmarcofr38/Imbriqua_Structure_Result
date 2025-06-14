//! InclusiveGateway
#![allow(unused_imports)]

/// Conversion of InclusiveGateway (Class : InclusiveGateway)
///
/// ```json
/// CMOFClass {
///     xmi_id: "InclusiveGateway",
///     name: "InclusiveGateway",
///     is_abstract: false,
///     super_class: Some(
///         "Gateway",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "InclusiveGateway-default",
///                 name: "default",
///                 visibility: Public,
///                 simple_type: Some(
///                     "SequenceFlow",
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
///                 association: "A_default_inclusiveGateway",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct InclusiveGateway {
    pub default: Option<SequenceFlow>,
}

