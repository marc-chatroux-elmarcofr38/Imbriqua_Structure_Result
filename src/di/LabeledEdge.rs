
/// Conversion of LabeledEdge (Class : LabeledEdge)
///
/// ```json
/// CMOFClass {
///     xmi_id: "LabeledEdge",
///     name: "LabeledEdge",
///     is_abstract: true,
///     super_class: Some(
///         "Edge",
///     ),
///     super_class_link: None,
///     owned_attribute: [
///         Property(
///             CMOFProperty {
///                 xmi_id: "LabeledEdge-ownedLabel",
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
///                 association: "A_ownedLabel_owningEdge",
///                 redefined_property_link: None,
///                 subsetted_property_link: None,
///             },
///         ),
///     ],
///     owned_rule: [],
/// }
/// ```

#[derive(Builder, Debug, Clone)]
pub struct LabeledEdge<'a> {
    #[builder(setter(into, strip_option), default)]
    pub owned_label: Option<Vec<&'a Label<'a>>>,
}

