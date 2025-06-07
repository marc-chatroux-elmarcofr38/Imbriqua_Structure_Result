
/// Conversion of BPMNLabelStyle (Class : BPMNLabelStyle)
///
/// ```json
/// CMOFClass {
///     xmi_id: "BPMNLabelStyle",
///     name: "BPMNLabelStyle",
///     is_abstract: false,
///     super_class: None,
///     super_class_link: Some(
///         Class(
///             SuperClass {
///                 href: "DI.cmof#Style",
///             },
///         ),
///     ),
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "BPMNLabelStyle-font",
///                 name: "font",
///                 visibility: Public,
///                 simple_type: None,
///                 complex_type: Some(
///                     DataTypeLink(
///                         DataTypeLink {
///                             href: "DC.cmof#Font",
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

#[derive(Builder, Debug, Clone)]
pub struct BPMNLabelStyle<'a> {
    #[builder(setter(into))]
    pub font: i8,
}

