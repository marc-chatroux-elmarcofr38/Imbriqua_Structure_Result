//! DataInput
#![allow(unused_imports)]

/// Conversion of DataInput (Class : DataInput)
///
/// ```json
/// CMOFClass {
///     xmi_id: "DataInput",
///     name: "DataInput",
///     is_abstract: false,
///     super_class: Some(
///         "ItemAwareElement",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataInput-name",
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
///                 association: "",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataInput-isCollection",
///                 name: "isCollection",
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
///                 xmi_id: "DataInput-inputSetRefs",
///                 name: "inputSetRefs",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputSet",
///                 ),
///                 complex_type: None,
///                 datatype: None,
///                 lower: 1,
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
///                 association: "A_dataInputRefs_inputSetRefs",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataInput-inputSetWithOptional",
///                 name: "inputSetWithOptional",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputSet",
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
///                 association: "A_optionalInputRefs_inputSetWithOptional",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///         Property(
///             CMOFProperty {
///                 xmi_id: "DataInput-inputSetWithWhileExecuting",
///                 name: "inputSetWithWhileExecuting",
///                 visibility: Public,
///                 simple_type: Some(
///                     "InputSet",
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
///                 association: "A_whileExecutingInputRefs_inputSetWithWhileExecuting",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```


#[derive(Debug, Clone)]
pub struct DataInput {
    pub name: Option<dc::String>,
    pub is_collection: dc::Boolean,
    pub input_set_refs: Vec<InputSet>,
    pub input_set_with_optional: Option<Vec<InputSet>>,
    pub input_set_with_while_executing: Option<Vec<InputSet>>,
}

