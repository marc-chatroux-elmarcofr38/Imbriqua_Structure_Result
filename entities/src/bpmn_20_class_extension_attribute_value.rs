//! ExtensionAttributeValue
#![allow(unused_imports)]

/// Conversion of ExtensionAttributeValue (Class : ExtensionAttributeValue)
///
/// ```json
/// CMOFClass {
///     xmi_id: "ExtensionAttributeValue",
///     name: "ExtensionAttributeValue",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExtensionAttributeValue-valueRef",
///                 name: "valueRef",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ClassLink(
///                         ClassLink {
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
///                 ),
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
///                 association: "A_valueRef_extensionAttributeValue",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExtensionAttributeValue-value",
///                 name: "value",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     ClassLink(
///                         ClassLink {
///                             href: "http://schema.omg.org/spec/MOF/2.0/cmof.xml#Element",
///                         },
///                     ),
///                 ),
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
///                 association: "A_value_extensionAttributeValue",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "ExtensionAttributeValue-extensionAttributeDefinition",
///                 name: "extensionAttributeDefinition",
///                 visibility: Public,
///                 simple_type: Some(
///                     "ExtensionAttributeDefinition",
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
///                 association: "A_extensionAttributeDefinition_extensionAttributeValue",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct ExtensionAttributeValue {
    pub value_ref: Option<i8>,
    pub value: Option<i8>,
    pub extension_attribute_definition: ExtensionAttributeDefinition,
}

